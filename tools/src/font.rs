use crate::{load_image, ConvertFont, Result};
use image::{GenericImageView, Pixel, Rgba, RgbaImage};
use qu::ick_use::*;
use std::{
    collections::{BTreeMap, HashSet},
    convert::{TryFrom, TryInto},
    fmt::{self, Write},
    fs,
    path::Path,
};

// font pixel format b00 is always transparent, b01, b10, b11 are in a lookup table.
// This means 2 bits per pixel, or 4 pixels per byte.

pub(crate) fn convert_font(config: ConvertFont) -> Result {
    // load source image.
    let font_img = SourceImage::load(&config.src, config.height)?;

    // Load extents
    let extents = Extents::load(&config.extents_src)?;
    log::info!("font extents: {:?}", extents);

    let mut pixels = Vec::new();
    let mut keys = [u32::MAX; 128];
    for (ch, extent) in extents {
        // Save the offset
        keys[usize::from(ch)] = pixels.len().try_into().expect("usize -> u32 overflow");
        // append the pixels
        font_img.to_pixels(&mut pixels, extent.offset, extent.width);
    }

    let mut palette = [0; 3];
    for (idx, color) in font_img.palette.iter().enumerate() {
        palette[idx] = color_to_u16(*color);
    }

    let font_gen = FontGen {
        height: font_img.height,
        palette,
        keys,
        pixels,
    };

    if let Some(ch) = config.print_char {
        font_gen.print_ch(ch as u8);
    }

    fs::write(&config.dst, &font_gen.gen())?;

    Ok(())
}

/// A structure that can be serialized as a font into rust source code
struct FontGen {
    /// Height of font (no concept of baseline etc.).
    height: u32,
    /// Colors for drawing idx 1, 2, 3 (0 is transparent).
    palette: [u16; 3],
    /// a key's index is its ascii value. Value is the offset of the start of the pixels for the
    /// character. u32::MAX means key not present.
    keys: [u32; 128],
    /// blob of all pixels. Use `keys` to get data for specific pixel. First 4 bytes are length
    /// (big endian).
    pixels: Vec<u8>,
}

impl FontGen {
    fn gen(&self) -> String {
        let mut out = format!(
            "Font {{\n\
            height: {},\n\
            palette: [{}, {}, {}],\n\
            keys: [{}",
            self.height, self.palette[0], self.palette[1], self.palette[2], self.keys[0]
        );

        for i in 1..128 {
            write!(out, ", {}", self.keys[i]).unwrap();
        }
        let mut i = self.pixels.iter();
        write!(out, "],\npixels: &[{}", i.next().unwrap()).unwrap();
        for p in i {
            write!(out, ", {}", p).unwrap();
        }
        write!(out, "]\n}}").unwrap();
        out
    }

    /// For testing
    fn print_ch(&self, ch: u8) {
        fn val_at(buf: &[u8], pos: usize) -> u8 {
            let byte = buf[pos / 4];
            match pos % 4 {
                3 => byte & 0b11,
                2 => (byte >> 2) & 0b11,
                1 => (byte >> 4) & 0b11,
                0 => (byte >> 6) & 0b11,
                _ => panic!(),
            }
        }

        let offset = self.keys[ch as usize];
        if offset == u32::MAX {
            panic!("{} not supported", ch as char);
        }
        let offset = offset as usize;
        let len = u32::from_be_bytes(self.pixels[offset..offset + 4].try_into().unwrap()) as usize;
        let pixels = &self.pixels[offset + 4..offset + 4 + (len + 3) / 4];
        let width = len / usize::try_from(self.height).unwrap();
        for idx in 0..len {
            match val_at(pixels, idx) {
                0 => print!(" "),
                1 => print!("█"),
                2 => print!("▒"),
                _ => panic!(),
            }
            if (idx + 1) % width == 0 {
                println!();
            }
        }
    }
}

struct SourceImage {
    img: RgbaImage,
    height: u32,
    palette: Vec<Rgba<u8>>,
}

impl SourceImage {
    fn load(path: &Path, height_overwrite: Option<u32>) -> Result<Self> {
        let mut img = load_image(&path)?.to_rgba8();
        let (width, height) = img.dimensions();
        let mut colors = HashSet::new();
        for pixel in img.pixels_mut() {
            if pixel[3] == 0 {
                // normalize transparent
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            } else {
                if pixel[3] != 255 {
                    log::warn!("partial transparency not supported -> setting to opaque");
                    pixel[3] = 255;
                }
                colors.insert(pixel.to_rgba());
            }
        }
        if colors.len() > 3 {
            return Err(format_err!("only up to 3 colors (+ transparent) supported"));
        }

        log::info!(
            "loaded font image (dims {}x{}), colors {:?}",
            width,
            height,
            colors
        );
        // The user may want to discard some of the image.
        let height = height_overwrite.unwrap_or(height);
        // convert colors to vec, to implicitly give each color a key (its index)
        Ok(SourceImage {
            img,
            height,
            palette: colors.iter().copied().collect(),
        })
    }

