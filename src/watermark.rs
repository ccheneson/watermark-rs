use crate::args::WatermarkCli;
use crate::file_with_suffix;
use ab_glyph::{FontRef, PxScale};
use anyhow::Result;
use core::convert::AsRef;
use image::{Rgba, RgbaImage, open};
use imageproc::drawing::draw_text_mut;
use imageproc::geometric_transformations::{Interpolation, rotate_about_center};
use std::path::{Path, PathBuf};
use tracing::debug;

pub fn add_watermark(watermark: &WatermarkCli, filename: impl AsRef<Path>) -> Result<PathBuf> {
    let mut img = open(&filename)?.to_rgba8();

    let (w, h) = img.dimensions();

    debug!("Image dimension = {w}/{h}");

    let font = FontRef::try_from_slice(include_bytes!("../DejaVuSans.ttf"))?;

    let mut watermark_image = RgbaImage::from_pixel(w + 500, h, Rgba([0, 0, 0, 0]));

    let text = &watermark.text1;
    let text2 = if let Some(t) = &watermark.text2 {
        &t
    } else {
        text
    };

    let text_x_3 = [text.as_str(); 3].join(" - ");
    let text2_x_3 = [text2.as_str(); 3].join(" - ");

    let scale = PxScale { x: 80.0, y: 222.0 };

    let range = 1..5; // 4 lines of text
    let y_text_base = h / 5 as u32;

    debug!("Transparency set to {}", watermark.transparency);

    range.for_each(|i| {
        let text = if i % 2 == 0 { &text2_x_3 } else { &text_x_3 };
        draw_text_mut(
            &mut watermark_image,
            Rgba([255, 0, 0, watermark.transparency]), // red, transparent
            0 as i32,
            (y_text_base * i) as i32,
            scale,
            &font,
            &text,
        );
    });

    // Rotate the whole watermark layer
    let rotated = rotate_about_center(
        &watermark_image,
        (25_f32).to_degrees(),
        Interpolation::Bilinear,
        Rgba([0, 0, 0, 0]),
    );

    // Blend watermark onto image
    image::imageops::overlay(&mut img, &rotated, 0, 0);

    let output_file = file_with_suffix(&filename, "watermarked")?;

    img.save(&output_file)?;

    Ok(output_file)
}
