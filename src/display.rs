use core::{
    mem::{self, MaybeUninit},
    pin::Pin as StdPin,
    ptr,
};
use embassy::{
    time::{Duration, Timer},
    traits::spi::FullDuplex,
};
use embassy_nrf::{
    hal::{
        gpio::{Disconnected, Level, Output, Pin, PushPull},
        prelude::*,
    },
    spim::{self, Spim},
};
use pin_utils::{pin_mut, unsafe_pinned, unsafe_unpinned};
use rtt_target::rprintln;

const DISPLAY_BATCH_SIZE: usize = 2048;

pub struct DisplayOn<S: spim::Instance> {
    /// Backlight pin
    bl_pin: Pin<Output<PushPull>>,
    /// Reset pin
    rst_pin: Pin<Output<PushPull>>,
    /// Chip select pin
    cs_pin: Pin<Output<PushPull>>,
    /// data/clock switch
    dc_pin: Pin<Output<PushPull>>,
    /// SPI master for controlling display
    spim: Spim<S>,
    /// Pin numbers to recover SPI pins on power off.
    spi_clk_psel_bits: u32,
    spi_mosi_psel_bits: u32,
}

pub struct DisplayOff<S: spim::Instance> {
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
    /// SPI master peripheral
    spim_p: S,
    /// Matching SPI interrupt
    interrupt: S::Interrupt,
}

impl<S> DisplayOff<S>
where
    S: spim::Instance,
{
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
        spim_p: S,
        interrupt: S::Interrupt,
    ) -> Self {
        Self {
            bl_pin,
            rst_pin,
            cs_pin,
            dc_pin,
            spi_clk_pin,
            spi_mosi_pin,
            spim_p,
            interrupt,
        }
    }

    /// Assert that the display is powered on
    pub async fn power_on(self) -> DisplayOn<S> {
        // Turn on pins
        let spi_clk_psel_bits = self.spi_clk_pin.psel_bits();
        let spi_mosi_psel_bits = self.spi_mosi_pin.psel_bits();
        let bl_pin = self.bl_pin.into_push_pull_output(Level::Low);
        let rst_pin = self.rst_pin.into_push_pull_output(Level::Low);
        let cs_pin = self.cs_pin.into_push_pull_output(Level::Low);
        let dc_pin = self.dc_pin.into_push_pull_output(Level::Low);
        let spi_clk_pin = self.spi_clk_pin.into_push_pull_output(Level::Low);
        let spi_mosi_pin = self.spi_mosi_pin.into_push_pull_output(Level::Low);

        // construct spim
        let spi_config = spim::Config {
            pins: spim::Pins {
                sck: spi_clk_pin,
                mosi: Some(spi_mosi_pin),
                miso: None,
            },
            frequency: spim::Frequency::M8,
            mode: spim::MODE_3,
            orc: 122,
        };
        let spim = Spim::new(self.spim_p, self.interrupt, spi_config);

        let mut display = DisplayOn {
            bl_pin,
            rst_pin,
            cs_pin,
            dc_pin,
            spim,
            spi_clk_psel_bits,
            spi_mosi_psel_bits,
        };

        // hard reset the display
        display.hard_reset().await;

        unsafe {
            let mut display = StdPin::new_unchecked(&mut display);
            display.send_command(Instruction::SWRESET).await; // reset display
            Timer::after(Duration::from_millis(150)).await;
            display.send_command(Instruction::SLPOUT).await; // turn off sleep
            Timer::after(Duration::from_millis(10)).await;
            display.send_command(Instruction::INVOFF).await; // turn off invert
            display.send_command(Instruction::VSCRDER).await; // vertical scroll definition
            let data = [0u8, 0u8, 0x14u8, 0u8, 0u8, 0u8];
            display.send_data(&data).await; // 0 TSA, 320 VSA, 0 BSA
            display.send_command(Instruction::MADCTL).await; // left -> right, bottom -> top RGB
            let data = [0b0000_0000];
            display.send_data(&data).await;
            display.send_command(Instruction::COLMOD).await; // 16bit 65k colors
            let data = [0b0101_0101];
            display.send_data(&data).await;
            display.send_command(Instruction::INVON).await; // hack?
            Timer::after(Duration::from_millis(10)).await;
            display.send_command(Instruction::NORON).await; // turn on display
            Timer::after(Duration::from_millis(10)).await;
            display.send_command(Instruction::DISPON).await; // turn on display
            Timer::after(Duration::from_millis(10)).await;
        } // pinned ref is dropped, releasing display

        display
    }
}

