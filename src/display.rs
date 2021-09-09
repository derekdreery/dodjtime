use core::iter;
use defmt::{assert, debug, panic, unwrap, Format};
use embassy::{
    time::{Duration, Timer},
    traits::spi::{FullDuplex, Write},
    util::mpsc,
};
use embassy_nrf::{
    gpio::{FlexPin, Level, NoPin, Output, OutputDrive},
    interrupt,
    peripherals::{P0_02, P0_03, P0_04, P0_05, P0_14, P0_18, P0_22, P0_23, P0_25, P0_26, TWISPI0},
    spim::{self, Spim},
};
use embedded_hal::digital::v2::OutputPin;

pub use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::{IntoStorage, Rgb565, RgbColor},
    primitives::Rectangle,
};

const DISPLAY_WIDTH: usize = 240;
const DISPLAY_HEIGHT: usize = 240;
const DISPLAY_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

/// These are the commands that can be sent to the SPI (the display and the nor flash memory)
#[derive(Format)]
pub enum Cmd {
    /// Wake the display
    SleepOff,
    /// Power down the display
    SleepOn,
    /// Fill a rectangular area with the given color
    FillRectWithColor {
        /// The area to fill
        area: Rectangle,
        /// The color to fill it with
        color: Rgb565,
    },
    DrawImage {
        top_left: Point,
        image: &'static [u8],
        scale: u8,
    },
    /// Change the backlight level
    SetBacklight { level: Backlight },
    /// A high-level command to display a power on indicator.
    PowerOn,
}

impl Cmd {
    #[inline]
    pub fn fill_rect_with_color(area: Rectangle, color: Rgb565) -> Self {
        Self::FillRectWithColor { area, color }
    }
}

pub type Channel = mpsc::Channel<mpsc::WithNoThreads, Cmd, { crate::CHANNEL_SIZE }>;
pub type Sender<'ch> = mpsc::Sender<'ch, mpsc::WithNoThreads, Cmd, { crate::CHANNEL_SIZE }>;

/// A display & nor flash driver hard-coded to the PineTime.
///
/// These are combined because they share the same SPI peripheral.
pub struct DisplayFlashSpi {
    /// Backlight low brightness pin
    bl_low_pin: FlexPin<'static, P0_14>,
    /// Backlight medium brightness pin
    bl_mid_pin: FlexPin<'static, P0_22>,
    /// Backlight high brightness pin
    bl_high_pin: FlexPin<'static, P0_23>,
    /// SPI master periperal
    spim: TWISPI0,
    /// SPI master periperal interrupt
    spim_irq: interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    /// SPI clock pin
    spi_clock_pin: P0_02,
    /// SPI master -> slave data pin
    spi_mosi_pin: P0_03,
    /// SPI slave -> master data pin. I believe this isn't connected for the display.
    spi_miso_pin: P0_04,
    /// Display reset pin
    reset_pin: P0_26,
    /// Display chip select pin
    display_cs_pin: P0_25,
    /// Flash mem chip select pin
    flash_cs_pin: P0_05,
    /// Display data/command switch (low for command, high for data)
    dc_pin: P0_18,
}

/// Levels that the backlight can be set to.
#[derive(Debug, Format)]
pub enum Backlight {
    Off,
    Low,
    Mid,
    High,
}

impl DisplayFlashSpi {
    pub fn new(
        bl_low_pin: P0_14,
        bl_mid_pin: P0_22,
        bl_high_pin: P0_23,
        spim: TWISPI0,
        spim_irq: interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
        spi_clock_pin: P0_02,
        spi_mosi_pin: P0_03,
        spi_miso_pin: P0_04,
        reset_pin: P0_26,
        display_cs_pin: P0_25,
        flash_cs_pin: P0_05,
        dc_pin: P0_18,
    ) -> Self {
        let mut bl_low_pin = FlexPin::new(bl_low_pin);
        let mut bl_mid_pin = FlexPin::new(bl_mid_pin);
        let mut bl_high_pin = FlexPin::new(bl_high_pin);
        unwrap!(bl_low_pin.set_low());
        unwrap!(bl_mid_pin.set_low());
        unwrap!(bl_high_pin.set_low());
        Self {
            bl_low_pin,
            bl_mid_pin,
            bl_high_pin,
            spim,
            spim_irq,
            spi_clock_pin,
            spi_mosi_pin,
            spi_miso_pin,
            reset_pin,
            display_cs_pin,
            flash_cs_pin,
            dc_pin,
        }
    }

