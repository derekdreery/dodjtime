#![no_main]
#![no_std]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![feature(type_alias_impl_trait)]
#![feature(const_generics_defaults)]

extern crate panic_abort;

#[cfg(not(target_pointer_width = "32"))]
compile_error!("code assumes pointer width is 32 bits");

use cortex_m_rt::{entry, exception, ExceptionFrame};
use defmt::{unwrap, Format};
use defmt_rtt as _;
use embassy::{
    blocking_mutex::kind::CriticalSection,
    channel::mpsc,
    executor::{Executor, Spawner},
    interrupt::InterruptExt,
    task,
    time::{Duration, Timer},
    util::Forever,
};
use embassy_nrf::{
    gpiote,
    gpiote::{InputChannel, InputChannelPolarity},
    interrupt::{self, Priority},
    peripherals::{self, P0_12, P0_13, P0_15, P0_31, SAADC},
    Peripherals,
};
use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
};
use futures::prelude::*;
use heapless::String;
use nrf_softdevice::{ble, Softdevice};
use pin_utils::pin_mut;

mod battery;
//mod ble;
mod display;
mod power_button;

//use crate::display::DisplayOff;
use crate::{
    battery::Battery,
    display::{Backlight, DisplayFlashSpi},
};

const CHANNEL_SIZE: usize = 3;
const EASY_DMA_SIZE: usize = 255;
const BG_IMAGE: &[u8] = include_bytes!("../data/pictures/clock_bg.rgb565");

static EXECUTOR: Forever<Executor> = Forever::new();
static BATTERY_CHANNEL: Forever<battery::Channel> = Forever::new();
static DISPLAY_FLASH_CHANNEL: Forever<display::Channel> = Forever::new();
static MAIN_CHANNEL: Forever<Channel<Cmd>> = Forever::new();

type Channel<C, M = CriticalSection, const S: usize = CHANNEL_SIZE> = mpsc::Channel<M, C, S>;
type Sender<'ch, C, M = CriticalSection, const S: usize = CHANNEL_SIZE> =
    mpsc::Sender<'ch, M, C, S>;
type Receiver<'ch, C, M = CriticalSection, const S: usize = CHANNEL_SIZE> =
    mpsc::Receiver<'ch, M, C, S>;

/// Commands that can be sent to the main task
#[derive(Format)]
enum Cmd {
    /// The power button was pressed
    PowerButtonPressed,
    /// The battery task responded to a request with the current battery state
    BatteryState(battery::State),
}

#[entry]
fn main() -> ! {
    // Setup embassy to use priority 2, so we don't clash with softdevice
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    // Setup bluetooth
    let config: nrf_softdevice::Config = Default::default();
    //let softdev = Softdevice::enable(&config);

    let executor = EXECUTOR.put(Executor::new());
    executor.run(|spawner| {
        // Setup display & flash task
        let mut display_flash_channel = DISPLAY_FLASH_CHANNEL.put(display::Channel::new());
        let (display_sender, display_receiver) = mpsc::split(display_flash_channel);
        let spim0_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
        spim0_irq.set_priority(Priority::P2);
        let display_flash_spi = DisplayFlashSpi::new(
            p.P0_14, p.P0_22, p.P0_23, p.TWISPI0, spim0_irq, p.P0_02, p.P0_03, p.P0_04, p.P0_26,
            p.P0_25, p.P0_05, p.P0_18,
        );

        // Setup battery task
        let mut battery_channel = BATTERY_CHANNEL.put(battery::Channel::new());
        let (battery_sender, battery_receiver) = mpsc::split(battery_channel);
        let saadc_irq = interrupt::take!(SAADC);
        saadc_irq.set_priority(Priority::P2);

        // Setup the channel to send messages to the main event loop
        let mut main_channel = MAIN_CHANNEL.put(Channel::new());
        let (main_sender, main_receiver) = mpsc::split(main_channel);

        // Tick tock
        //unwrap!(spawner.spawn(alive()));
        // Start the SoftDevice
        //unwrap!(spawner.spawn(softdevice_task(softdev)));
        // Start power button task
        unwrap!(spawner.spawn(power_button_task(p.P0_15, p.P0_13, main_sender.clone())));
        // Start display task
        unwrap!(spawner.spawn(display_flash_task(display_flash_spi, display_receiver)));
        // Spawn battery task
        unwrap!(spawner.spawn(battery_task(
            p.SAADC,
            saadc_irq,
            p.P0_31,
            p.P0_12,
            battery_receiver,
            main_sender,
        )));
        unwrap!(spawner.spawn(main_task(
            main_receiver,
            display_sender,
            battery_sender,
            /*softdev*/
        )));
    });
}

