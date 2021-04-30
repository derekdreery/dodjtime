use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};
use qu::ick_use::*;
use std::{
    fmt::Write,
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
        /// The name that should be used for the image in Rust.
        ///
        /// Don't use an invalid or reserved name. The name should be SCREAMING_SNAKE_CASE.
        name: String,
        /// A size to resize to, (format 'n,n' e.g. '240,320').
        ///
        /// If not set, then the input image size will be used.
        #[structopt(long, short, parse(try_from_str))]
        size: Option<Size>,
    },
}

pub struct Size {
    width: u32,
    height: u32,
}

impl FromStr for Size {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^(\d+),?(\d+)$").unwrap();
        let caps = re
            .captures(input)
            .ok_or(format_err!("input does look like a size (like n,n)"))?;
        Ok(Size {
            width: caps.get(1).unwrap().as_str().parse()?,
            height: caps.get(2).unwrap().as_str().parse()?,
        })
    }
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    match opt.cmd {
        Cmd::ConvertImage {
            src,
            dst,
            name,
            size,
        } => convert_image(src, dst, name, size)?,
    }
    Ok(())
}

fn convert_image(src: PathBuf, dst: PathBuf, name: String, size: Option<Size>) -> Result {
    fn load_image(path: &Path) -> Result<DynamicImage> {
        Ok(image::io::Reader::open(path)?.decode()?)
    }
    let mut src =
        load_image(&src).context(format!("could not read src image \"{}\"", src.display()))?;

    if let Some(size) = size {
        src = src.resize_exact(size.width, size.height, FilterType::Lanczos3);
    }

    let img = image_to_rust_const(&src, &name);
    fs::write(&dst, &img).context(format!("could not write dst image \"{}\"", dst.display()))?;

    Ok(())
}

/// Output the image buffer data as a rust array
fn image_to_rust_const(image: &DynamicImage, name: impl AsRef<str>) -> String {
    let data = image_to_bytes_rgb565(image);
    let (width, height) = image.dimensions();
    let mut output = format!(
        "// Image size {}x{}\nconst {}: [u8; {}] = [",
        width,
        height,
        name.as_ref(),
        data.len()
    );
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

fn image_to_bytes_rgb565(img: &DynamicImage) -> Vec<u8> {
    /// Scale a number in f64 space then round.
    fn scale(input: u8, scale: f64) -> u8 {
        (input as f64 * scale) as u8
    }
    let mut output = vec![];
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
