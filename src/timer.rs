// TODO think about when you set the cc[0], sometimes the event will not fire if the counter is
// very close to the event. Handle these cases. This is a big one because currently sometimes
// timeouts will not fire until the next timeout completes. One way around this might be to only
// set timeouts at a lower resolution than the RTC0: maybe 1:4. That way there will be at least 4
// ticks between timeouts, which I think is enough according to the datasheet (this needs checking
// though).

use core::{
    cmp,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::atomic::{self, AtomicBool, Ordering},
    task::{Context, Poll, Waker},
};
use cortex_m::peripheral::NVIC;
use heapless::{
    binary_heap::{BinaryHeap, Min},
    consts::{U3, U8},
    i, Vec,
};
use nrf52832_hal::pac::{interrupt, Interrupt, RTC0};
use rtt_target::rprintln;

use crate::time::Time;

pub unsafe fn init() {
    // Unsafe: we are in exclusive mode.
    let rtc0 = &*RTC0::ptr();

    // enable compare[0] and ovrflw interrupts
    rtc0.intenset
        .write(|w| w.compare0().set_bit().ovrflw().set_bit());

    // start the peripheral.
    rtc0.tasks_clear.write(|w| w.bits(1));
    rtc0.tasks_start.write(|w| w.bits(1));
}

static mut CURRENT_TIME: Time = Time::from_bits(0);

/// Handle to the singleton timer.
pub static mut TIMER: Timer = Timer {
    timeout_queue: BinaryHeap(i::BinaryHeap::new()),
};

/// Provides an interface for setting and waiting on timeouts.
pub struct Timer {
    /// A priority queue of the currently set timeouts.
    ///
    /// Since timeouts with the same time are considered equal, only one can live in the heap at a
    /// time.
    // Random note: wakers are 2 pointers wide.
    timeout_queue: BinaryHeap<Timeout, U8, Min>,
}

impl Timer {
    /// Add a waker to the queue.
    fn add_waker(&mut self, time: Time, waker: Waker) {
        if let Some(timeout) = self.timeout_queue.iter_mut().find(|t| t.time == time) {
            timeout.wakers.push(waker).unwrap();
        } else {
            // Insert a new timeout and add the waker.
            self.timeout_queue
                .push(Timeout {
                    time,
                    wakers: {
                        let mut v = Vec::new();
                        v.push(waker).unwrap();
                        v
                    },
                })
                .unwrap();
        }
    }

    /// Called from the interrupt handler to respond to the compare[0] event firing.
    ///
    /// Make sure that `CURRENT_TIME` is up to date before running this function.
    ///
    /// # Safety
    ///
    /// The caller must ensure nothing will race with this function to access CURRENT_TIME or TIMER
    /// (Self).
    unsafe fn handle_cc0(&mut self) {
        // Steps:
        //  - Update the tick counter.
        //  - Check for timeouts that have passed. For each one:
        //    - wake the wakers,
        //    - remove the timeouts from the queue.
        //  - If the next timeout is before overflow
        //    - set the cc[0] to the ticks
        //  - else
        //    - disable the compare[0] event.
        // before the overflow, or disable it otherwise.
        while let Some(timeout) = self.timeout_queue.peek() {
            rprintln!(
                "checking timeout {:?} to see if it's in the past",
                timeout.time
            );
            rprintln!("current time: {}", CURRENT_TIME);
            if timeout.time > CURRENT_TIME {
                // We've got through all the timers before now.
                rprintln!("in the future so carrying on");
                break;
            }
            // This will never fail (because we checked above with `peek`).
            let timeout = self.timeout_queue.pop().unwrap();
            for waker in timeout.wakers {
                waker.wake();
            }
            rprintln!("removing timeout for {}", timeout.time);
            // drop timeout
        }

        self.set_cc0();
    }

    /// Set (or unset) the compare[0] event and cc[0] registers to trigger an interrupt when the
    /// next timeout completes, or nothing if an overflow will happen first/there are no timers
    /// left.
    ///
    ///  - *Current time must be updated before calling this function.*
    ///  - *Completed timeouts should be removed before calling this function.
    ///
    /// # Safety
    ///
    /// This function must not be preempted by the RTC0 interrupt, which would data race the static
    /// vars. Nothing else must access the rtc0 registers during this function call.
    unsafe fn set_cc0(&self) {
        let rtc = &*RTC0::ptr();
        match self.timeout_queue.peek() {
            Some(timeout) if timeout.time.overflows() == CURRENT_TIME.overflows() => {
                // timer coming up: set it

                rprintln!("next timeout: {}", timeout.time);
                // Make sure the compare0 event is enabled
                rtc.evten.write(|w| w.compare0().enabled());
                // Set the new timeout tick value. We already know that it must be in the future.
                rprintln!("setting timeout to {:x}", timeout.time.ticks());
                rtc.cc[0].write(|w| w.bits(timeout.time.ticks()));
            }
            _ => {
                // no more timers or timer far in future: unset compare[0] event
                rprintln!("no timeouts until next overflow");
                rtc.evten.write(|w| w.compare0().disabled());
            }
        }
    }
}