/*
#[task]
async fn alive() {
    let mut cnt = 0;
    loop {
        Timer::after(Duration::from_secs(1)).await;
        defmt::debug!("tick {=usize}", cnt);
        cnt += 1;
    }
}
*/

#[task]
async fn softdevice_task(sd: &'static Softdevice) {
    sd.run().await;
}

// Note: the display controller also handles flash. This is because they share the same SPI
// peripheral, and some operations need to coordinate between them (e.g. copy image from flash to
// display).
#[task]
async fn display_flash_task(
    mut display: DisplayFlashSpi,
    mut channel: Receiver<'static, display::Cmd>,
) {
    while let Some(cmd) = channel.recv().await {
        display.handle(cmd).await;
    }
}

#[task]
async fn power_button_task(mut p0_15: P0_15, mut p0_13: P0_13, channel: Sender<'static, Cmd>) {
    use embassy::{
        time::{Duration, Timer},
        traits::gpio::{WaitForHigh, WaitForLow},
    };
    use embassy_nrf::{
        gpio::{Input, Level, Output, OutputDrive, Pull},
        gpiote::PortInput,
    };
    use embedded_hal::digital::v2::InputPin;
    let _enable_pin = Output::new(&mut p0_15, Level::High, OutputDrive::Standard);
    let input = Input::new(&mut p0_13, Pull::None);
    let mut port = PortInput::new(input);
    loop {
        port.wait_for_low().await;
        // Wait a time to make sure we don't get spurious stuff on button press/release
        Timer::after(Duration::from_millis(10)).await;
        if unwrap!(port.is_low()) {
            // try again
            continue;
        }
        port.wait_for_high().await;
        // Wait some time and check we are still high, to avoid spurious short-lived signals.
        Timer::after(Duration::from_millis(10)).await;
        if unwrap!(port.is_high()) {
            // we detected a button press, return
            unwrap!(channel.send(Cmd::PowerButtonPressed).await);
        }
    }
}

#[task]
async fn battery_task(
    adc: SAADC,
    irq: interrupt::SAADC,
    level_pin: P0_31,
    source_pin: P0_12,
    mut cmd_in: Receiver<'static, battery::Cmd>,
    cmd_out: Sender<'static, Cmd>,
) {
    let mut battery = Battery::new(adc, irq, level_pin, source_pin);
    // Only 1 command for now
    while let Some(_) = cmd_in.recv().await {
        unwrap!(
            cmd_out
                .send(Cmd::BatteryState(battery.current_state().await))
                .await,
            "unreachable"
        );
    }
}

