use core::fmt;
use defmt::Format;
use embassy::util::mpsc;
use embassy_nrf::{interrupt as i, peripherals as p};
use embedded_hal::digital::v2::InputPin;
use futures::pin_mut;

/// Commands for the battery task
#[derive(Format)]
pub enum Cmd {
    /// Request that the battery is sampled and the result is sent to the main task.
    SampleBattery,
}

pub type Channel = mpsc::Channel<mpsc::WithNoThreads, Cmd, { crate::CHANNEL_SIZE }>;
pub type Sender<'ch> = mpsc::Sender<'ch, mpsc::WithNoThreads, Cmd, { crate::CHANNEL_SIZE }>;

#[derive(Format)]
pub struct State {
    level: Level,
    source: Source,
}

pub struct Battery {
    adc: p::SAADC,
    irq: i::SAADC,
    level_pin: p::P0_31,
    source_pin: p::P0_12,
}

impl Battery {
    pub fn new(adc: p::SAADC, irq: i::SAADC, level_pin: p::P0_31, source_pin: p::P0_12) -> Self {
        Battery {
            adc,
            irq,
            level_pin,
            source_pin,
        }
    }

    pub async fn current_state(&mut self) -> State {
        State {
            level: self.level().await,
            source: self.source(),
        }
    }

    /// Get remaining charge
    pub async fn level(&mut self) -> Level {
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
        let adc = OneShot::new(&mut self.adc, &mut self.irq, config);
        pin_mut!(adc);
        let sample = adc.as_mut().sample(&mut self.level_pin).await;
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
        // Both percent and voltage cannot be over i16::MAX, so safe to truncate.
        Level {
            mv: mv as u16,
            percent_m10: percent_m10 as u16,
        }
    }

    /// Gets the power source
    pub fn source(&mut self) -> Source {
        use embassy_nrf::gpio::{Input, Pull};
        if Input::new(&mut self.source_pin, Pull::None)
            .is_high()
            .unwrap()
        {
            Source::Battery
        } else {
            Source::Charger
        }
    }
}

#[derive(Copy, Clone)]
pub struct Level {
    /// The output of the battery in mv.
    pub mv: u16,
    /// The percentage of battery left x10.
    pub percent_m10: u16,
}

// Display stuff

impl Format for Level {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Level {{ voltage: {}.{:03}V, percent: {}.{:01}% }}",
            self.mv / 1000,
            self.mv % 1000,
            self.percent_m10 / 10,
            self.percent_m10 % 10
        )
    }
}

#[derive(Format, Copy, Clone)]
pub enum Source {
    Battery,
    Charger,
}
