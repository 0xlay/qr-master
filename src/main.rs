#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

mod config;
mod generator;

use clap::Parser;
use config::{Args, ImageType};
use generator::{BmpQrCode, JpgQrCode, PngQrCode};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = Args::parse();

    match arg.image_type {
        ImageType::Png { data, path } => {
            generator::generate_qrcode(PngQrCode::new(), data.as_str(), path.as_path())?;
        }
        ImageType::Jpg { data, path } => {
            generator::generate_qrcode(JpgQrCode::new(), data.as_str(), path.as_path())?;
        }
        ImageType::Bmp { data, path } => {
            generator::generate_qrcode(BmpQrCode::new(), data.as_str(), path.as_path())?;
        }
    }

    Ok(())
}