    /// Respond to a message in the channel.
    pub async fn handle(&mut self, cmd: Cmd) {
        debug!("{:?}", cmd);
        match cmd {
            Cmd::SleepOff => self.display().sleep_off().await,
            Cmd::SleepOn => self.display().sleep_on().await,
            Cmd::FillRectWithColor { area, color } => {
                debug!("about to fill");
                self.display()
                    .draw_rect_color(area, color.into_storage().to_be_bytes())
                    .await;
                debug!("filled");
            }
            Cmd::DrawImage {
                top_left,
                image,
                scale,
            } => self.display().draw_image(top_left, image, scale).await,
            Cmd::SetBacklight { level } => self.set_backlight(level),
            Cmd::PowerOn => {
                self.display().power_on().await;
                self.set_backlight(Backlight::High);
                Timer::after(Duration::from_secs(3)).await;
                self.set_backlight(Backlight::Off);
            }
        }
        debug!("finished cmd");
    }

    pub fn set_backlight(&mut self, level: Backlight) {
        use Backlight::*;
        // TODO remove me once I've checked this actually works.
        match level {
            Off => {
                self.bl_low_pin.set_as_disconnected();
                self.bl_mid_pin.set_as_disconnected();
                self.bl_high_pin.set_as_disconnected();
            }
            Low => {
                self.bl_low_pin.set_as_output(OutputDrive::Standard);
                self.bl_mid_pin.set_as_disconnected();
                self.bl_high_pin.set_as_disconnected();
            }
            Mid => {
                self.bl_low_pin.set_as_disconnected();
                self.bl_mid_pin.set_as_output(OutputDrive::Standard);
                self.bl_high_pin.set_as_disconnected();
            }
            High => {
                self.bl_low_pin.set_as_disconnected();
                self.bl_mid_pin.set_as_disconnected();
                self.bl_high_pin.set_as_output(OutputDrive::Standard);
            }
        }
    }

    pub fn display<'a>(&'a mut self) -> Display<'a> {
        Display::new(
            &mut self.spim,
            &mut self.spim_irq,
            &mut self.spi_clock_pin,
            &mut self.spi_mosi_pin,
            &mut self.reset_pin,
            &mut self.display_cs_pin,
            &mut self.dc_pin,
        )
    }
}

/// Initialize the display.
///
/// The SPI is powered down when this function exits. The backlight is handled separately.
pub struct Display<'a> {
    spim: Spim<'a, TWISPI0>,
    reset_pin: Output<'a, P0_26>,
    cs_pin: Output<'a, P0_25>,
    dc_pin: Output<'a, P0_18>,
}