impl<S> DisplayOn<S>
where
    S: spim::Instance,
{
    unsafe_pinned!(spim: Spim<S>);
    unsafe_unpinned!(bl_pin: Pin<Output<PushPull>>);
    unsafe_unpinned!(rst_pin: Pin<Output<PushPull>>);
    unsafe_unpinned!(cs_pin: Pin<Output<PushPull>>);
    unsafe_unpinned!(dc_pin: Pin<Output<PushPull>>);

    pub async fn power_off(self) -> DisplayOff<S> {
        let (spim_p, interrupt) = self.spim.into_inner();
        // the pins are now unused, so safe to steal them again. TODO maybe suggest that
        // `free` should return ownership of the pins
        let (spi_clk_pin, spi_mosi_pin): (Pin<Output<PushPull>>, Pin<Output<PushPull>>) = unsafe {
            (
                Pin::from_psel_bits(self.spi_clk_psel_bits),
                Pin::from_psel_bits(self.spi_mosi_psel_bits),
            )
        };
        DisplayOff {
            bl_pin: self.bl_pin.into_disconnected(),
            rst_pin: self.rst_pin.into_disconnected(),
            cs_pin: self.cs_pin.into_disconnected(),
            dc_pin: self.dc_pin.into_disconnected(),
            spi_clk_pin: spi_clk_pin.into_disconnected(),
            spi_mosi_pin: spi_mosi_pin.into_disconnected(),
            spim_p,
            interrupt,
        }
    }

    /// Send a display command.
    ///
    /// There will be a different function for each N used. Might be better to push the length to
    /// runtime with a slice, but then would have to work out how to provide space for discard.
    /// alloca would be perfect.
    async fn send_commands(self: &mut StdPin<&mut Self>, instructions: &[Instruction]) {
        self.as_mut().dc_pin().set_low().unwrap();
        // Safety: instructions are valid u8.
        self.as_mut()
            .spim()
            .write(unsafe { mem::transmute(&[instructions][..]) })
            .await
            .unwrap()
    }

    #[inline]
    async fn send_command(self: &mut StdPin<&mut Self>, instruction: Instruction) {
        self.as_mut().dc_pin().set_low().unwrap();
        self.send_commands(&[instruction]).await
    }

    /// Send display data.
    ///
    /// Panics if display not in right mode.
    async fn send_data(self: &mut StdPin<&mut Self>, data: &[u8]) {
        self.as_mut().dc_pin().set_high().unwrap();
        self.as_mut().spim().write(data).await.unwrap()
    }

    pub async fn set_pixel(self: &mut StdPin<&mut Self>, x: u16, y: u16, color: u16) {
        self.set_address_window(x, y, x, y).await;
        self.send_command(Instruction::RAMWR).await;
        self.send_data(&color.to_be_bytes()).await;
    }

    /// Set a region of pixels from an iterator. Pixels are chunked into DISPLAY_BATCH_SIZE
    /// batches, the size of which is a trade-off between space and speed.
    pub async fn set_pixel_region(
        self: &mut StdPin<&mut Self>,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        pixels: impl Iterator<Item = u16>,
    ) {
        unsafe {
            self.set_address_window(x0, y0, x1, y1).await;
            self.send_command(Instruction::RAMWR).await;
            let mut chunks = Chunks::new(FlattenArray::new(pixels.map(|p| p.to_be_bytes())));
            while let Some(buf) = chunks.next() {
                self.send_data(buf).await;
            }
        }
    }

    async fn set_address_window(
        self: &mut StdPin<&mut Self>,
        x_start: u16,
        y_start: u16,
        x_end: u16,
        y_end: u16,
    ) {
        rprintln!("set address window");
        let x_start = x_start.to_be_bytes();
        let x_end = x_end.to_be_bytes();
        let x = [x_start[0], x_start[1], x_end[0], x_end[1]];
        rprintln!("send command CASET");
        self.send_command(Instruction::CASET).await;
        rprintln!("send data {:?}", x);
        self.send_data(&x).await;
        let y_start = y_start.to_be_bytes();
        let y_end = y_end.to_be_bytes();
        let y = [y_start[0], y_start[1], y_end[0], y_end[1]];
        rprintln!("send command RASET");
        self.send_command(Instruction::RASET).await;
        rprintln!("send data {:?}", y);
        self.send_data(&y).await;
        rprintln!("finish: set address window");
    }

    /// Reset the screen.
    pub async fn hard_reset(&mut self) {
        hard_reset(&mut self.rst_pin).await
    }
}

