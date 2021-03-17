#![no_main]
#![no_std]

use async_embedded::task;
use core::time::Duration;
use cortex_m_rt::{entry, exception, interrupt, ExceptionFrame};
use embedded_hal::{blocking::delay::DelayMs, prelude::*};
use nrf52832_hal::{
    gpio::{
        self, p0::Parts, Disconnected, Floating, Input, Level, Output, Pin, PullDown, PushPull,
    },
    pac,
    prelude::*,
    spim::{self, Spim},
    Delay,
};
use rtt_target::{rprintln, rtt_init_print};

mod display;
mod time;
mod timer;

use display::Display;

#[panic_handler] // panicking behavior
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", panic_info);
    cortex_m::asm::udf();
}

type PushPullPin = gpio::Pin<gpio::Output<gpio::PushPull>>;

unsafe fn pre_init() {
    init_clocks();
    timer::init();
}

unsafe fn init_clocks() {
    // We are exclusive here so can borrow registers safely at most 1 time.

    // configure the LFCLK to use the external crystal (32.768Hz)
    let clock = &*pac::CLOCK::ptr();
    clock.lfclksrc.write(|w| w.src().xtal());
    clock.tasks_lfclkstart.write(|w| w.bits(1));

    // spin until the low freq clock has started up. According to the datasheet this is ~250 μs.
    // The time could be spend doing something else.
    while clock.events_lfclkstarted.read().bits() == 0 {
        continue;
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("main");
    let p = pac::Peripherals::take().unwrap();
    unsafe { pre_init() }
    let p0 = Parts::new(p.P0);
    // *Don't use RTC0!*

    task::spawn(async {
        loop {
            rprintln!("100 ms loop");
            cortex_m::asm::bkpt();
            timer::wait(Duration::from_millis(100)).await;
        }
    });
    //let batt = Battery::new(p0.p0_12);
    task::block_on(async {
        loop {
            rprintln!("1_000 ms loop");
            //rprintln!("on batt: {}", batt.using_battery());
            timer::wait(Duration::from_millis(1_000)).await;
        }
    });
    rprintln!("unreachable");
    // unreachable, but the compiler can't tell.
    loop {
        cortex_m::asm::nop();
    }
}

struct Battery {
    pin: gpio::p0::P0_12<Input<PullDown>>,
}

impl Battery {
    fn new<M>(pin: gpio::p0::P0_12<M>) -> Self {
        Self {
            pin: pin.into_pulldown_input(),
        }
    }

    fn into_pin(self) -> gpio::p0::P0_12<Disconnected> {
        self.pin.into_disconnected()
    }

    /// True if running on batt, false if charging.
    fn using_battery(&self) -> bool {
        match self.pin.is_high() {
            Ok(v) => v,
            Err(e) => match e {},
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("{:#?}", ef);
    loop {}
}