    fn to_pixels(&self, px: &mut Vec<u8>, x: u32, width: u32) {
        let old_len = px.len();
        // First write length of char (big-endian). This is the len in pixels: byte len is
        // (len + 3) / 4. +3 to round up.
        let len = width * self.height;
        px.extend_from_slice(&len.to_be_bytes());

        // Then write pixel data. 4 pixels per byte. Last byte extended with 0.
        let mut byte_idx = 0;
        let mut byte = 0; // accumulator
        for (_x, _y, pixel) in self.img.view(x, 0, width, self.height).pixels() {
            //log::debug!("handling pixel {}x{}", x, y);
            if byte_idx == 4 {
                byte_idx = 0;
                px.push(byte);
                byte = 0;
            }

            byte = (byte << 2) | self.color_idx(pixel);

            byte_idx += 1;
        }
        if byte_idx != 0 {
            px.push(byte);
        }

        // sanity check
        assert!(
            old_len + 4 + (usize::try_from(len).unwrap() + 3) / 4 == px.len(),
            "{} + 4 + ({} + 3) / 4 == {}",
            old_len,
            len,
            px.len()
        );
    }

    fn color_idx(&self, color: Rgba<u8>) -> u8 {
        if color.0[3] == 0 {
            0
        } else {
            let idx: u8 = self
                .palette
                .iter()
                .position(|itm| *itm == color)
                .expect("color not in palette")
                .try_into()
                .expect("idx not fit in u8");
            assert!(idx < 3);
            idx + 1
        }
    }
}

struct Extents(BTreeMap<u8, Extent>);

impl Extents {
    fn load(path: &Path) -> Result<Self> {
        fn skip_ws(mut i: &str) -> &str {
            loop {
                match i.chars().next() {
                    Some(s) if s.is_whitespace() => i = &i[s.len_utf8()..],
                    _ => break,
                }
            }
            i
        }
        fn parse_colon(i: &str) -> Result<&str> {
            match i.chars().next() {
                Some(':') => Ok(&i[1..]),
                Some(o) => Err(format_err!("expected ':', found {}", o)),
                None => Err(format_err!("expected ':', found EOL")),
            }
        }
        fn parse_ascii(i: &str) -> Result<(u8, &str)> {
            match i.chars().next() {
                Some(o) if o.is_ascii() => Ok((o as u8, &i[1..])),
                Some(o) => Err(format_err!("expected ascii char, found {}", o)),
                None => Err(format_err!("expected ascii char, found EOL")),
            }
        }
        fn parse_u32(i: &str) -> Result<(u32, &str)> {
            let mut chrs = i.char_indices().peekable();
            if !matches!(chrs.peek(), Some((_, ch)) if ch.is_ascii_digit()) {
                return Err(format_err!("expected ascii number"));
            }
            while matches!(chrs.peek(), Some((_, ch)) if ch.is_ascii_digit()) {
                chrs.next();
            }
            let rest_idx = chrs.next().map(|(idx, _)| idx);
            match rest_idx {
                Some(idx) => {
                    let output = (&i[..idx]).parse::<u32>().context("parsing u32")?;
                    Ok((output, &i[idx..]))
                }
                None => {
                    let output = i.parse::<u32>().context("parsing u32")?;
                    Ok((output, ""))
                }
            }
        }
        let raw = fs::read_to_string(path)?;
        let mut extents = BTreeMap::new();
        for line in raw.lines() {
            let (byte, line) = parse_ascii(line)?;
            let line = skip_ws(line);
            let line = parse_colon(line)?;
            let line = skip_ws(line);
            let (offset, line) = parse_u32(line)?;
            let line = skip_ws(line);
            let (width, _line) = parse_u32(line)?;
            // assert!(line.is_empty());
            extents.insert(byte, Extent { offset, width });
        }
        Ok(Extents(extents))
    }
}

impl IntoIterator for Extents {
    type IntoIter = std::collections::btree_map::IntoIter<u8, Extent>;
    type Item = (u8, Extent);
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for Extents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fields = self.0.iter().map(|(k, v)| (*k as char, v));
        f.write_str("Extents ")?;
        f.debug_map().entries(fields).finish()
    }
}

struct Extent {
    /// width offset from start of image.
    offset: u32,
    width: u32,
}

impl fmt::Debug for Extent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-", self.offset, self.width)
    }
}

fn color_to_u16(color: Rgba<u8>) -> u16 {
    fn scale(input: u8, scale: f64) -> u8 {
        (input as f64 * scale) as u8
    }
    // We ignore the alpha channel.
    let chan = color.channels();
    let r = scale(chan[0], 31. / 255.) as u16; // 5 bit
    let g = scale(chan[1], 63. / 255.) as u16; // 6 bit
    let b = scale(chan[2], 31. / 255.) as u16; // 5 bit
    r << (5 + 6) | g << 5 | b
}