async fn hard_reset(rst_pin: &mut Pin<Output<PushPull>>) {
    rst_pin.set_high().unwrap();
    Timer::after(Duration::from_micros(10)).await;
    rst_pin.set_low().unwrap();
    Timer::after(Duration::from_micros(10)).await;
    rst_pin.set_high().unwrap();
    Timer::after(Duration::from_micros(10)).await;
}

/// ST7789 instructions.
#[repr(u8)]
enum Instruction {
    NOP = 0x00,
    SWRESET = 0x01,
    RDDID = 0x04,
    RDDST = 0x09,
    SLPIN = 0x10,
    SLPOUT = 0x11,
    PTLON = 0x12,
    NORON = 0x13,
    INVOFF = 0x20,
    INVON = 0x21,
    DISPOFF = 0x28,
    DISPON = 0x29,
    CASET = 0x2A,
    RASET = 0x2B,
    RAMWR = 0x2C,
    RAMRD = 0x2E,
    PTLAR = 0x30,
    VSCRDER = 0x33,
    COLMOD = 0x3A,
    MADCTL = 0x36,
    VSCAD = 0x37,
    VCMOFSET = 0xC5,
}

/// Collects an iterator into batches for DMA.
struct Chunks<I> {
    iter: I,
    buf: [MaybeUninit<u8>; DISPLAY_BATCH_SIZE],
}

impl<I> Chunks<I>
where
    I: Iterator<Item = u8>,
{
    fn new(iter: I) -> Self {
        Chunks {
            iter,
            buf: MaybeUninit::uninit_array(),
        }
    }

    // Can't use iterator because we can't tie the lifetime of the array to self.
    fn next(&mut self) -> Option<&[u8]> {
        // maybe there is a better way to do this than a for loop
        let mut i = 0;
        while i < DISPLAY_BATCH_SIZE {
            match self.iter.next() {
                Some(b) => {
                    self.buf[i].write(b);
                    i += 1;
                }
                None => break,
            }
        }
        if i > 0 {
            unsafe { Some(MaybeUninit::slice_assume_init_ref(&self.buf[..i])) }
        } else {
            None
        }
    }
}

/// Consumes an iterator of arrays, and returns an iterator flatting the arrays into a single
/// stream of values.
struct FlattenArray<T, I, const N: usize> {
    inner: I,
    buf: [MaybeUninit<T>; N],
    pos: usize,
}

impl<T, I, const N: usize> FlattenArray<T, I, { N }>
where
    I: Iterator<Item = [T; N]>,
{
    fn new(inner: I) -> Self {
        Self {
            inner,
            buf: MaybeUninit::uninit_array(),
            pos: 0,
        }
    }
}

impl<T, I, const N: usize> Iterator for FlattenArray<T, I, { N }>
where
    I: Iterator<Item = [T; N]>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.pos < N {
                // we've still got data
                let out = Some(self.buf[self.pos].assume_init_read());
                self.pos += 1;
                out
            } else {
                // get some more data
                match self.inner.next() {
                    Some(arr) => {
                        // TODO better way of copying array? mem::copy_nonoverlapping?
                        let arr = MaybeUninit::new(arr);
                        ptr::copy_nonoverlapping(
                            &arr as *const _ as *const [MaybeUninit<T>; N],
                            &mut self.buf as *mut [MaybeUninit<T>; N],
                            1,
                        );
                        let out = Some(self.buf[0].assume_init_read());
                        self.pos = 1;
                        out
                    }
                    None => None,
                }
            }
        }
    }
}

impl<T, I, const N: usize> core::iter::FusedIterator for FlattenArray<T, I, { N }> where
    I: Iterator<Item = [T; N]> + core::iter::FusedIterator
{
}

// We need to implement drop because only we know how many of buf are live.
impl<T, I, const N: usize> Drop for FlattenArray<T, I, { N }> {
    fn drop(&mut self) {
        unsafe {
            for i in self.pos..N {
                self.buf[i].assume_init_read(); // take ownership
            }
        }
    }
}
