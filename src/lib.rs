//! A crate for stitching together manhua/manhwa/manga raws.
//!
//! todo: add example for quickly getting started

mod stitcher;

pub use stitcher::image_splitter::ImageOutputFormat;

use std::path::Path;

use image::RgbImage;
use stitcher::{
    image_loader::{find_images, load_images, ImageLoaderError, Sort},
    image_splitter::{find_splitpoints, find_splitpoints_debug, split_image, ImageSplitterError},
};

pub trait StitcherState {}

// No images loaded
pub struct Empty;

// Images have been loaded and combined
pub struct Loaded {
    strip: RgbImage,
}

// Images have been cut up
pub struct Stitched {
    strip: RgbImage,
    splitpoints: Vec<usize>,
}

impl StitcherState for Empty {}
impl StitcherState for Loaded {}
impl StitcherState for Stitched {}

pub struct Stitcher<S: StitcherState> {
    data: S,
}

impl Stitcher<Empty> {
    pub fn load_dir(
        self,
        directory: impl AsRef<Path>,
        width: Option<u32>,
        ignore_unloadable: bool,
    ) -> Result<Stitcher<Loaded>, ImageLoaderError> {
        let images = find_images(directory, Sort::Natural)?;
        Ok(Stitcher {
            data: Loaded {
                strip: load_images(&images, width, ignore_unloadable)?,
            },
        })
    }
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
    pub fn new() -> Stitcher<Empty> {
        Stitcher { data: Empty {} }
    }
}

impl Stitcher<Loaded> {
    pub fn stitch(
        self,
        target_height: usize,
        scan_interval: usize,
        sensitivity: u8,
    ) -> Stitcher<Stitched> {
        let splitpoints =
            find_splitpoints(&self.data.strip, target_height, scan_interval, sensitivity);
        Stitcher {
            data: Stitched {
                strip: self.data.strip,
                splitpoints,
            },
        }
    }
    pub fn stitch_debug(
        mut self,
        target_height: usize,
        scan_interval: usize,
        sensitivity: u8,
    ) -> Stitcher<Stitched> {
        let splitpoints = find_splitpoints_debug(
            &mut self.data.strip,
            target_height,
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
}

impl Stitcher<Stitched> {
    pub fn view_image(&self) -> &RgbImage {
        &self.data.strip
    }
    pub fn export(
        &self,
        output_directory: impl AsRef<Path>,
        output_filetype: ImageOutputFormat,
    ) -> Result<(), Vec<ImageSplitterError>> {
        split_image(
            &self.data.strip,
            &self.data.splitpoints,
            output_directory,
            output_filetype,
        )
    }
}
