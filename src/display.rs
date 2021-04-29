use core::pin::Pin;
use embassy::{
    time::{Duration, Timer},
    traits::spi::FullDuplex,
    util::PeripheralBorrow,
};
use embassy_extras::unborrow;
use embassy_nrf::{
    gpio::{AnyPin, Level, NoPin, Output, OutputDrive, Pin as GpioPin},
    hal::prelude::*,
    spim::{self, Spim},
    target_constants::EASY_DMA_SIZE,
};
use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::*,
};
use pin_utils::{unsafe_pinned, unsafe_unpinned};
use rtt_target::rprintln;

pub use embedded_graphics::{geometry::Point, primitives::Rectangle};

const DISPLAY_WIDTH: usize = 240;
const DISPLAY_HEIGHT: usize = 240;
const DISPLAY_AREA: Rectangle = Rectangle {
    top_left: Point { x: 0, y: 0 },
    bottom_right: Point {
        x: DISPLAY_WIDTH as i32,
        y: DISPLAY_HEIGHT as i32,
    },
};
const DISPLAY_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
// we are writing u16s so we need an even number of bytes.
const BUFFER_SIZE: usize = EASY_DMA_SIZE - (EASY_DMA_SIZE % 2);

/// A display driver hard-coded to the PineTime.
pub struct Display<'d, T: spim::Instance> {
    // TODO borrow pins with PeripheralBorrow
    /// Backlight pin 1
    bl1_pin: Output<'static, AnyPin>,
    /// Backlight pin 2
    bl2_pin: Output<'static, AnyPin>,
    /// Backlight pin 3
    bl3_pin: Output<'static, AnyPin>,
    /// SPI master for the display
    spim: Spim<'d, T>,
    /// Reset pin
    rst_pin: Output<'static, AnyPin>,
    /// Chip select pin
    cs_pin: Output<'static, AnyPin>,
    /// data/clock switch
    dc_pin: Output<'static, AnyPin>,
    /// Is the screen on
    display_on: bool,
    // Commented out because spim captures the lifetime, but left in to remind us that we rely on
    // this.
    ///// We need this because we must unborrow the pins before we can degrade them.
    //marker: PhantomData<&'d mut ()>,
}

/// Levels that the backlight can be set to.
pub enum Backlight {
    Off,
    Low,
    Mid,
    High,
}

