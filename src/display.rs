use core::mem;
use nrf52832_hal::{
    gpio::{self, p0::Parts, Disconnected, Level, Output, Pin, PushPull},
    pac,
    spim::{self, Spim},
    Delay,
};

use crate::PushPullPin;

/// Object that logically owns and manages a ST7789 display.
pub enum Display {
    PowerOn {
        /// backlight
        bl_pin: PushPullPin,
        /// TODO work out what this is.
        cs_pin: PushPullPin,
        //display: ST7789<SPIInterfaceNoCS<Spim<pac::SPIM0>, PushPullPin>, PushPullPin>,
    },
    PowerOff {
        /// backlight
        bl_pin: Pin<Disconnected>,
        /// reset
        rst_pin: Pin<Disconnected>,
        /// Chip select pin: hold low when using the display adaptor.
        cs_pin: Pin<Disconnected>,
        /// data/clock switch
        dc_pin: Pin<Disconnected>,
        /// SPI clock to LCD
        spi_clk_pin: Pin<Disconnected>,
        /// SPI master-out-slave-in to LCD
        spi_mosi_pin: Pin<Disconnected>,
    },
    /// Used so we can take all the contents of `Display`, do stuff with them, then put them back,
    /// all the while having a valid value.
    Dummy,
}

impl Display {
    /// Create the display from its constituent parts.
    ///
    /// Does not do any I/O.
    pub fn new(
        bl_pin: Pin<Disconnected>,
        rst_pin: Pin<Disconnected>,
        cs_pin: Pin<Disconnected>,
        dc_pin: Pin<Disconnected>,
        spi_clk_pin: Pin<Disconnected>,
        spi_mosi_pin: Pin<Disconnected>,
    ) -> Self {
        Self::PowerOff {
            bl_pin,
            rst_pin,
            cs_pin,
            dc_pin,
            spi_clk_pin,
            spi_mosi_pin,
        }
    }

    /// Assert that the display is powered on
    pub async fn power_on(&mut self) {
        if matches!(self, Self::PowerOff { .. }) {
            // Take the contents of `Display` out to manipulate them.
            let this = mem::replace(self, Display::Dummy);

            if let Self::PowerOff {
                bl_pin,
                rst_pin,
                cs_pin,
                dc_pin,
                spi_clk_pin,
                spi_mosi_pin,
            } = this
            {
                // Turn on pins
                let bl_pin = bl_pin.into_push_pull_output(Level::Low);
                let rst_pin = rst_pin.into_push_pull_output(Level::Low);
                let cs_pin = cs_pin.into_push_pull_output(Level::Low);
                let dc_pin = dc_pin.into_push_pull_output(Level::Low);
                let spi_clk_pin = spi_clk_pin.into_push_pull_output(Level::Low);
                let spi_mosi_pin = spi_mosi_pin.into_push_pull_output(Level::Low);
                todo!()
            } else {
                // Safety: we just checked the variant.
                unsafe { core::hint::unreachable_unchecked() }
            }
        }
    }

    fn power_off(&mut self) {
        todo!()
    }
}