// The main task should be a big event loop where different IO are handled. Use channels to send
// commands to other tasks, and have a mpsc channel to collect things to handle from other tasks.
#[task]
async fn main_task(
    mut main_channel: Receiver<'static, Cmd>,
    display_channel: display::Sender<'static>,
    battery_channel: battery::Sender<'static>,
    /*softdev: &'static Softdevice,*/
) {
    use ble::peripheral::{
        advertise, Config as AdvertiseConfig, NonconnectableAdvertisement::ScannableUndirected,
    };

    // Update reported battery level.
    unwrap!(battery_channel.send(battery::Cmd::SampleBattery).await);
    defmt::info!("{}", unwrap!(main_channel.recv().await));

    // First attempt at ble, advertise a connection.
    /*
    let addr = ble::get_address(softdev);
    defmt::info!(
        "BLE address: {=[u8]:x} (type {})",
        addr.bytes(),
        addr.address_type()
    );
    */

    //ble::advertise(softdevice);

    /* powerup indication */
    //unwrap!(display_channel.send(display::Cmd::PowerOn).await);

    let mut cnt = 0;
    loop {
        match unwrap!(main_channel.recv().await) {
            Cmd::PowerButtonPressed => {
                defmt::info!("button pressed {}", cnt);
                cnt += 1;
                unwrap!(battery_channel.send(battery::Cmd::SampleBattery).await);
                defmt::info!("show some stuff");
                //debug!("sleep off");
                unwrap!(display_channel.send(display::Cmd::SleepOff).await);

                defmt::debug!("draw black");
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            Rectangle::new(Point::new(0, 0), Size::new(240, 240)),
                            Rgb565::BLACK,
                        ))
                        .await
                );
                defmt::debug!("draw image");
                unwrap!(
                    display_channel
                        .send(display::Cmd::draw_image(Point::new(2, 2), BG_IMAGE, 4,))
                        .await
                );
                unwrap!(
                    display_channel
                        .send(display::Cmd::draw_text(
                            Point::new(0, 0),
                            {
                                let mut s = String::new();
                                unwrap!(s.push_str("12:32"));
                                s
                            },
                            5,
                        ))
                        .await
                );
                defmt::debug!("backlight up");
                unwrap!(
                    display_channel
                        .send(display::Cmd::SetBacklight {
                            level: Backlight::High,
                        })
                        .await
                );
                // This waits 5 secs after the message was sent - not necessarily the same as when
                // the operation was completed (TODO).
                // TODO don't sleep in the main thread, this is just for an example for now.
                Timer::after(Duration::from_secs(5)).await;
                unwrap!(
                    display_channel
                        .send(display::Cmd::SetBacklight {
                            level: Backlight::Off,
                        })
                        .await
                );

                /*
                defmt::debug!("draw black");
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            Rectangle::new(Point::new(0, 0), Size::new(240, 240)),
                            Rgb565::BLACK,
                        ))
                        .await
                );
                defmt::debug!("draw bg");
                let top_left_area = Rectangle::new(Point::new(0, 0), Size::new(100, 100));
                let top_right_area = Rectangle::new(Point::new(140, 0), Size::new(240, 100));
                let bottom_left_area = Rectangle::new(Point::new(0, 140), Size::new(100, 240));
                let bottom_right_area = Rectangle::new(Point::new(140, 140), Size::new(240, 240));

                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            top_left_area,
                            Rgb565::WHITE,
                        ))
                        .await
                );
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            top_left_area,
                            Rgb565::BLACK,
                        ))
                        .await
                );
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            top_right_area,
                            Rgb565::WHITE,
                        ))
                        .await
                );
                Timer::after(Duration::from_secs(3)).await;
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            top_right_area,
                            Rgb565::BLACK,
                        ))
                        .await
                );
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            bottom_left_area,
                            Rgb565::WHITE,
                        ))
                        .await
                );
                Timer::after(Duration::from_secs(3)).await;
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            bottom_left_area,
                            Rgb565::BLACK,
                        ))
                        .await
                );
                unwrap!(
                    display_channel
                        .send(display::Cmd::fill_rect_with_color(
                            bottom_right_area,
                            Rgb565::WHITE,
                        ))
                        .await
                );
                Timer::after(Duration::from_secs(3)).await;
                unwrap!(display_channel.send(display::Cmd::SleepOn).await);
                unwrap!(
                    display_channel
                        .send(display::Cmd::SetBacklight {
                            level: Backlight::Off,
                        })
                        .await
                );
                */
            }
            Cmd::BatteryState(state) => defmt::info!("battery: {:?}", state),
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    defmt::error!("HardFault");
    loop {}
}
