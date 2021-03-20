#![no_main]
#![no_std]
#![allow(incomplete_features)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(min_type_alias_impl_trait)]

use core::pin::Pin as StdPin;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use embassy::{
    executor::{task, Executor},
    time::{Duration, Timer},
    util::Forever,
};
use embassy_nrf::{
    hal::{
        clocks,
        gpio::{
            self, p0::Parts, Disconnected, Floating, Input, Level, Output, Pin, PullDown, PushPull,
        },
        prelude::*,
        spim::{self, Spim},
        Delay,
    },
    interrupt, pac, rtc,
};
use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb565, Rgb888},
    prelude::*,
};
use rtt_target::{rprintln, rtt_init_print};

mod display;

use crate::display::DisplayOff;

#[panic_handler] // panicking behavior
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", panic_info);
    cortex_m::asm::udf();
}

#[task]
async fn ms100() {
    loop {
        rprintln!("1s tick");
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[task]
async fn periodic_display(mut d: DisplayOff<pac::SPIM0>) {
    loop {
        Timer::after(Duration::from_secs(3)).await;
        rprintln!("screen power on");
        let mut don = d.power_on().await;
        rprintln!("finish: screen power on");
        for i in 0..100u8 {
            for j in 0..100u8 {
                unsafe {
                    let mut don = StdPin::new_unchecked(&mut don);
                    let color: Rgb565 = Rgb888::new(i, j, 0).into();
                    rprintln!("writing pixel {}x{}", i, j);
                    don.set_pixel(i.into(), j.into(), RawU16::from(color).into_inner())
                        .await;
                }
            }
        }
        Timer::after(Duration::from_secs(3)).await;
        rprintln!("screen power off");
        d = don.power_off().await;
        rprintln!("finish: screen power off");
    }
}

static RTC: Forever<rtc::RTC<pac::RTC1>> = Forever::new();
static ALARM: Forever<rtc::Alarm<pac::RTC1>> = Forever::new();
static EXECUTOR: Forever<Executor> = Forever::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("main");

    let p = pac::Peripherals::take().unwrap();

    clocks::Clocks::new(p.CLOCK)
        .enable_ext_hfosc()
        .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
        .start_lfclk();

    let rtc = RTC.put(rtc::RTC::new(p.RTC1, interrupt::take!(RTC1)));
    rtc.start();
    unsafe { embassy::time::set_clock(rtc) };

    let p0 = Parts::new(p.P0);
    let display = DisplayOff::new(
        p0.p0_22.degrade(),
        p0.p0_26.degrade(),
        p0.p0_25.degrade(),
        p0.p0_18.degrade(),
        p0.p0_02.degrade(),
        p0.p0_03.degrade(),
        p.SPIM0,
        interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0),
    );

    let alarm = ALARM.put(rtc.alarm0());
    let executor = EXECUTOR.put(Executor::new());
    executor.set_alarm(alarm);
    executor.run(move |spawner| {
        spawner.spawn(ms100()).unwrap();
        spawner.spawn(periodic_display(display)).unwrap();
    });
}

/*
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
*/

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("{:#?}", ef);
    loop {}
}
