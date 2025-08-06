//! A crate for stitching together manhua/manhwa/manga raws.
//!
//! todo: add example for quickly getting started

// API

mod stitcher;

pub use stitcher::image_loader::Sort;
pub use stitcher::image_splitter::{ImageOutputFormat, Splitpoint};

use std::path::Path;

use image::RgbImage;
use stitcher::{
    image_loader::{ImageLoaderError, find_images, load_images},
    image_splitter::{ImageSplitterError, find_splitpoints, split_image},
};

mod seal {
    pub trait Seal {}
}

/// The state of the stitcher.
pub trait StitcherState: seal::Seal {}

/// The stitcher will be in the `Empty` state if no images have been loaded.
pub struct Empty;

/// The `Loaded` state denotes that the source images have been loaded and combined.
pub struct Loaded {
    strip: RgbImage,
}

/// The `Stitched` state denotes that the combined image has been scanned, and splitpoints have been found.
pub struct Stitched {
    strip: RgbImage,
    splitpoints: Vec<Splitpoint>,
}

// By sealing all the states, it prevents them from being modified by other crates downstream.
impl seal::Seal for Empty {}
impl seal::Seal for Loaded {}
impl seal::Seal for Stitched {}
impl StitcherState for Empty {}
impl StitcherState for Loaded {}
impl StitcherState for Stitched {}

pub struct Stitcher<S: StitcherState> {
    data: S,
}

impl Stitcher<Empty> {
    /// This allows all images within a directory to be loaded into the program.
    ///
    /// Parameters:
    /// - directory: The path of the selected source directory
    /// - width: An optional parameter for fixing the width of the combined image.
    ///          If the width is not provided, the width of the image with the
    ///          smallest width will be used.
    /// - ignore_unloadable: Skips images that are unable to be loaded properly.
    ///                      This may be useful if the source directory is known
    ///                      to have duplicate images, of which one is.
    /// - sort: Sorting method for the images in the directory.
    pub fn load_dir(
        self,
        directory: impl AsRef<Path>,
        width: Option<u32>,
        ignore_unloadable: bool,
        sort: Sort,
    ) -> Result<Stitcher<Loaded>, ImageLoaderError> {
        let images = find_images(directory, sort)?;
        Ok(Stitcher {
            data: Loaded {
                strip: load_images(&images, width, ignore_unloadable)?,
            },
        })
    }
    /// This loads individual images in the order they are given.
    ///
    /// Parameters:
    /// - images: File paths to each individual image.
    /// - width: An optional parameter for fixing the width of the combined image.
    ///          If the width is not provided, the width of the image with the
    ///          smallest width will be used.
    /// - ignore_unloadable: Skips images that are unable to be loaded properly.
    ///                      This may be useful if the source directory is known
    ///                      to have duplicate images, of which one is.
    pub fn load(
        self,
        images: &[impl AsRef<Path>],
        width: Option<u32>,
        ignore_unloadable: bool,
    ) -> Result<Stitcher<Loaded>, ImageLoaderError> {
        Ok(Stitcher {
            data: Loaded {
                strip: load_images(images, width, ignore_unloadable)?,
            },
        })
    }
    /// Create an empty stitcher.
    pub fn new() -> Stitcher<Empty> {
        Stitcher { data: Empty {} }
    }
}

impl Stitcher<Loaded> {
    /// Find the splitpoints for the loaded images.
    pub fn stitch(
        self,
        target_height: usize,
        min_height: usize,
        scan_interval: usize,
        sensitivity: u8,
    ) -> Stitcher<Stitched> {
        let splitpoints = find_splitpoints(
            &self.data.strip,
            target_height,
            min_height,
            scan_interval,
            sensitivity,
        );
        Stitcher {
            data: Stitched {
                strip: self.data.strip,
                splitpoints,
            },
        }
    }
    /// Get a reference to the combined image.
    pub fn view_image(&self) -> &RgbImage {
        &self.data.strip
    }
}

impl Stitcher<Stitched> {
    /// Get a reference to the combined image.
    pub fn view_image(&self) -> &RgbImage {
        &self.data.strip
    }
    /// Export all the resulting images into a given directory.
    ///
    /// Parameters:
    /// - output_directory: The path to the output directory.
    /// - output_filetype: The image file format that should be used
    ///                    for exporting the images.
    /// - debug: Enable debug mode. This causes red and blue/grey lines to
    ///          appear in the output images, denoting cut and skipped
    ///          splitpoints. Useful for tuning the scan interval.
    pub fn export(
        &self,
        output_directory: impl AsRef<Path>,
        output_filetype: ImageOutputFormat,
        debug: bool,
    ) -> Result<(), Vec<ImageSplitterError>> {
        split_image(
            &self.data.strip,
            &self.data.splitpoints,
            output_directory,
            output_filetype,
            debug,
        )
    }
    /// Get a reference to the splitpoints.
    pub fn splitpoits(&self) -> &Vec<Splitpoint> {
        &self.data.splitpoints
    }
}
