use core::{convert::TryInto, ops::Range};
use defmt::{unwrap, Format};
use embedded_graphics::geometry::Size;

// TODO use try_into rather than `as` after defmt 3 comes out (for defmt::Format on error types).

/// My own compact font format.
///
/// 2 parts: ascii -> index mapping, then font data. Both parts are prefixed with a 4 byte number
/// denoting their size.
pub struct Font {
    height: u32,
    palette: [u16; 3],
    keys: [u32; 128],
    pixels: &'static [u8],
}

#[derive(Format, Copy, Clone)]
pub struct Extents {
    /// Start of letter in buffer, *not* width offset.
    offset: usize,
    /// Number of pixels in the letter.
    len_in_pixels: usize,
}

pub enum Color {
    Opaque(u16),
    Transparent,
}

impl Extents {
    pub fn offset(self) -> usize {
        self.offset
    }

    pub fn len_in_pixels(self) -> usize {
        self.len_in_pixels
    }

    /// Number of bytes used (number of pixels / 4, rounded up)
    pub fn len_in_bytes(self) -> usize {
        (self.len_in_pixels + 3) / 4
    }

    pub fn width(self, font: &Font) -> usize {
        debug_assert!(self.len_in_pixels % font.height as usize == 0);
        self.len_in_pixels / font.height as usize
    }

    fn scaled_len_in_pixels(self, scale: usize) -> usize {
        self.len_in_pixels * scale * scale
    }

    fn buf_range(self) -> Range<usize> {
        self.offset..(self.offset + self.len_in_bytes())
    }
}

impl Font {
    /// (width, height)
    pub fn extents(&self, ch: u8) -> Option<Extents> {
        defmt::info!("{} {}", usize::from(ch), self.keys.get(usize::from(ch)));
        if ch >= 128 {
            return None;
        }
        let offset = *unwrap!(self.keys.get(usize::from(ch)));
        if offset == u32::MAX {
            return None;
        }
        let offset = offset as usize;
        let len_in_pixels = u32::from_be_bytes(slice_to_array(unwrap!(self
            .pixels
            .get(offset..offset + 4)))) as usize;
        Some(Extents {
            offset: offset + 4,
            len_in_pixels,
        })
    }

    /// Takes a character, and returns the pixels to go on the screen (ST7789).
    pub fn pixels<'a>(&'a self, extents: Extents, scale: u8) -> impl Iterator<Item = Color> + 'a {
        Pixels::new(self, extents, scale.into())
    }

    pub fn extents_size(&self, extents: Extents, scale: u8) -> Size {
        let height = self.height as usize;
        let scale = usize::from(scale);
        Size {
            width: ((extents.len_in_pixels / height) * scale) as u32,
            height: (height * scale) as u32,
        }
    }
}

struct Pixels<'a> {
    font: &'a Font,
    extents: Extents,
    scale: usize,
    /// Which pixel are we outputting
    idx: usize,
}

impl<'a> Pixels<'a> {
    fn new(font: &'a Font, extents: Extents, scale: u8) -> Self {
        Pixels {
            font,
            extents,
            scale: usize::from(scale),
            idx: 0,
        }
    }
}

impl<'a> Iterator for Pixels<'a> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        let scaled_len = self.extents.scaled_len_in_pixels(self.scale);
        if self.idx >= scaled_len {
            return None;
        }

        let buf = &unwrap!(self.font.pixels.get(self.extents.offset..));

        // Convert scaled co-ords to unscaled
        let width = self.extents.width(self.font);
        let scaled_width = width * self.scale;
        let (scaled_x, scaled_y) = (self.idx % scaled_width, self.idx / scaled_width);
        let (x, y) = (scaled_x / self.scale, scaled_y / self.scale);
        let idx = y * width + x;

        let byte = defmt::unwrap!(buf.get(idx / 4));
        let color_idx = match idx % 4 {
            3 => byte & 0b11,
            2 => (byte >> 2) & 0b11,
            1 => (byte >> 4) & 0b11,
            0 => (byte >> 6) & 0b11,
            _ => defmt::panic!(),
        };
        self.idx += 1;
        let color_idx = usize::from(color_idx);
        Some(if color_idx == 0 {
            Color::Transparent
        } else {
            Color::Opaque(self.font.palette[color_idx - 1])
        })
    }
}

pub static FONT: Font = include!("../../data/fonts/build/font.rs");

fn slice_to_array(s: &[u8]) -> [u8; 4] {
    match s.try_into() {
        Ok(a) => a,
        Err(_) => defmt::unreachable!(),
    }
}
