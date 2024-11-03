//! A crate for stitching together manhua/manhwa/manga raws.
//!
//! todo: add example for quickly getting started

pub mod stitcher;

#[cfg(feature = "cli")]
pub mod _cli;

#[cfg(feature = "gui")]
pub mod _gui;
