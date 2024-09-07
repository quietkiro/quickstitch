pub mod image_loader;
pub mod splitter;

use std::{io, marker::PhantomData, path::Path};

use image_loader::ImageLoaderError;

mod private {
    pub trait Seal {}
}

/// Marker trait for possible states of a [Stitcher] type.
pub trait StitcherState: private::Seal {}

/// A marker type declaring a new [Stitcher] object.
/// 
/// [stitch](Stitcher::stitch) cannot be called on a [Blank] [Stitcher] object.
pub struct Blank;

/// A marker type declaring a [Stitcher] object containing images.
/// 
/// [stitch](Stitcher::stitch) can be called on a [Loaded] [Stitcher] object.
pub struct Loaded;

/// A marker type declaring a [Stitcher] object containing stitched images.
pub struct Stitched;

impl private::Seal for Blank {}
impl StitcherState for Blank {}
impl private::Seal for Loaded {}
impl StitcherState for Loaded {}
impl private::Seal for Stitched {}
impl StitcherState for Stitched {}

/// A simple way to load, stitch, and output images.
/// 
/// # Usage
/// 
/// ```rust
/// # use quickstitch::{Stitcher, stitcher::image_loader::ImageLoaderError};
/// # use std::io;
/// fn main() -> Result<(), ImageLoaderError> {
///     Stitcher::new()
///         .load(&["chapter-1.1", "chapter-1.2", "chapter-1.3"])?
///         .stitch()
///         .write();
/// 
///     Ok(())
/// }
/// ```
pub struct Stitcher<S: StitcherState> {
    _marker: PhantomData<S>,
}

impl Default for Stitcher<Blank> {
    fn default() -> Self {
        Self::new()
    }
}

impl Stitcher<Blank> {
    pub fn new() -> Self {
        Stitcher {
            _marker: PhantomData,
        }
    }

    pub fn load(self, images: &[impl AsRef<Path>]) -> Result<Stitcher<Loaded>, ImageLoaderError> {
        Ok(Stitcher {
            _marker: PhantomData,
        })
    }

    pub fn load_dir(self, dir: impl AsRef<Path>) -> Result<Stitcher<Loaded>, ImageLoaderError> {
        Ok(Stitcher {
            _marker: PhantomData,
        })
    }
}

impl Stitcher<Loaded> {
    pub fn stitch(self) -> Stitcher<Stitched> {
        Stitcher {
            _marker: PhantomData,
        }
    }
}

impl Stitcher<Stitched> {
    pub fn write(self) -> Result<(), io::Error> {
        Ok(())
    }
}