impl<'d, T: spim::Instance> Display<'d, T> {
    unsafe_pinned!(spim: Spim<'d, T>);
    unsafe_unpinned!(bl1_pin: Output<'static, AnyPin>);
    unsafe_unpinned!(bl2_pin: Output<'static, AnyPin>);
    unsafe_unpinned!(bl3_pin: Output<'static, AnyPin>);
    unsafe_unpinned!(rst_pin: Output<'static, AnyPin>);
    unsafe_unpinned!(dc_pin: Output<'static, AnyPin>);
    unsafe_unpinned!(display_on: bool);

    pub fn new(
        bl1_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        bl2_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        bl3_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        spim: impl PeripheralBorrow<Target = T> + 'd,
        irq: impl PeripheralBorrow<Target = T::Interrupt> + 'd,
        sck_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        mosi_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        rst_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        cs_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
        dc_pin: impl PeripheralBorrow<Target = impl GpioPin> + 'd,
    ) -> Self {
        unborrow!(bl1_pin, bl2_pin, bl3_pin, rst_pin, cs_pin, dc_pin);

        let config = spim::Config {
            frequency: spim::Frequency::M8,
            mode: spim::MODE_3,
            orc: 122,
        };
        let spim = Spim::new(spim, irq, sck_pin, NoPin, mosi_pin, config);
        Self {
            bl1_pin: Output::new(bl1_pin.degrade(), Level::High, OutputDrive::Standard),
            bl2_pin: Output::new(bl2_pin.degrade(), Level::High, OutputDrive::Standard),
            bl3_pin: Output::new(bl3_pin.degrade(), Level::High, OutputDrive::Standard),
            spim,
            rst_pin: Output::new(rst_pin.degrade(), Level::Low, OutputDrive::Standard),
            cs_pin: Output::new(cs_pin.degrade(), Level::Low, OutputDrive::Standard),
            dc_pin: Output::new(dc_pin.degrade(), Level::Low, OutputDrive::Standard),
            display_on: false,
            //marker: PhantomData,
        }
    }

    /// Turns on the screen.
    pub async fn sleep_off(self: &mut Pin<&mut Self>) {
        self.hard_reset().await;
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
    }

    pub fn set_backlight(&mut self, level: Backlight) {
        use Backlight::*;
        match level {
            Off => {
                self.bl1_pin.set_high().unwrap();
                self.bl2_pin.set_high().unwrap();
                self.bl3_pin.set_high().unwrap();
            }
            Low => {
                self.bl1_pin.set_low().unwrap();
                self.bl2_pin.set_high().unwrap();
                self.bl3_pin.set_high().unwrap();
            }
            Mid => {
                self.bl1_pin.set_low().unwrap();
                self.bl2_pin.set_low().unwrap();
                self.bl3_pin.set_high().unwrap();
            }
            High => {
                self.bl1_pin.set_low().unwrap();
                self.bl2_pin.set_low().unwrap();
                self.bl3_pin.set_low().unwrap();
            }
        }
    }

    /// Copy a pre-existing buffer of data in RAM onto the screen.
    pub async fn draw_rect_buf(
        self: &mut Pin<&mut Self>,
        area: Rect,
        // This byte array should contain 16 bit Rgb565 colors in big-endian order.
        mut buf: &[u8],
    ) {
        debug_assert_eq!(buf.len(), area.area() * 2);
        // TODO check the buffer is in RAM (requirement for DMA).

        self.set_address_window(
            area.x0() as u16,
            area.y0() as u16,
            // The screen expects the bottom right corner to be inside the Rect, we use the
            // convention that it is outside.
            area.x1() as u16 - 1,
            area.y1() as u16 - 1,
        );

        self.send_command(Instruction::WriteToRam);
        // chunk into slices of max EASY_DMA_SIZE
        for chunk in buf.chunks(EASY_DMA_SIZE) {
            self.send_data(chunk).await;
        }
    }

    /// Copy data from an iterator onto the screen.
    ///
    /// This method copies into an intermediate buffer.
    pub async fn draw_rect_iter(
        self: &mut Pin<&mut Self>,
        area: Rect,
        data: impl IntoIterator<Item = u8>,
    ) {
        self.set_address_window(
            area.x0() as u16,
            area.y0() as u16,
            // The screen expects the bottom right corner to be inside the Rect, we use the
            // convention that it is outside.
            area.x1() as u16 - 1,
            area.y1() as u16 - 1,
        );

        self.send_command(Instruction::WriteToRam);
        // chunk into slices of max EASY_DMA_SIZE
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut i = 0;
        for byte in data.into_iter() {
            buffer[i] = byte;
            i += 1;
            if i >= BUFFER_SIZE {
                self.send_data(&buffer).await;
                i = 0;
            }
        }
        // send the last buffer
        if i > 0 {
            self.send_data(&buffer[..i]).await;
        }
    }

    /*
        #[inline]
        /// Clear the display to the selected color, batching pixels for speed.
        pub async fn clear(self: &mut Pin<&mut Self>, color: Rgb565) {
            self.clear_rect(color, DISPLAY_AREA).await
        }

        /// Clear the display to the selected color, batching pixels for speed.
        pub async fn clear_rect(self: &mut Pin<&mut Self>, color: Rgb565, rect: Rectangle) {
            let rect = intersect(rect, DISPLAY_AREA);
            if area(rect) == 0 {
                // nothing to do
                return;
            }
            let color = (RawU16::from(color).into_inner()).to_be_bytes();
            //rprintln!("fill {:?}", color);
            let mut buffer = [0u8; BUFFER_SIZE];

            self.set_address_window(
                rect.top_left.x as u16,
                rect.top_left.y as u16,
                rect.bottom_right.x as u16 - 1,
                rect.bottom_right.y as u16 - 1,
            );
            self.send_command(Instruction::WriteToRam);

            let display_bytes = area(rect).max(0) as usize * 2;
            let mut data_sent = 0;
            loop {
                //rprintln!("{} - {} <= {}", display_bytes, data_sent, BUFFER_SIZE);
                if display_bytes - data_sent <= BUFFER_SIZE {
                    let mut i = 0;
                    while i < display_bytes - data_sent {
                        debug_assert!(i < BUFFER_SIZE - 1);
                        buffer[i] = color[0];
                        buffer[i + 1] = color[1];
                        i += 2;
                    }
                    self.send_data(&buffer[..i]).await;
                    break;
                } else {
                    // manual loop to step 2
                    let mut i = 0;
                    while i < BUFFER_SIZE {
                        debug_assert!(i < BUFFER_SIZE - 1);
                        buffer[i] = color[0];
                        buffer[i + 1] = color[1];
                        i += 2;
                    }
                    self.send_data(&buffer).await;
                    data_sent += BUFFER_SIZE;
                }
            }
        }

        /// Draw pixels to a rectangular area of the screen.
        pub async fn draw_rect(
            self: &mut Pin<&mut Self>,
            rect: Rectangle,
            colors: impl Iterator<Item = Rgb565>,
        ) {
            let rect = intersect(rect, DISPLAY_AREA);
            if area(rect) == 0 {
                // nothing to do
                return;
            }
            let color = (RawU16::from(color).into_inner()).to_be_bytes();
            //rprintln!("fill {:?}", color);
            let mut buffer = [0u8; BUFFER_SIZE];

            self.set_address_window(
                rect.top_left.x as u16,
                rect.top_left.y as u16,
                rect.bottom_right.x as u16 - 1,
                rect.bottom_right.y as u16 - 1,
            );
            self.send_command(Instruction::WriteToRam);

            let display_bytes = area(rect).max(0) as usize * 2;
            let mut data_sent = 0;
            loop {
                //rprintln!("{} - {} <= {}", display_bytes, data_sent, BUFFER_SIZE);
                if display_bytes - data_sent <= BUFFER_SIZE {
                    let mut i = 0;
                    while i < display_bytes - data_sent {
                        debug_assert!(i < BUFFER_SIZE - 1);
                        buffer[i] = color[0];
                        buffer[i + 1] = color[1];
                        i += 2;
                    }
                    self.send_data(&buffer[..i]).await;
                    break;
                } else {
                    // manual loop to step 2
                    let mut i = 0;
                    while i < BUFFER_SIZE {
                        debug_assert!(i < BUFFER_SIZE - 1);
                        buffer[i] = color[0];
                        buffer[i + 1] = color[1];
                        i += 2;
                    }
                    self.send_data(&buffer).await;
                    data_sent += BUFFER_SIZE;
                }
            }
        }
    */

    // Commands and their args.

    #[inline]
    async fn hard_reset(self: &mut Pin<&mut Self>) {
        self.as_mut().rst_pin().set_high().unwrap();
        Timer::after(Duration::from_micros(10)).await;
        self.as_mut().rst_pin().set_low().unwrap();
        Timer::after(Duration::from_micros(10)).await;
        self.as_mut().rst_pin().set_high().unwrap();
        Timer::after(Duration::from_micros(10)).await;
    }

    #[inline]
    async fn soft_reset(self: &mut Pin<&mut Self>) {
        self.send_command(Instruction::SoftwareReset);
        Timer::after(Duration::from_millis(150)).await;
    }

    #[inline]
    async fn sleep_out(self: &mut Pin<&mut Self>) {
        self.send_command(Instruction::SleepOut);
        Timer::after(Duration::from_millis(10)).await;
    }

    #[inline]
    fn pixel_format(
        self: &mut Pin<&mut Self>,
        color: RgbInterfaceColorFormat,
        ctrl: ControlInterfaceColorFormat, // this is the one that we use I think
    ) {
        // 16bit 65k colors
        self.send_command(Instruction::InterfacePixelFormat);
        self.send_data_blocking(&[color as u8 | ctrl as u8]);
    }

    /// Invert the colors of the display, so e.g. red would become cyan.
    #[inline]
    async fn invert_display(self: &mut Pin<&mut Self>, invert: bool) {
        if invert {
            self.send_command(Instruction::DisplayInversionOn);
        } else {
            self.send_command(Instruction::DisplayInversionOff);
        }
        Timer::after(Duration::from_millis(10)).await;
    }

    #[inline]
    fn vertical_scrolling_definition(
        self: &mut Pin<&mut Self>,
        top_fixed_area: u16,
        vertical_scrolling_area: u16,
        bottom_fixed_area: u16,
    ) {
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
        self: &mut Pin<&mut Self>,
        page_address_order: ColDir,
        column_address_order: RowDir,
        page_col_order_reverse: bool,
        line_address_order: ColDir,
        color_order: ColorOrder,
        display_data_latch_order: RowDir,
    ) {
        // let's hope this all disappears through optimization
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
    fn set_address_window(
        self: &mut Pin<&mut Self>,
        startx: u16,
        starty: u16,
        endx: u16,
        endy: u16,
    ) {
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
    fn vertical_scroll_start_address(self: &mut Pin<&mut Self>, line: u16) {
        self.send_command(Instruction::VerticalScrollStartAddress);
        self.send_data_blocking(&line.to_be_bytes());
    }

    // raw methods for commands & data

    #[inline]
    fn send_command(self: &mut Pin<&mut Self>, inst: Instruction) {
        self.as_mut().dc_pin().set_low().unwrap();
        self.as_mut().spim().write_blocking(&[inst.into()]).unwrap();
    }

    #[inline]
    fn send_data_blocking(self: &mut Pin<&mut Self>, data: &[u8]) {
        self.as_mut().dc_pin().set_high().unwrap();
        self.as_mut().spim().write_blocking(data).unwrap();
    }

    #[inline]
    async fn send_data(self: &mut Pin<&mut Self>, data: &[u8]) {
        self.as_mut().dc_pin().set_high().unwrap();
        self.as_mut().spim().write(data).await.unwrap();
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

fn intersect(rect1: Rectangle, rect2: Rectangle) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: rect1.top_left.x.max(rect2.top_left.x),
            y: rect1.top_left.y.max(rect2.top_left.y),
        },
        bottom_right: Point {
            x: rect1.bottom_right.x.min(rect2.bottom_right.x),
            y: rect1.bottom_right.y.min(rect2.bottom_right.y),
        },
    }
}

fn area(rect: Rectangle) -> i32 {
    let area = (rect.bottom_right.x - rect.top_left.x) * (rect.bottom_right.y - rect.top_left.y);
    area.max(0)
}
