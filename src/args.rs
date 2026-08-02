use anyhow::anyhow;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ImageSizeChoice {
    Normal,
    High,
}

pub struct ImageSize {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for ImageSizeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ImageSizeChoice::Normal => write!(f, "normal"),
            ImageSizeChoice::High => write!(f, "high"),
        }
    }
}

impl ImageSize {
    pub const DPI_150: Self = ImageSize { x: 1240, y: 1754 };
    pub const DPI_300: Self = ImageSize { x: 2480, y: 3508 };
}

impl ImageSizeChoice {
    pub const fn image_size(self) -> ImageSize {
        match self {
            Self::Normal => ImageSize::DPI_150,
            Self::High => ImageSize::DPI_300,
        }
    }
}

fn non_empty(s: &str) -> anyhow::Result<String> {
    if s.trim().is_empty() {
        Err(anyhow!("text1 must not be empty"))
    } else {
        Ok(s.to_string())
    }
}

#[derive(Parser, Debug)]
pub struct WatermarkCli {
    ///Path to the document to add watermarks to
    pub file: PathBuf,

    #[arg(long, value_parser = non_empty, help="primary watermark to add")]
    pub text1: String,

    #[arg(
        long,
        help = "optional secondary watermark to add - if omitted, primary watermark is used"
    )]
    pub text2: Option<String>,

    #[arg(long, short, default_value_t = ImageSizeChoice::Normal, help="for smaller size, choose 'normal' (Default) - for good resolution, choose 'high'")]
    pub resolution: ImageSizeChoice,

    #[arg(long, short, default_value_t = 30)]
    pub transparency: u8,
}
