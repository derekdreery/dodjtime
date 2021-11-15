use ab_glyph::{Font, FontArc, PxScale, PxScaleFont, ScaleFont};
use qu::ick_use::*;
use std::{fmt, fs, path::PathBuf};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    font_file: PathBuf,
    point_size: f32,
}

struct DisplayFont<'a, F>(&'a F);

impl<'a, F: Font> fmt::Display for DisplayFont<'a, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Font metrics:")?;
        match self.0.units_per_em() {
            Some(em) => writeln!(f, "     units per em: {}", em),
            None => writeln!(f, "     units per em: none"),
        }?;
        writeln!(f, "  ascent unscaled: {}", self.0.ascent_unscaled())?;
        writeln!(f, " descent unscaled: {}", self.0.descent_unscaled())?;
        writeln!(f, "line gap unscaled: {}", self.0.line_gap_unscaled())?;
        Ok(())
    }
}

struct DisplayFontScaled<'a, F>(&'a PxScaleFont<F>);

impl<'a, F> fmt::Display for DisplayFontScaled<'a, F>
where
    F: Font,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Font metrics:")?;
        writeln!(f, "  ascent: {}", self.0.ascent())?;
        writeln!(f, " descent: {}", self.0.descent())?;
        writeln!(f, "line gap: {}", self.0.line_gap())?;
        Ok(())
    }
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    let font_data = fs::read(opt.font_file).context("opening font file")?;
    let font = FontArc::try_from_vec(font_data).context("could not parse font file")?;
    log::info!("{}", DisplayFont(&font));

    let scale = pt_size_to_px_scale(&font, opt.point_size, 1.);
    log::info!("calculated scale: {:?}", scale);
    let font = font.into_scaled(scale);
    log::info!("{}", DisplayFontScaled(&font));

    let mut buf: Vec<u8> = Vec::new();
    for ch in
        r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789:;@'~#<>,.?/\|!""#.chars()
    {
        let glyph = font.scaled_glyph(ch);
        let glyph = font
            .outline_glyph(glyph)
            .ok_or_else(|| format_err!("getting outline for \"{}\"", ch))?;

        // collect image data
        let bounds = glyph.px_bounds();
        let height = bounds.height() as usize;
        ensure!(height > 0, "character {} height not > 0", ch);
        let width = bounds.width() as usize;
        ensure!(width > 0, "character {} height not > 0", ch);

        log::info!(
            "for char {} bounds ({}, {}) -> ({}, {}) size {}x{}",
            ch,
            bounds.min.x,
            bounds.min.y,
            bounds.max.x,
            bounds.max.y,
            width,
            height
        );

        // rasterize image
        let mut data = vec![0.; width * height];
        // map to the vec
        let m = |x, y| {
            let x = x as usize;
            let y = y as usize;
            y * width + x
        };
        glyph.draw(|x, y, amt| {
            let offset = m(x, y);
            data[offset] = amt;
        });

        debug_render(&data, width);
        // offset from baseline to draw glyph.
        let origin = bounds.min;
        /*
        buf.extend(array::IntoIter::new(
            u16::try_from(size.width())
                .context("converting width")?
                .to_be_bytes(),
        ));
        buf.extend(array::IntoIter::new(
            u16::try_from(size.height())
                .context("converting height")?
                .to_be_bytes(),
        ));
        let area = size.width() as usize * size.height() as usize;
        // one bit per pixel.
        println!("letter {}", ch);
        let mut acc = 0;

        glyph.draw(|x, y, amt| {
            println!("  ({},{}) -> {}", x, y, amt);
            acc += 1;
        });
        */
    }
    Ok(())
}

// convert size in points to scale to use.
fn pt_size_to_px_scale<F: Font>(font: &F, pt_size: f32, screen_scale_factor: f32) -> PxScale {
    let px_per_em = pt_size * screen_scale_factor * (96.0 / 72.0);
    let units_per_em = font.units_per_em().unwrap();
    let height = font.height_unscaled();
    PxScale::from(px_per_em * height / units_per_em)
}

/// draw the glyph in `data`.
fn debug_render(data: &[f32], width: usize) {
    for (idx, amt) in data.iter().copied().enumerate() {
        let ch = if amt < 0.2 {
            ' '
        } else if amt < 0.4 {
            '░'
        } else if amt < 0.6 {
            '▒'
        } else if amt < 0.8 {
            '▓'
        } else {
            '█'
        };

        print!("{}", ch);
        if (idx + 1) % width == 0 {
            println!();
        }
    }
    println!();
}