impl<'a> Display<'a> {
    fn new(
        spim: &'a mut TWISPI0,
        irq: &'a mut interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
        sck_pin: &'a mut P0_02,
        mosi_pin: &'a mut P0_03,
        reset_pin: &'a mut P0_26,
        cs_pin: &'a mut P0_25,
        dc_pin: &'a mut P0_18,
    ) -> Display<'a> {
        let mut config = spim::Config::default();
        config.frequency = spim::Frequency::M8;
        config.mode = spim::MODE_3;
        config.orc = 122;
        let spim = Spim::new(spim, irq, sck_pin, NoPin, mosi_pin, config);
        let reset_pin = Output::new(reset_pin, Level::High, OutputDrive::Standard);
        // Drive low to send spi data, then drive high to signal end of data.
        let cs_pin = Output::new(cs_pin, Level::High, OutputDrive::Standard);
        // We will always set this before sending a command or data.
        let dc_pin = Output::new(dc_pin, Level::Low, OutputDrive::Standard);
        Self {
            spim,
            reset_pin,
            cs_pin,
            dc_pin,
        }
    }

    /// Turns on the screen.
    pub async fn sleep_off(&mut self) {
        self.hard_reset().await;
        unwrap!(self.cs_pin.set_low());
        self.soft_reset().await;
        self.sleep_out().await;
        self.pixel_format(
            RgbInterfaceColorFormat::_65KOfRgb,
            ControlInterfaceColorFormat::_16BitPerPixel,
        );
        self.vertical_scrolling_definition(0, 320, 0);
        self.memory_data_access_control(
            ColDir::BottomToTop,
            RowDir::LeftToRight,
            true,
            ColDir::BottomToTop,
            ColorOrder::Rgb,
            RowDir::LeftToRight,
        );
        self.invert_display(true).await;
        self.send_command(Instruction::NormalModeOn);
        Timer::after(Duration::from_millis(10)).await;
        self.send_command(Instruction::DisplayOn);
        unwrap!(self.cs_pin.set_high());
    }

    pub async fn sleep_on(&mut self) {
        unwrap!(self.cs_pin.set_low());
        self.sleep_in().await;
        unwrap!(self.cs_pin.set_high());
    }

    /// Copy a pre-existing buffer of data in RAM onto the screen.
    pub async fn draw_rect_buf(
        &mut self,
        area: Rectangle,
        // This byte array should contain 16 bit Rgb565 colors in big-endian order.
        buf: &[u8],
    ) {
        self.draw_rect_iter(area, buf.iter().copied()).await
    }

    /// Draw an image at the given location. The image must be [width, height, data...];
    pub async fn draw_image(&mut self, top_left: Point, buf: &[u8], scale: u8) {
        struct Scaler<'a> {
            scale: u8,
            width: u8,
            buf: &'a [u8],
            pos: usize,
        }

        impl<'a> Scaler<'a> {
            fn new(scale: u8, width: u8, buf: &'a [u8]) -> Self {
                debug_assert_eq!(buf.len() % width as usize, 0);
                Scaler {
                    scale,
                    width,
                    buf,
                    pos: 0,
                }
            }
        }

        impl Iterator for Scaler<'_> {
            type Item = u8;
            fn next(&mut self) -> Option<Self::Item> {
                let width = self.width as usize * self.scale as usize;
                // pixels come in pairs of bytes (u16).
                let (pos, byte) = (self.pos / 2, self.pos % 2);
                let (row, col) = (pos / width, pos % width);
                let (orig_row, orig_col) = (row / self.scale as usize, col / self.scale as usize);
                let out = self.buf.get((orig_row * width + orig_col) * 2 + byte);
                self.pos += 1;
                out.map(|s| *s)
            }
        }

        assert!(scale > 0);
        let width = buf[0];
        let height = buf[1];
        defmt::debug!(
            "size ({=u8}, {=u8}) orig ({=u8}, {=u8})",
            width * scale,
            height * scale,
            width,
            height
        );
        Timer::after(Duration::from_secs(3)).await;
        let buf = &buf[2..];
        let area = Rectangle::new(
            top_left,
            Size::new(width as u32 * scale as u32, height as u32 * scale as u32),
        );
        self.draw_rect_iter(area, Scaler::new(scale, width, buf))
            .await
    }

    /// Copy data from an iterator onto the screen.
    ///
    /// This method copies into an intermediate buffer.
    pub async fn draw_rect_iter(&mut self, area: Rectangle, data: impl IntoIterator<Item = u8>) {
        check_rect_u16(area);
        let tl = area.top_left;
        // No area means top-left = bottom-right
        let br = match area.bottom_right() {
            Some(x) => x,
            None => tl,
        };

        defmt::debug!("set address window");
        unwrap!(self.cs_pin.set_low());
        self.set_address_window(tl.x as u16, tl.y as u16, br.x as u16, br.y as u16);

        defmt::debug!("write to ram");
        self.send_command(Instruction::WriteToRam);
        // chunk into slices of max EASY_DMA_SIZE
        let mut buffer = [0u8; crate::EASY_DMA_SIZE];
        let mut cnt = 0;
        let mut i = 0;
        for (idx, byte) in data.into_iter().enumerate() {
            *unwrap!(buffer.get_mut(i)) = byte;
            i += 1;
            if i >= crate::EASY_DMA_SIZE {
                //defmt::debug!("send buffer {} (idx {=usize})", cnt, idx);
                self.send_data_blocking(&buffer);
                i = 0;
                cnt += 1;
            }
        }
        defmt::debug!("i = {}", i);
        // send the last buffer
        if i > 0 {
            //defmt::debug!("send buffer remaining {=usize}", i);
            self.send_data(&buffer[..i]).await;
        }
        defmt::debug!("pin high");
        unwrap!(self.cs_pin.set_high());
    }

    /// Draw a filled Rectangle with the given color.
    ///
    /// This method copies into an intermediate buffer. It assumes that the color has already been
    /// converted into bytes, with the first byte sent on the wire first. (i.e. big endian)
    pub async fn draw_rect_color<const COLOR_BYTES: usize>(
        &mut self,
        area: Rectangle,
        color: [u8; COLOR_BYTES],
    ) {
        self.draw_rect_iter(
            area,
            iter::repeat(&color)
                .take((area.size.width * area.size.height) as usize)
                .flatten()
                .copied(),
        )
        .await;
    }

    pub async fn clear(&mut self, color: Rgb565) {
        self.draw_rect_color(
            Rectangle::new(Point::new(0, 0), Size::new(240, 240)),
            color.into_storage().to_be_bytes(),
        )
        .await
    }

    pub async fn power_on(&mut self) {
        self.sleep_off();
        // clear screen
        self.clear(Rgb565::WHITE).await;
    }

    // Display commands and their args.

    #[inline]
    async fn hard_reset(&mut self) {
        //debug!("hard_reset");
        unwrap!(self.reset_pin.set_high());
        Timer::after(Duration::from_micros(10)).await;
        unwrap!(self.reset_pin.set_low());
        Timer::after(Duration::from_micros(10)).await;
        unwrap!(self.reset_pin.set_high());
        Timer::after(Duration::from_micros(10)).await;
    }

    #[inline]
    async fn soft_reset(&mut self) {
        //debug!("soft_reset");
        self.send_command(Instruction::SoftwareReset);
        Timer::after(Duration::from_millis(150)).await;
    }

    #[inline]
    async fn sleep_out(&mut self) {
        //debug!("sleep_out");
        self.send_command(Instruction::SleepOut);
        Timer::after(Duration::from_millis(10)).await;
    }

    #[inline]
    async fn sleep_in(&mut self) {
        //debug!("sleep_out");
        self.send_command(Instruction::SleepIn);
        Timer::after(Duration::from_millis(10)).await;
    }

    #[inline]
    fn pixel_format(
        &mut self,
        color: RgbInterfaceColorFormat,
        ctrl: ControlInterfaceColorFormat, // this is the one that we use I think
    ) {
        //debug!("pixel_format");
        // 16bit 65k colors
        self.send_command(Instruction::InterfacePixelFormat);
        self.send_data_blocking(&[color as u8 | ctrl as u8]);
    }

    /// Invert the colors of the display, so e.g. red would become cyan.
    #[inline]
    async fn invert_display(&mut self, invert: bool) {
        if invert {
            self.send_command(Instruction::DisplayInversionOn);
        } else {
            self.send_command(Instruction::DisplayInversionOff);
        }
        Timer::after(Duration::from_millis(10)).await;
    }

    #[inline]
    fn vertical_scrolling_definition(
        &mut self,
        top_fixed_area: u16,
        vertical_scrolling_area: u16,
        bottom_fixed_area: u16,
    ) {
        //debug!("vertical_scrolling_definition");
        debug_assert_eq!(
            top_fixed_area + vertical_scrolling_area + bottom_fixed_area,
            320
        );
        let tfa = top_fixed_area.to_be_bytes();
        let vsa = vertical_scrolling_area.to_be_bytes();
        let bfa = bottom_fixed_area.to_be_bytes();
        let data = [tfa[0], tfa[1], vsa[0], vsa[1], bfa[0], bfa[1]];
        self.send_command(Instruction::VerticalScrollingDefinition);
        self.send_data_blocking(&data);
    }

    #[inline]
    fn memory_data_access_control(
        &mut self,
        page_address_order: ColDir,
        column_address_order: RowDir,
        page_col_order_reverse: bool,
        line_address_order: ColDir,
        color_order: ColorOrder,
        display_data_latch_order: RowDir,
    ) {
        // let's hope this all disappears through optimization (constant propagation)
        let mut mdac = 0;
        if matches!(page_address_order, ColDir::BottomToTop) {
            mdac |= 1 << 7;
        }
        if matches!(column_address_order, RowDir::RightToLeft) {
            mdac |= 1 << 6;
        }
        if page_col_order_reverse {
            mdac |= 1 << 5;
        }
        if matches!(line_address_order, ColDir::BottomToTop) {
            mdac |= 1 << 4;
        }
        if matches!(color_order, ColorOrder::Bgr) {
            mdac |= 1 << 3;
        }
        if matches!(display_data_latch_order, RowDir::RightToLeft) {
            mdac |= 1 << 2;
        }
        self.send_command(Instruction::MemoryDataAccessControl);
        self.send_data_blocking(&[mdac]);
    }

    #[inline]
    fn set_address_window(&mut self, startx: u16, starty: u16, endx: u16, endy: u16) {
        // The screen we use (ST7789) actually has video memory that is slightly bigger than the
        // screen, and supports scrolling. I think this is there to mean you can update the image
        // faster than you can write data to it using the SPI.
        // Because we need to change the display's orientation, we actually show the bottom bit of
        // the framebuffer. This offset means we draw to the part of the framebuffer that is
        // visible.
        const SCROLL_OFFSET: u16 = 80;
        let startx = (startx + SCROLL_OFFSET).to_be_bytes();
        let starty = starty.to_be_bytes();
        let endx = (endx + SCROLL_OFFSET).to_be_bytes();
        let endy = endy.to_be_bytes();

        // column address
        self.send_command(Instruction::ColumnAddressSet);
        self.send_data_blocking(&[startx[0], startx[1], endx[0], endx[1]]);

        // row address
        self.send_command(Instruction::RowAddressSet);
        self.send_data_blocking(&[starty[0], starty[1], endy[0], endy[1]]);
    }

    #[inline]
    fn vertical_scroll_start_address(&mut self, line: u16) {
        self.send_command(Instruction::VerticalScrollStartAddress);
        self.send_data_blocking(&line.to_be_bytes());
    }

    // raw methods for commands & data

    #[inline]
    fn send_command(&mut self, inst: Instruction) {
        unwrap!(self.dc_pin.set_low());
        unwrap!(embedded_hal::blocking::spi::Write::write(
            &mut self.spim,
            &[inst.into()]
        ));
    }

    #[inline]
    fn send_data_blocking(&mut self, data: &[u8]) {
        unwrap!(self.dc_pin.set_high());
        unwrap!(embedded_hal::blocking::spi::Write::write(
            &mut self.spim,
            data
        ));
    }

    #[inline]
    async fn send_data(&mut self, data: &[u8]) {
        /*
        defmt::debug!(
            "writing data at {=usize:x} -> {=usize:x}",
            data.as_ptr() as _,
            (data.as_ptr() as usize) + data.len()
        );
        */
        unwrap!(self.dc_pin.set_high());
        //defmt::debug!("let's go");
        unwrap!(self.spim.write(data).await);
        //defmt::debug!("write finished");
    }
}

