use anyhow;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info};
use watermark_rs::args::WatermarkCli;
use watermark_rs::pdf2pngs::convert_pdf2pngs;
use watermark_rs::pngs2pdf::merge_images2pdf;
use watermark_rs::watermark::add_watermark;
use watermark_rs::{create_tmp_workspace, file_with_suffix, remove_all_files};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let result_tmp_dir = create_tmp_workspace();

    let watermark = WatermarkCli::parse();

    info!("Using resolution: {:?}", watermark.resolution);

    match result_tmp_dir {
        Ok(page_tmp_path) => {
            debug!("Temporary workspace: {:?}", page_tmp_path);
            let final_out = file_with_suffix(&watermark.file, "watermarked")?;
            info!("1)Converting pdf to pngs ... ");
            let pngs = convert_pdf2pngs(&watermark.file, &page_tmp_path, &watermark.resolution)?;
            info!("> OK");
            info!("2)Adding watermarks to pngs ... ");
            let images_watermarked = add_watermark_to_pages(&watermark, &pngs)?;
            info!("> OK");
            info!("3)Merging watermarked pngs to pdf ... ");
            let _ = merge_images2pdf(&final_out, &images_watermarked)?;
            info!("> OK");
            info!("Result = {:?}", &final_out);
            info!("Cleaning up ...");
            let _ = clean_up(&page_tmp_path);
            info!("> OK");
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

pub fn add_watermark_to_pages(
    watermark: &WatermarkCli,
    pngs: &[PathBuf],
) -> anyhow::Result<Vec<PathBuf>> {
    pngs.iter()
        .map(|page| add_watermark(watermark, page))
        .collect::<anyhow::Result<Vec<PathBuf>>>()
}

pub fn clean_up(page_tmp_path: &PathBuf) -> anyhow::Result<()> {
    remove_all_files(&page_tmp_path)?;
    fs::remove_dir(&page_tmp_path).map_err(|err| err.into())
}