/// A timeout.
#[derive(Debug)]
struct Timeout {
    /// number of RTC ticks until the timeout expires.
    time: Time,
    /// Used to wake the calling task.
    ///
    /// Currently hard-coded to max 3 wakers per timeout. Might be better to allocate these vecs in
    /// some sort of arena so we don't need them all to be the same length.
    wakers: Vec<Waker, U3>,
}

impl cmp::Ord for Timeout {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl cmp::PartialOrd for Timeout {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Timeout {
    fn eq(&self, other: &Self) -> bool {
        self.time.eq(&other.time)
    }
}

impl Eq for Timeout {}

/// Waits for at least `dur`
///
/// The resolution of the timer is about 30 microseconds. The duration will be rounded down to
/// this resolution.
pub async fn wait(dur: core::time::Duration) {
    rprintln!("in timer::wait");
    struct Wait {
        end: Time,
        installed_waker: bool,
    }

    impl Future for Wait {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            // we have to turn off rtc interrupts to avoid data races on CURRENT_TIME
            // Safety: we are exclusive with RTC0 interrupt when we access the static var.
            let current_time = rtc0_cs(|| unsafe {
                update_current_time_from_counter();
                CURRENT_TIME
            });

            rprintln!(
                "polling wait future, current time: {0:} ({0:?})",
                current_time
            );

            if self.end <= current_time {
                Poll::Ready(())
            } else {
                if !self.installed_waker {
                    rprintln!("installing waker");
                    rtc0_cs(|| unsafe {
                        // Get or insert a slot in the queue, thenm push this waker to it.
                        // Unsafe: we have exclusive access in a critical section, so safe to
                        // create a mut reference.
                        let old_next = TIMER.timeout_queue.peek().map(|t| t.time);
                        TIMER.add_waker(self.end, cx.waker().clone());
                        if Some(TIMER.timeout_queue.peek().unwrap().time) != old_next {
                            // need to update the first timeout
                            TIMER.set_cc0();
                        }
                        self.installed_waker = true;
                    });
                } else {
                    // spurious wakeup or poll called spuriously, do nothing
                    rprintln!("spurious wakeup");
                }

                Poll::Pending
            }
        }
    }

    // Make sure our current time object is up-to-date, then calculate the timeout end and
    // create the object.
    //
    // No timer will actually be set until it is first poll'd. If by that point the timeout is
    // already in the past, we can skip setting the timer altogether and mark the future
    // complete.
    let current_time = rtc0_cs(|| unsafe {
        update_current_time_from_counter();
        CURRENT_TIME
    });
    let dur = Time::from(dur);
    // Unsafe: CURRENT_TIME is only modified in a higher priority interrupt, so won't be
    // touched while we read it.
    let end = current_time + dur;
    rprintln!(
        "setting timeout to current_time ({}) + dur ({}) = {}",
        current_time,
        dur,
        end
    );

    Wait {
        end,
        installed_waker: false,
    }
    .await
}

/// Run a function with interrupts for rtc0 off.
fn rtc0_cs<T>(f: impl FnOnce() -> T) -> T {
    // Safety: I think altering the interrupt mask is fully atomic and can't be interrupted. Should
    // check.
    unsafe {
        NVIC::mask(Interrupt::RTC0);
        let out = f();
        NVIC::unmask(Interrupt::RTC0);
        out
    }
}

/// Update the current time in memory, using the counter from RTC0.
///
/// Overflows are handled in the interrupt handler and so this does not touch the top 40 bits.
///
/// # Safety
///
/// It is up to the caller to make sure no data races will occur with `CURRENT_TIME`.
#[inline]
unsafe fn update_current_time_from_counter() {
    let rtc = &*RTC0::ptr();
    let counter = rtc.counter.read().bits();
    // Update the time. counter will always be 0 in the top 8 bytes.
    CURRENT_TIME.set_ticks(counter);
}

/// RTC0 interrupt handler.
#[interrupt]
unsafe fn RTC0() {
    rprintln!("RTC0 interrupt");
    handle_rtc_event();
}

/// Check if any rtc events have occurred and process them if they have.
///
/// # Safety
///
/// No function should have access to `CURRENT_TIME`, `TIMER`, or the RTC0 peripheral while this
/// function is running.
unsafe fn handle_rtc_event() {
    rprintln!("RTC0 interrupt!");
    let rtc = &*RTC0::ptr();

    // We enable the compare[0] and ovrflw events only.
    if rtc.events_ovrflw.read().bits() != 0 {
        rtc.events_ovrflw.reset();
        CURRENT_TIME.increment_overflows();
        rprintln!("ovrflw: counter = {}", rtc.counter.read().bits());
        // TODO check if the next timeout is less than 1 overflow, and schedule it if it is.
    }

    if rtc.events_compare[0].read().bits() != 0 {
        rtc.events_compare[0].reset();
        rprintln!("compare[0]: counter = {}", rtc.counter.read().bits());
        // Safety: nothing that can preempt this interrupt will access the static var, so we can
        // create an exclusive reference, and satisfy the requirements of the unsafe fn.
        update_current_time_from_counter();
        unsafe { TIMER.handle_cc0() }
    }
}
