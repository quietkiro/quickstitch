//! This module is for all methods involved in getting selected images loaded into memory.

use image::{
    error::ImageError, image_dimensions, imageops::FilterType::Lanczos3, GenericImage, ImageReader,
    RgbImage,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    io,
    fs::read_dir,
    path::{Path, PathBuf},
    time::Instant,
};
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors related to loading images.
pub enum ImageLoaderError {
    // IO Errors
    #[error("Could not find a valid path at \"{0}\"")]
    NotFound(PathBuf),
    #[error("Permission denied while attempting to access \"{0}\"")]
    PermissionDenied(PathBuf),

    // Logical Errors
    #[error("No images were found in the selected directory")]
    NoImagesInDirectory,
    #[error("Expected a directory at \"{0}\"")]
    ExpectedDirectory(PathBuf),

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

impl ImageLoaderError {
    // Convert a [io::Error] into a [ImageLoaderError].
    fn from_io_error(err: std::io::Error, path: PathBuf) -> ImageLoaderError {
        use std::io::ErrorKind as Kind;
        match err.kind() {
            Kind::NotFound => ImageLoaderError::NotFound(path),
            Kind::PermissionDenied => ImageLoaderError::PermissionDenied(path),
            // add more cases as required
            _ => ImageLoaderError::IoError(err),
        }
    }
}

/// Finds all `.jpg`, `.jpeg`, `.png` and `.webp` images within a directory.
///
/// Throws an error if:
///  - The directory is invalid or does not contain any images.
///  - The directory does not contain any jpg, jpeg, png, or webp images.
pub fn find_images(directory_path: impl AsRef<Path>) -> Result<Vec<PathBuf>, ImageLoaderError> {
    // create pathbuf, check if path is a directory
    let path = PathBuf::from(directory_path.as_ref());
    if !path.is_dir() {
        return Err(ImageLoaderError::ExpectedDirectory(path));
    }

    // get images
    let mut images: Vec<_> = read_dir(directory_path)
        .map_err(|e| ImageLoaderError::from_io_error(e, path))?
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
        return Err(ImageLoaderError::NoImagesInDirectory.into());
    }

    // sort images by natural order
    images.sort_by(|a, b| natord::compare(&a.display().to_string(), &b.display().to_string()));

    // return images
    Ok(images)
}

/// Loads the images at the provided paths into a single image strip.
///
/// If the `width` parameter is set to `None`, the width of the image with the smallest width will be used.
/// Otherwise, the given width will be used.
///
/// Throws an error if:
///  - The directory is invalid or does not contain any images.
///  - The directory does not contain any jpg, jpeg, png, or webp images.
///  - An image cannot be opened.
pub fn load_images(paths: &[impl AsRef<Path>], width: Option<u32>) -> Result<RgbImage, ImageLoaderError> {
    // get a vec of path refs from the generic parameter
    let paths = paths.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>();

    let dimensions = paths
        .iter()
        .map(|&image| image_dimensions(image).map_err(|e| ImageLoaderError::from(e)))
        .collect::<Result<Vec<(u32, u32)>, ImageLoaderError>>()?;

    // the width to resize images to
    let width = match width {
        Some(v) => v,
        None => {
            // let dimensions = find_images(directory_path)?
            //     .into_iter()
            //     .map(|image| image_dimensions(image).map_err(|e| anyhow!(e)))
            //     .collect::<anyhow::Result<Vec<(u32, u32)>>>()?;
            // find_images will already throw an error if the directory does not contain any images, so unwrap is safe here.
            dimensions.iter().map(|pair| pair.0).min().unwrap()
        }
    };

    // the height to resize images to
    let height = dimensions.iter().map(|pair| pair.1).max().unwrap();

    // load images
    let images: Vec<RgbImage> = paths
        .par_iter()
        .map(|&image_path| {
            let image = ImageReader::open(image_path)
                .map_err(|e| ImageLoaderError::from_io_error(e, image_path.to_path_buf()))?
                .decode()
                .map_err(|e| ImageLoaderError::from(e))?;

            if image.width() == width {
                // noop if widths match
                Ok(image.into())
            } else { 
                // resize image otherwise
                // let height = width as f32 * image.height() as f32 / image.width() as f32;
                Ok(image.resize(width, height, Lanczos3).into())
            }
        })
        .collect::<Result<Vec<RgbImage>, ImageLoaderError>>()?;

    // combine all images into one big strip
    let mut combined_image = RgbImage::new(width, images.iter().map(|image| image.height()).sum());
    let mut height_cursor = 0;

    for i in images {
        // This should never throw an error because the combined image height is set to the sum of all image heights.
        combined_image.copy_from(&i, 0, height_cursor).expect("all according to keikaku");
        height_cursor += i.height();
    }

    Ok(combined_image)
}