/// ST7789 instructions (some of them).
#[repr(u8)]
enum Instruction {
    NoOp = 0x00,
    SoftwareReset = 0x01,
    ReadDisplayID = 0x04,
    ReadDisplayStatus = 0x09,
    SleepIn = 0x10,
    SleepOut = 0x11,
    PartialModeOn = 0x12,
    NormalModeOn = 0x13,
    DisplayInversionOff = 0x20,
    DisplayInversionOn = 0x21,
    DisplayOff = 0x28,
    DisplayOn = 0x29,
    ColumnAddressSet = 0x2A,
    RowAddressSet = 0x2B,
    WriteToRam = 0x2C,
    ReadFromRam = 0x2E,
    PartialStartEndAddressSet = 0x30,
    VerticalScrollingDefinition = 0x33,
    MemoryDataAccessControl = 0x36,
    VerticalScrollStartAddress = 0x37,
    InterfacePixelFormat = 0x3A,
    VcomOffsetSet = 0xC5,
}

#[repr(u8)]
enum RgbInterfaceColorFormat {
    _65KOfRgb = 0b0101_0000,
    _262KOfRgb = 0b0110_0000,
}

#[repr(u8)]
enum ControlInterfaceColorFormat {
    _12BitPerPixel = 0b0011,
    _16BitPerPixel = 0b0101,
    // default (after hw reset and power on)
    _18BitPerPixel = 0b0110,
    _16MTruncated = 0b0111,
}

enum ColDir {
    TopToBottom,
    BottomToTop,
}

enum RowDir {
    LeftToRight,
    RightToLeft,
}

enum ColorOrder {
    Rgb,
    Bgr,
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self as u8
    }
}

fn check_rect_u16(r: Rectangle) {
    let tl = r.top_left;
    // No area means top-left = bottom-right
    let br = match r.bottom_right() {
        Some(x) => x,
        None => tl,
    };
    let max = u16::MAX as i32;
    defmt::assert!(
        tl.x >= 0
            && tl.x <= max
            && tl.y >= 0
            && tl.y <= max
            && br.x >= 0
            && br.x <= max
            && br.y >= 0
            && br.y <= max
    );
}
