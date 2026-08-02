use crate::args::ImageSizeChoice;
use anyhow::anyhow;
use pdfium_render::prelude::*;
use std::path::PathBuf;
use tracing::debug;

pub fn convert_pdf2pngs(
    file_path: &PathBuf,
    page_tmp_path: &PathBuf,
    image_resolution: &ImageSizeChoice,
) -> anyhow::Result<Vec<PathBuf>> {
    let pdfium = Pdfium::new(Pdfium::bind_to_library("./libpdfium.dylib")?);
    let document = pdfium
        .load_pdf_from_file(&file_path, None)
        .map_err(|e| match e {
            PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::FormatError) => {
                anyhow!("File {:?} is not a pdf file", file_path)
            }
            err => anyhow!(err),
        })?;

    let pdf_stem = file_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .ok_or(anyhow!("Could not built file stem from filename "))?;

    let resolution = image_resolution.image_size();

    let pdf_render_config = PdfRenderConfig::new()
        .set_format(PdfBitmapFormat::BGRA)
        .set_target_width(resolution.x)
        .set_maximum_height(resolution.y);

    let mut pngs: Vec<PathBuf> = vec![];

    for (index, page) in document.pages().iter().enumerate() {
        let image = page.render_with_config(&pdf_render_config)?;
        let pdf_page = format!("{}-{}.png", pdf_stem, index.to_string());
        let page_tmp_path = page_tmp_path.join(pdf_page);
        debug!("Converting page {} to png", index);
        image
            .as_image()?
            .into_rgba8()
            .save_with_format(&page_tmp_path, image::ImageFormat::Png)?;

        pngs.push(page_tmp_path);
    }

    Ok(pngs)
}
