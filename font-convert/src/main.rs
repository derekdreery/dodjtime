use qu::ick_use::*;
use rusttype::Font;
use std::{array, convert::TryFrom, fs, path::PathBuf};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    font_file: PathBuf,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    let font_data = fs::read(opt.font_file)?;
    let font = Font::try_from_vec(font_data).context("could not parse font file")?;
    let scale = rusttype::Scale::uniform(font.scale_for_pixel_height(12.));
    let origin = rusttype::Point { x: 0., y: 0. };
    println!("{:?}", font.v_metrics_unscaled());
    let mut buf: Vec<u8> = Vec::new();
    for ch in
        r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789:;@'~#<>,.?/\|!""#.chars()
    {
        let glyph = font.glyph(ch).scaled(scale).positioned(origin);
        let size = glyph
            .pixel_bounding_box()
            .context("could not get pixel bounding box")?;
        ensure!(size.width() > 0, "character {} width not > 0", ch);
        ensure!(size.height() > 0, "character {} height not > 0", ch);
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
    }
    Ok(())
}
