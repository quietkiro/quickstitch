//! This module is for all methods involved in getting selected images loaded into memory.

use anyhow::anyhow;
use image::{
    image_dimensions, imageops::FilterType::Lanczos3, GenericImage, ImageReader, RgbImage,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fs::read_dir, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageLoaderError {
    #[error("No images were found in the selected directory")]
    NoImagesInDirectory,
}

/// Finds all `.jpg`, `.jpeg`, `.png` and `.webp` images within a directory.
///
/// Throws an error if:
///  - The directory is invalid or does not contain any images.
///  - The directory does not contain any jpg, jpeg, png, or webp images.
fn find_images(directory_path: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut images: Vec<_> = read_dir(directory_path)?
        .into_iter()
        .map(|file| file.unwrap().path())
        .filter(|path| match path.extension() {
            Some(os_str) => match os_str.to_str() {
                Some("jpg" | "webp" | "jpeg" | "png") => true,
                _ => false,
            },
            _ => false,
        })
        .collect();
    if images.is_empty() {
        return Err(ImageLoaderError::NoImagesInDirectory.into());
    }
    images.sort_by(|a, b| {
        natord::compare(
            a.file_name().unwrap().to_str().unwrap(),
            b.file_name().unwrap().to_str().unwrap(),
        )
    });
    Ok(images)
}

/// Loads any jpg, jpeg, png or webp images in the given directory into memory and resizing all images to a set width.
///
/// If the `width` parameter is set to `None`, the width of the image with the smallest width will be used.
/// Otherwise, the given width will be used.
///
/// Throws an error if:
///  - The directory is invalid or does not contain any images.
///  - The directory does not contain any jpg, jpeg, png, or webp images.
///  - An image cannot be opened.
pub fn load_images(directory_path: &str, width: Option<u32>) -> anyhow::Result<RgbImage> {
    let width = match width {
        Some(v) => v,
        None => {
            let dimensions = find_images(directory_path)?
                .into_iter()
                .map(|image| image_dimensions(image).map_err(|e| anyhow!(e)))
                .collect::<anyhow::Result<Vec<(u32, u32)>>>()?;
            // find_images will already throw an error if the directory does not contain any images, so unwrap is safe here.
            dimensions.iter().map(|pair| pair.0).min().unwrap()
        }
    };
    let images: Vec<RgbImage> = find_images(directory_path)?
        .into_par_iter()
        .map(|image_path| {
            Ok(ImageReader::open(image_path)?
                .decode()
                .map_err(|e| anyhow!(e))?
                .resize(width, u32::MAX, Lanczos3)
                .into())
        })
        .collect::<anyhow::Result<Vec<RgbImage>>>()?;
    let mut combined_image = RgbImage::new(width, images.iter().map(|image| image.height()).sum());
    let mut height_cursor = 0;
    for i in images {
        // This should never throw an error because the combined image height is set to the sum of all image heights.
        combined_image.copy_from(&i, 0, height_cursor)?;
        height_cursor += i.height();
    }
    Ok(combined_image)
}
