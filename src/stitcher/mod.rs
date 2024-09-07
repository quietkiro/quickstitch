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
pub struct Blank;

/// A marker type declaring a [Stitcher] object containing images.
pub struct Loaded;

/// A marker type declaring a [Stitcher] object containing split images.
pub struct Split;

impl private::Seal for Blank {}
impl StitcherState for Blank {}
impl private::Seal for Loaded {}
impl StitcherState for Loaded {}
impl private::Seal for Split {}
impl StitcherState for Split {}

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
/// 
/// # States
/// 
/// [Stitcher] is implemented such that you can only call certain methods on a
/// [Stitcher] object of a given state. For instance, you cannot call
/// [split](Stitcher::split) before calling [load](Stitcher::load).
/// 
/// ```compile_fail
/// # use quickstitch::Stitcher;
/// Stitcher::new().stitch(); // this will fail to compile
/// ```
/// 
/// There are three states: [Blank], [Loaded], and [Split].
/// 
/// ## [Blank]
/// 
/// The [Blank] state represents a new [Stitcher] object, with no images loaded.
/// 
/// You can call [load](Stitcher::load) to load images into the [Stitcher]
/// object, giving you a [Loaded] [Stitcher] object.
/// 
/// ## [Loaded]
/// 
/// The [Loaded] state represents a [Stitcher] object with images loaded. This
/// means that you can call [split](Stitcher::split), but can no longer call
/// [load](Stitcher::load) and cannot call [write](Stitcher::write).
/// 
/// ## [Split]
/// 
/// The [Split] state represents a [Stitcher] object that has split its images.
/// You can now call [write](Stitcher::write), but can no longer call any of the
/// other families of methods.
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
    pub fn split(self) -> Stitcher<Split> {
        Stitcher {
            _marker: PhantomData,
        }
    }
}

impl Stitcher<Split> {
    pub fn write(self) -> Result<(), io::Error> {
        Ok(())
    }
}