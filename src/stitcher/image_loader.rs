//! This module is for all methods involved in getting selected images loaded into memory.

use image::{
    error::ImageError, image_dimensions, imageops::FilterType::Lanczos3, GenericImage, ImageReader,
    RgbImage,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs::read_dir,
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors related to loading images.
pub enum ImageLoaderError {
    // IO Errors
    #[error("Could not find the provided file or directory")]
    NotFound,
    #[error("Insufficient permissions to access the provided file or directory")]
    PermissionDenied,

    // Logical Errors
    #[error("No images were found in the selected directory")]
    NoImagesInDirectory,
    #[error("Expected a directory")]
    ExpectedDirectory,

    // upstream errors
    #[error("{0}")]
    ImageError(ImageError),
    #[error("{0}")]
    IoError(io::Error),
}

impl From<ImageError> for ImageLoaderError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<io::Error> for ImageLoaderError {
    fn from(value: io::Error) -> Self {
        use io::ErrorKind as Kind;

        match value.kind() {
            Kind::NotFound => ImageLoaderError::NotFound,
            Kind::PermissionDenied => ImageLoaderError::PermissionDenied,
            // add more cases as required
            _ => ImageLoaderError::IoError(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Logical,
    Natural,
}

/// Finds all `.jpg`, `.jpeg`, `.png` and `.webp` images within a directory.
///
/// Throws an error if:
///  - The directory is invalid or does not contain any images.
///  - The directory does not contain any jpg, jpeg, png, or webp images.
pub fn find_images(
    directory_path: impl AsRef<Path>,
    sort: Sort,
) -> Result<Vec<PathBuf>, ImageLoaderError> {
    // create pathbuf, check if path is a directory
    let path = directory_path.as_ref();
    if !path.is_dir() {
        return Err(ImageLoaderError::ExpectedDirectory);
    }

    // get images
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

    // if no images were found
    if images.is_empty() {
        return Err(ImageLoaderError::NoImagesInDirectory);
    }

    match sort {
        Sort::Logical => images.sort(),
        Sort::Natural => images
            .sort_by(|a, b| natord::compare(&a.display().to_string(), &b.display().to_string())),
    }

    // return images
    Ok(images)
}

/// Loads the images at the provided paths into a single image strip.
///
/// If the `width` parameter is set to `None`, the width of the image with the smallest width will be used.
/// Otherwise, the given width will be used.
///
/// Parameters:
///  - paths: A slice containing paths to each individual input image.
///  - width: The width that the final stitched images will have.
///  - ignore_unloadable: Sometimes, there is an issue where an image has a duplicate
///                       and the duplicate has a filesize of 0. For cases like this,
///                       this setting exists to allow you to only load images that are
///                       able to be loaded.
///
/// Throws an error if:
///  - The directory is invalid or does not contain any images.
///  - The directory does not contain any jpg, jpeg, png, or webp images.
///  - An image cannot be opened.
pub fn load_images(
    paths: &[impl AsRef<Path>],
    width: Option<u32>,
    ignore_unloadable: bool,
) -> Result<RgbImage, ImageLoaderError> {
    // get a vec of path refs from the generic parameter
    let paths = paths.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>();

    let dimensions = paths
        .iter()
        .map(|&image| image_dimensions(image).map_err(|e| ImageLoaderError::from(e)));
    let dimensions: Vec<_> = if ignore_unloadable {
        dimensions.filter_map(|res| res.ok()).collect()
    } else {
        dimensions.collect::<Result<Vec<(u32, u32)>, ImageLoaderError>>()?
    };

    // the width to resize images to
    let width = match width {
        Some(v) => v,
        None => {
            // find_images will already throw an error if the directory does not contain any images, so unwrap is safe here.
            dimensions.iter().map(|pair| pair.0).min().unwrap()
        }
    };

    // the height to resize images to
    // let height = dimensions.iter().map(|pair| pair.1).max().unwrap();

    // load images
    let images = paths.par_iter().map(|&image_path| {
        let image = ImageReader::open(image_path)?
            .decode()
            .map_err(|e| ImageLoaderError::from(e))?;

        if image.width() == width {
            // noop if widths match
            Ok(image.into())
        } else {
            // resize image otherwise
            Ok(image.resize(width, u32::MAX, Lanczos3).into())
        }
    });
    let images: Vec<RgbImage> = if ignore_unloadable {
        images.filter_map(|res| res.ok()).collect::<Vec<_>>()
    } else {
        images.collect::<Result<Vec<RgbImage>, ImageLoaderError>>()?
    };

    // combine all images into one big strip
    let mut combined_image = RgbImage::new(width, images.iter().map(|image| image.height()).sum());
    let mut height_cursor = 0;

    for i in images {
        // This should never throw an error because the combined image height is set to the sum of all image heights.
        combined_image
            .copy_from(&i, 0, height_cursor)
            .expect("all according to keikaku");
        height_cursor += i.height();
    }

    Ok(combined_image)
}
