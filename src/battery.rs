use core::fmt;
use embassy_nrf::{interrupt as i, peripherals as p};
use futures::pin_mut;

#[derive(Copy, Clone)]
pub struct Level {
    /// The output of the battery in mv.
    pub mv: u16,
    /// The percentage of battery left x10.
    pub percent_m10: u16,
}

// Display stuff

struct Voltage {
    mv: u16,
}

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:03}V", self.mv / 1000, self.mv % 1000)
    }
}

impl fmt::Debug for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

struct Percent {
    per_m10: u16,
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:01}", self.per_m10 / 10, self.per_m10 % 10)
    }
}

impl fmt::Debug for Percent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Level")
            .field("voltage", &self.display_voltage())
            .field("percent", &self.display_percent())
            .finish()
    }
}

impl Level {
    pub fn display_voltage(self) -> impl fmt::Display + fmt::Debug {
        Voltage { mv: self.mv }
    }

    pub fn display_percent(self) -> impl fmt::Display + fmt::Debug {
        Percent {
            per_m10: self.percent_m10,
        }
    }
}

// Get charge

pub async fn remaining(mut adc: p::SAADC, mut irq: i::SAADC, mut pin: p::P0_31) -> Level {
    use embassy_nrf::saadc::{
        Config, Gain, OneShot, Oversample, Reference, Resolution, Sample, Time,
    };
    let config = Config {
        resolution: Resolution::_12BIT,
        oversample: Oversample::BYPASS,
        reference: Reference::INTERNAL,
        gain: Gain::GAIN1_5,
        time: Time::_3US,
        ..Default::default()
    };
    let adc = OneShot::new(&mut adc, &mut irq, &mut pin, config);
    pin_mut!(adc);
    let sample = adc.as_mut().sample().await;
    // Result = vin * (gain / reference) * 2 ** (resolution)
    //        = vin * (0.2 / 0.6) * (2 ** 12)
    //        = vin * 1365
    // vin_V = result / 1365
    // vin_mv = (result * 3000) / 4095
    let mv = (sample as i32 * 3000) / 4095;
    // for sanity make sure we are positive.
    let mv = mv.max(0);
    // Source for percentage: https://forum.pine64.org/showthread.php?tid=8147
    // approximate with line:
    //  100% (3.9V) -> 0% (3.5V)
    //
    // give 1 decimal point of resolution:
    //  percent * 10 = (mv - 3500) / (3900 / 3500) * 100 * 10
    //               = (mv - 3500) / (3900 / 3.5)
    //               = (mv - 3500) / (7800 / 7)
    //
    let percent_m10 = (mv - 3500) / (7800 / 7);
    let percent_m10 = percent_m10.clamp(0, 1000);
    // The both percent and voltage cannot be over i16::MAX, so safe to truncate.
    Level {
        mv: mv as u16,
        percent_m10: percent_m10 as u16,
    }
}

/// Gets the power source
pub fn power_source(pin: &mut p::P0_12) -> Source {
    use embassy_nrf::{
        gpio::{Input, Pull},
        hal::prelude::*,
    };
    if Input::new(pin, Pull::None).is_high().unwrap() {
        Source::Battery
    } else {
        Source::Charger
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Source {
    Battery,
    Charger,
}
