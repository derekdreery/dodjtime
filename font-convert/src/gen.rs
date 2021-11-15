use ab_glyph::{Font, PxScaleFont, ScaleFont};
use qu::ick_use::*;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    convert::{TryFrom, TryInto},
    fmt::{self, Write},
    fs, io,
    path::Path,
};

// Parts of font:
//  - Max ascender (pixels)
//  - max descender
//  - glyphs (offset_x, offset_y, advance_x, size_x, size_y, pointer) - end of character is defined by size_x and size_y
//  - pixels
// TODO no kerning table for now
struct FontGen {
    // all present glyphs (only ascii supported)
    glyphs: HashMap<u8, Glyph>,
}

struct Glyph {
    offset: Point<i8>,
    size: Point<i8>,
    advance: i8,
    // on or off for each pixel
    pixels: Vec<bool>,
}

struct Point<T> {
    x: T,
    y: T,
}

impl FontGen {
    fn from(font: &PxScaleFont<impl Font>) -> Result<Self> {
        let mut gen = FontGen {
            glyphs: HashMap::new(),
        };
        for ch in 10..128u8 {
            let ch = char::from(ch);
            let glyph = font.scaled_glyph(ch);
            let glyph = match font.outline_glyph(glyph) {
                Some(g) => g,
                None => continue,
            };
            let bounds = glyph.px_bounds();
            let (width, height) = (bounds.width() as i32, bounds.height() as i32);
            let (width, height) = (
                i8::try_from(width).with_context(|| format!("character {:?} width too big", ch))?,
                i8::try_from(height)
                    .with_context(|| format!("character {:?} height too big", ch))?,
            );
            if width == 0 || height == 0 {
                // skip
                continue;
            }
            let (min_x, min_y) = (bounds.min.x as i32, bounds.min.y as i32);
            let (min_x, min_y) = (
                i8::try_from(min_x).with_context(|| format!("character {:?} min_x too big", ch))?,
                i8::try_from(min_y).with_context(|| format!("character {:?} min_y too big", ch))?,
            );
            let advance = font.h_advance(glyph.glyph().id);

            let glyph = Glyph {
                offset: Point { x: min_x, y: min_y },
                size: Point {
                    x: width,
                    y: height,
                },
            };
        }
        Ok(gen)
    }

    // a string of source code ready to go into a file.
    fn gen(&self) -> String {
        "".into()
    }

    fn max_ascender(&self) -> i16 {
        let mut max = i16::MIN;
        for (_, glyph) in &self.glyphs {
            let ascender = glyph.offset.y.into();
            if ascender > max {
                max = ascender;
            }
        }
        max
    }

    fn max_descender(&self) -> i16 {
        let mut min = i16::MAX;
        for (_, glyph) in &self.glyphs {
            // we need to negate the size, because smaller number is bigger descender
            let descender = i16::from(glyph.offset.y) - i16::from(glyph.size.y);
            if descender < min {
                min = descender;
            }
        }
        min
    }
}
