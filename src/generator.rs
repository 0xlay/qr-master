use image::{ImageFormat, Luma};
use qrcode::QrCode as QrEncoder;
use std::error::Error;
use std::path::Path;

pub trait QrCode {
    fn add_data(&mut self, data: &str);
    fn generate(&mut self, path: &Path) -> Result<(), Box<dyn Error>>;
}

pub struct PngQrCode {
    data: String,
}

impl PngQrCode {
    pub const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl QrCode for PngQrCode {
    fn add_data(&mut self, data: &str) {
        self.data.push_str(data);
    }

    fn generate(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        QrEncoder::new(self.data.as_bytes())?
            .render::<Luma<u8>>()
            .build()
            .save_with_format(path, ImageFormat::Png)?;
        self.data.clear();
        Ok(())
    }
}

pub struct JpgQrCode {
    data: String,
}

impl JpgQrCode {
    pub const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl QrCode for JpgQrCode {
    fn add_data(&mut self, data: &str) {
        self.data.push_str(data);
    }

    fn generate(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        QrEncoder::new(self.data.as_bytes())?
            .render::<Luma<u8>>()
            .build()
            .save_with_format(path, ImageFormat::Jpeg)?;
        self.data.clear();
        Ok(())
    }
}

pub struct BmpQrCode {
    data: String,
}

impl BmpQrCode {
    pub const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl QrCode for BmpQrCode {
    fn add_data(&mut self, data: &str) {
        self.data.push_str(data);
    }

    fn generate(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        QrEncoder::new(self.data.as_bytes())?
            .render::<Luma<u8>>()
            .build()
            .save_with_format(path, ImageFormat::Bmp)?;
        self.data.clear();
        Ok(())
    }
}

pub fn generate_qrcode<T: QrCode>(gen: T, data: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut gen = gen;
    gen.add_data(data);
    gen.generate(path)
}
