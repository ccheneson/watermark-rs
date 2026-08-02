use anyhow;
use image::{ColorType, GenericImageView, ImageFormat};
use miniz_oxide::deflate::{CompressionLevel, compress_to_vec_zlib};
use pdf_writer::{Content, Filter, Finish, Name, Pdf, Rect, Ref};
use std::path::PathBuf;
use tracing::debug;

const OFFSET: i32 = 100;

fn assign_ids(index: i32) -> (Ref, Ref, Ref, Ref) {
    (
        Ref::new(index + 1),
        Ref::new(index + 2),
        Ref::new(index + 3),
        Ref::new(index + 4),
    )
}

pub fn merge_images2pdf(
    output_final: &PathBuf,
    images_watermarked: &[PathBuf],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut pdf = Pdf::new();

    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);

    let mut pages_ids: Vec<Ref> = Vec::new();

    for (index, image_watermarked) in images_watermarked.iter().enumerate() {
        let (page_id, image_id, s_mask_id, content_id) = assign_ids((index as i32 + 1) * OFFSET);
        pages_ids.push(page_id);

        let name = format!("Im{}", index);
        let image_name = Name(name.as_bytes());

        let mut page = pdf.page(page_id);

        let dim = image::open(image_watermarked)?.dimensions();

        debug!(
            "Assigning page {:?} to id {:?} with dimension {}/{}",
            index, page_id, dim.0, dim.1
        );

        let a4 = Rect::new(0.0, 0.0, dim.0 as f32, dim.1 as f32);
        page.media_box(a4);
        page.parent(page_tree_id);
        page.contents(content_id);
        page.resources().x_objects().pair(image_name, image_id);
        page.finish();

        let data = std::fs::read(image_watermarked)?;

        let format = image::guess_format(&data)?;
        let dynamic = image::load_from_memory(&data)?;

        let (filter, encoded, mask) = match format {
            ImageFormat::Jpeg => {
                assert!(dynamic.color() == ColorType::Rgb8);
                (Filter::DctDecode, data, None)
            }

            ImageFormat::Png => {
                let level = CompressionLevel::DefaultLevel as u8;
                let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

                let mask = dynamic.color().has_alpha().then(|| {
                    let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                    compress_to_vec_zlib(&alphas, level)
                });

                (Filter::FlateDecode, encoded, mask)
            }

            _ => panic!("unsupported image format"),
        };

        let mut image = pdf.image_xobject(image_id, &encoded);
        image.filter(filter);
        image.width(dynamic.width() as i32);
        image.height(dynamic.height() as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask.is_some() {
            image.s_mask(s_mask_id);
        }
        image.finish();

        let w = dynamic.width() as f32;
        let h = dynamic.height() as f32;

        let x = (a4.x2 - w) / 2.0;
        let y = (a4.y2 - h) / 2.0;

        let mut content = Content::new();
        content.save_state();
        content.transform([w, 0.0, 0.0, h, x, y]);
        content.x_object(image_name);
        content.restore_state();

        pdf.stream(content_id, &content.finish());
    }

    let pages_count = pages_ids.len();
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id)
        .kids(pages_ids)
        .count(pages_count as i32);

    std::fs::write(output_final, pdf.finish())?;
    Ok(Vec::new())
}
