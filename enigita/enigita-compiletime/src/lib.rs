use anyhow::Error;
use image::{DynamicImage, GenericImageView, Pixel};
use std::{fmt::Write, path::Path};

type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct Image {
    inner: DynamicImage,
}

impl Image {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let inner = image::io::Reader::open(path)?;
        Ok(Image {
            inner: inner.decode()?,
        })
    }

    /// Get the inner dynamic image.
    ///
    /// You can then manipulate the image as desired.
    pub fn as_dynamic_image(&mut self) -> &mut DynamicImage {
        &mut self.inner
    }

    /// Write the image data as a byte slice.
    ///
    /// Always most significan bit first (big endian).
    ///
    ///  - The `color` parameter determines the color format used.
    pub fn to_bytes(&self, color: Color) -> Vec<u8> {
        match color {
            Color::Rgb888 => self.to_bytes_rgb888(),
            Color::Rgb565 => self.to_bytes_rgb565(),
        }
    }

    /// Output the image buffer data as a rust array
    pub fn to_rust_const(&self, name: impl AsRef<str>, color: Color) -> String {
        let data = self.to_bytes(color);
        let mut output = format!("const {}: [u8; {}] = [", name.as_ref(), data.len());
        let mut data = data.into_iter();
        if let Some(num) = data.next() {
            write!(output, "{}", num).unwrap();
        }
        for num in data {
            write!(output, ", {}", num).unwrap();
        }
        write!(output, "];").unwrap();
        output
    }

    fn to_bytes_rgb888(&self) -> Vec<u8> {
        let mut output = vec![];
        for (_x, _y, pixel) in self.inner.pixels() {
            // pixel is type Rgba<u8>, which makes things nice
            output.extend(&pixel.channels()[0..3]);
            // ignore alpha (unsupported)
        }
        output
    }

    fn to_bytes_rgb565(&self) -> Vec<u8> {
        let mut output = vec![];
        for (_x, _y, pixel) in self.inner.pixels() {
            // pixel is type Rgba<u8>, which makes things nice
            let chan = pixel.channels();
            let r = scale(chan[0], 31. / 255.) as u16; // 5 bit
            let g = scale(chan[1], 63. / 255.) as u16; // 6 bit
            let b = scale(chan[2], 31. / 255.) as u16; // 5 bit
            let val = r << (5 + 6) | g << 5 | b;
            output.extend(&val.to_be_bytes());
            // ignore alpha (unsupported)
        }
        output
    }
}

/// Scale a number in f64 space then round.
fn scale(input: u8, scale: f64) -> u8 {
    (input as f64 * scale) as u8
}

// I really tried to use embedded_graphics types, but they are hard to use for this use case.
pub enum Color {
    Rgb888,
    Rgb565,
    // TODO
}
