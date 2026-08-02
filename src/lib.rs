pub mod args;
pub mod pdf2pngs;
pub mod pngs2pdf;
pub mod watermark;

use anyhow::anyhow;
use core::convert::AsRef;
use rand::RngExt;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_tmp_workspace() -> anyhow::Result<PathBuf> {
    let doc_temp_work_folder = random_string(6);
    let page_tmp_path = env::temp_dir()
        .join("watermark-rs")
        .join(doc_temp_work_folder);

    fs::create_dir_all(&page_tmp_path)?;
    Ok(page_tmp_path)
}

pub fn file_with_suffix(filename: impl AsRef<Path>, suffixe: &str) -> anyhow::Result<PathBuf> {
    let path = filename.as_ref();

    let stem = path
        .file_stem()
        .ok_or(anyhow!("Could not built file stem from filename "))?
        .to_string_lossy();

    let ext = path.extension().and_then(|e| e.to_str());

    let mut new_name = format!("{stem}-{suffixe}");
    if let Some(ext) = ext {
        new_name.push('.');
        new_name.push_str(ext);
    }

    Ok(path.with_file_name(new_name))
}

pub fn remove_all_files(dir: impl AsRef<Path>) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn random_string(len: usize) -> String {
    let mut rng = rand::rng();

    (0..len)
        .map(|_| {
            let c = rng.random_range(b'a'..=b'z') as char;
            c
        })
        .collect()
}
