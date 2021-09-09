use embassy_nrf::peripherals as p;
use embedded_hal::digital::v2::InputPin;
use futures::pin_mut;

/// A future that completes when the button is pressed.
pub async fn on_pressed(enable_pin: &mut p::P0_15, read_pin: &mut p::P0_13) {
    use embassy::{
        time::{Duration, Timer},
        traits::gpio::{WaitForHigh, WaitForLow},
    };
    use embassy_nrf::{
        gpio::{Input, Level, Output, OutputDrive, Pull},
        gpiote::PortInput,
    };

    // this pin will be dropped if the future is dropped, disconnecting the button to save power
    let _enable_pin = Output::new(enable_pin, Level::High, OutputDrive::Standard);
    let input = Input::new(&mut *read_pin, Pull::None);
    let port = PortInput::new(input);
    pin_mut!(port);
    loop {
        port.as_mut().wait_for_low().await;
        // Wait a time to make sure we don't get spurious stuff on button press/release
        Timer::after(Duration::from_millis(10)).await;
        if port.as_mut().is_low().unwrap() {
            // try again
            continue;
        }
        port.as_mut().wait_for_high().await;
        // Wait some time and check we are still high, to avoid spurious short-lived signals.
        Timer::after(Duration::from_millis(10)).await;
        if port.as_mut().is_high().unwrap() {
            // we detected a button press, return
            break;
        }
    }
}
