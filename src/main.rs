#![no_main]
#![no_std]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(min_type_alias_impl_trait)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embassy::{
    executor::{task, Executor},
    time::{Duration, Timer},
    util::Forever,
};
use embassy_nrf::{gpiote, hal::clocks, interrupt, pac, rtc, Peripherals};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use enigita::Rect;
use futures::{prelude::*, select_biased};
use pin_utils::pin_mut;
use rtt_target::{rprintln, rtt_init_print};

mod battery;
mod display;
mod power_button;

//use crate::display::DisplayOff;
use crate::display::{Backlight, Display};

#[panic_handler] // panicking behavior
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", panic_info);
    loop {
        cortex_m::asm::wfe();
    }
}

/*
/// A task that prints something when the button is pressed.
///
async fn button_pressed_poll(
    mut enable_pin: p::P0_15,
    mut read_pin: p::P0_13, /*, gpiote_tok: gpiote::Initialized*/
) {
    use gpio::{Input, Level, Output, OutputDrive, Pull};

    let mut pressed = false;
    let mut was_pressed = false;
    rprintln!("entering gpio poll loop");
    loop {
        use gpiote::PortInput;
        // Drive pin 15 high to enable the button.
        let read_pin = Input::new(&mut read_pin, Pull::Down);
        let enable_pin = Output::new(&mut enable_pin, Level::High, OutputDrive::Standard);
        // Calculated by trial and error: presses are picked up with 1 cycle delay, but not with 0
        // cycles delay.
        cortex_m::asm::delay(1);
        if read_pin.is_high().unwrap() {
            if pressed == false {
                pressed = true;
                rprintln!("button press event (todo do something in response)");
                was_pressed = true;
            }
        } else {
            if pressed == true {
                pressed = false;
            }
        }
        // power down the button
        drop(enable_pin);
        if was_pressed {
            was_pressed = false;
            // debounce (duration arbitrary)
            Timer::after(Duration::from_millis(200)).await;
        } else {
            // schedule another poll (duration arbitrary)
            Timer::after(Duration::from_millis(10)).await;
        }
    }
}
*/

static RTC: Forever<rtc::RTC<pac::RTC1>> = Forever::new();
static ALARM: Forever<rtc::Alarm<pac::RTC1>> = Forever::new();
static EXECUTOR: Forever<Executor> = Forever::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let alarm = unsafe {
        let p = pac::Peripherals::steal();
        clocks::Clocks::new(p.CLOCK)
            .enable_ext_hfosc()
            .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
            .start_lfclk();

        let rtc = RTC.put(rtc::RTC::new(p.RTC1, interrupt::take!(RTC1)));
        rtc.start();

        embassy::time::set_clock(rtc);
        ALARM.put(rtc.alarm0())
    };

    let executor = EXECUTOR.put(Executor::new());
    executor.set_alarm(alarm);
    executor.run(move |spawner| {
        spawner.spawn(run()).unwrap();
    });
}

#[task]
async fn run() {
    let Peripherals {
        gpiote,
        mut p0_02,
        mut p0_03,
        mut p0_12,
        mut p0_13,
        mut p0_14,
        mut p0_15,
        mut p0_18,
        mut p0_22,
        mut p0_23,
        mut p0_25,
        mut p0_26,
        p0_31,
        saadc,
        mut spim0,
        ..
    } = Peripherals::take().unwrap();
    let mut spim0_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);

    let gpiote_tok = gpiote::initialize(gpiote, interrupt::take!(GPIOTE));

    rprintln!("on {:?}", battery::power_source(&mut p0_12));
    let batt = battery::remaining(saadc, interrupt::take!(SAADC), p0_31).await;
    rprintln!("batt: {:?}", batt);

    loop {
        // wait for power on
        power_button::on_pressed(&mut p0_15, &mut p0_13, gpiote_tok).await;
        // show screen
        rprintln!("create display");
        let display = Display::new(
            &mut p0_14,
            &mut p0_22,
            &mut p0_23,
            &mut spim0,
            &mut spim0_irq,
            &mut p0_02,
            &mut p0_03,
            &mut p0_26,
            &mut p0_25,
            &mut p0_18,
        );
        pin_mut!(display);
        display.sleep_off().await;

        let top_left_area = Rect::new_unchecked(0, 0, 100, 100);
        let top_right_area = Rect::new_unchecked(140, 0, 240, 100);
        let bottom_left_area = Rect::new_unchecked(0, 140, 100, 240);
        let bottom_right_area = Rect::new_unchecked(140, 140, 240, 240);

        let black = Rgb565::BLACK.into_storage().to_be_bytes();
        let white = Rgb565::WHITE.into_storage().to_be_bytes();
        display
            .draw_rect_color(Rect::new_unchecked(0, 0, 240, 240), black)
            .await;
        rprintln!("top left");
        display.draw_rect_color(top_left_area, white).await;
        display.set_backlight(Backlight::High);
        rprintln!("backlight on");
        Timer::after(Duration::from_secs(3)).await;
        rprintln!("top right");
        display.draw_rect_color(top_left_area, black).await;
        display.draw_rect_color(top_right_area, white).await;
        Timer::after(Duration::from_secs(3)).await;
        rprintln!("bottom left");
        display.draw_rect_color(top_right_area, black).await;
        display.draw_rect_color(bottom_left_area, white).await;
        Timer::after(Duration::from_secs(3)).await;
        rprintln!("bottom right");
        display.draw_rect_color(bottom_left_area, black).await;
        display.draw_rect_color(bottom_right_area, white).await;
        // wait for power off with timeout
        select_biased! {
            _ = power_button::on_pressed(&mut p0_15, &mut p0_13, gpiote_tok).fuse() => {},
            _ = Timer::after(Duration::from_secs(5)).fuse() => {},
        }
        // hide screen
        drop(display);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("{:#?}", ef);
    loop {}
}
