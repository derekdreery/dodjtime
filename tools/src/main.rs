mod font;

use crate::font::convert_font;
use image::{DynamicImage, GenericImageView, Pixel};
use qu::ick_use::*;
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt)]
enum Cmd {
    ConvertImage {
        /// The location of the original image
        #[structopt(parse(from_os_str))]
        src: PathBuf,
        /// The location to put the converted image data.
        #[structopt(parse(from_os_str))]
        dst: PathBuf,
        /// A size to resize to, (format 'nxn' e.g. '240x320').
        ///
        /// If not set, then the input image size will be used.
        #[structopt(long, short, parse(try_from_str))]
        size: Option<Size>,
    },
    /// Converts a font into the format we expect. Outputs rust code.
    ConvertFont(ConvertFont),
}

#[derive(StructOpt)]
struct ConvertFont {
    /// The location of the original png
    #[structopt(parse(from_os_str))]
    src: PathBuf,
    /// The location of the original extents
    ///
    /// The extents are the advances - nothing fancy here. One number per line
    #[structopt(parse(from_os_str))]
    extents_src: PathBuf,
    /// The location to put the converted font data.
    #[structopt(parse(from_os_str))]
    dst: PathBuf,
    /// The height of the font. Defaults to the height of the image.
    #[structopt(long)]
    height: Option<u32>,
    /// Print the given character. This is for testing.
    #[structopt(long)]
    print_char: Option<char>,
}

pub struct Size {
    width: u32,
    height: u32,
}

impl FromStr for Size {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^(\d+)x?(\d+)$").unwrap();
        let caps = re
            .captures(input)
            .ok_or(format_err!("input does not look like a size (like nxn)"))?;
        Ok(Size {
            width: caps.get(1).unwrap().as_str().parse()?,
            height: caps.get(2).unwrap().as_str().parse()?,
        })
    }
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    match opt.cmd {
        Cmd::ConvertImage { src, dst, size } => convert_image(src, dst, size)?,
        Cmd::ConvertFont(config) => convert_font(config)?,
    }
    Ok(())
}

fn convert_image(src: PathBuf, dst: PathBuf, size: Option<Size>) -> Result {
    let src = load_image(&src)?;

    if let Some(size) = size {
        if src.dimensions() != (size.width, size.height) {
            return Err(format_err!(
                "requested size ({}x{}) does not match source size ({}x{}) - resize or crop image first",
                size.width, size.height, src.dimensions().0, src.dimensions().1,
            ));
        }
    }

    let img_out = image_to_bytes_rgb565(&src);
    fs::write(&dst, &img_out)
        .context(format!("could not write dst image \"{}\"", dst.display()))?;

    Ok(())
}

fn image_to_bytes_rgb565(img: &DynamicImage) -> Vec<u8> {
    /// Scale a number in f64 space then round.
    fn scale(input: u8, scale: f64) -> u8 {
        (input as f64 * scale) as u8
    }
    // first 2 bytes are width and height (u8)
    let (width, height) = img.dimensions();
    assert!(
        width < 256 && height < 256,
        "only support images upto 256 (the screen is 240)"
    );
    let mut output = vec![width as u8, height as u8];
    for (_x, _y, pixel) in img.pixels() {
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

fn load_image(path: &Path) -> Result<DynamicImage> {
    #[inline]
    fn load_image_inner(path: &Path) -> Result<DynamicImage> {
        Ok(image::io::Reader::open(path)?.decode()?)
    }
    load_image_inner(path).context(format!("could not read src image \"{}\"", path.display()))
}
