use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command[subcommand]]
    pub image_type: ImageType,
}

#[derive(Subcommand, Debug)]
pub enum ImageType {
    #[command(about = "Generates a QR code in png format", long_about = None)]
    Png {
        #[arg(short, long)]
        data: String,
        #[arg(short, long)]
        path: std::path::PathBuf,
    },

    #[command(about = "Generates a QR code in jpg format", long_about = None)]
    Jpg {
        #[arg(short, long)]
        data: String,
        #[arg(short, long)]
        path: std::path::PathBuf,
    },

    #[command(about = "Generates a QR code in bmp format", long_about = None)]
    Bmp {
        #[arg(short, long)]
        data: String,
        #[arg(short, long)]
        path: std::path::PathBuf,
    },
}
