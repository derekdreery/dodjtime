use qu::ick_use::*;
use std::path::PathBuf;

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
    },
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    match opt.cmd {
        Cmd::ConvertImage { src, dst, name } => convert_image(),
    }
    Ok(())
}

fn convert_image() {}
