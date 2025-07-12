//! This module consists of functions related to the splitting of the combined image.

use std::{
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use image::{
    GenericImageView, ImageError, Pixel, Rgb, RgbImage,
    codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
};
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use thiserror::Error;

#[derive(Debug)]
pub enum Splitpoint {
    Cut(usize),
    Skipped(usize),
}

impl Splitpoint {
    fn is_cut(&self) -> bool {
        match self {
            Self::Cut(_) => true,
            Self::Skipped(_) => false,
        }
    }
    fn get(&self) -> usize {
        match *self {
            Self::Cut(row) => row,
            Self::Skipped(row) => row,
        }
    }
    fn switch(&mut self) {
        match *self {
            Self::Cut(row) => *self = Self::Skipped(row),
            Self::Skipped(row) => *self = Self::Cut(row),
        }
    }
}

/// Finds all the rows of pixels which should be cut.
///
/// Input parameters:
///  - `image` - A reference to the combined image.
///  - `max_height` - How many pixels tall each page should be at most.
///  - `min_height` - How many pixels tall each page should be at least.
///  - `scan_interval` - The interval at which rows of pixels will be scanned.
///  - `sensitivity` - A value between 0 and 255, determining the threshold at which a row can be marked as a splitpoint.
///     - 0 would be no sensitivity, i.e. it doesn't matter what the pixels in the row are, it will be set as a splitpoint.
///     - 255 would be full sensitivity, i.e. all pixels in the row must be exactly the same color for it to be set as a splitpoint.
pub fn find_splitpoints(
    image: &RgbImage,
    max_height: usize,
    min_height: usize,
    scan_interval: usize,
    sensitivity: u8,
) -> Vec<Splitpoint> {
    let target_height = max_height + 1;
    let limit = u8::MAX - sensitivity;
    let mut splitpoints = vec![Splitpoint::Cut(0)];
    let mut cursor = target_height;
    // Loop until we have processed the whole strip
    loop {
        let row_max_pixel_diffs = image
            .rows()
            .map(|row| {
                row.into_iter()
                    .tuple_windows::<(_, _)>()
                    // Gets the maximum horizontal pixel difference of a row
                    .fold(0, |a, (pixel_a, pixel_b)| {
                        a.max(pixel_a.to_luma().0[0].abs_diff(pixel_b.to_luma().0[0]))
                    })
            })
            .enumerate()
            .take(cursor)
            .rev()
            .take(target_height - min_height)
            .step_by(scan_interval)
            .tuple_windows::<(_, _, _)>();
        let mut min_splitpoint: Option<(usize, u8)> = None;
        // This is to figure out how the loop exits. If a clean splitpoint (splitpoint which is under threshold) is found,
        // we won't need to push the min_splitpoint into the splitpoints vector.
        let mut clean_splitpoint_found = false;
        for (a, b, c) in row_max_pixel_diffs {
            // Debug mode
            // If all three rows' pixel diffs are below the threshold, mark it as a cut point.
            if a.1 <= limit && b.1 <= limit && c.1 <= limit {
                splitpoints.push(Splitpoint::Cut(a.0));
                cursor = a.0 + target_height;
                clean_splitpoint_found = true;
                break;
            } else {
                splitpoints.push(Splitpoint::Skipped(a.0));
            }
            // Otherwise, keep track of the minimum maximum of the three rows' max pixel diff.
            let curr_max = a.1.max(b.1.max(c.1));
            match min_splitpoint {
                Some(prev) => {
                    if prev.1 > curr_max {
                        min_splitpoint = Some(a)
                    }
                }
                None => min_splitpoint = Some(a),
            }
        }
        if !clean_splitpoint_found {
            if let Some(min) = min_splitpoint {
                // The minimum splitpoint is going to be one that we have already marked as "skipped".
                // To find and replace it efficiently, we can take advantage of the fact that the minimum
                // is most likely to be near the end of the vector, thus searching the vector in reverse
                // is most efficient.
                splitpoints
                    .iter_mut()
                    .rev()
                    .find(|splitpoint| splitpoint.get() == min.0)
                    .map(|splitpoint| splitpoint.switch());
                cursor = min_splitpoint.unwrap().0 + target_height;
            }
        }
        if cursor > image.height() as usize {
            break;
        }
    }
    splitpoints.push(Splitpoint::Cut(image.height() as usize));
    splitpoints
}

/// Does exactly the same thing as the `find_splitpoints` function, but each scan line in the image is visually
/// marked red (if max pixel diff exceeds threshold) or sky blue (if max pixel diff is below threshold)
/// to indicate the max pixel diff.
///
/// As a copy of the image must be created, this function may be slower than `find_splitpoints`.
///
/// Input parameters:
///  - `image` - A mutable reference to the combined image.
///  - `target_height` - How many pixels tall each page should be at most.
///  - `scan_interval` - The interval at which rows of pixels will be scanned.
///  - `sensitivity` - A value between 0 and 255, determining the threshold at which a row can be marked as a splitpoint.
///     - 0 would be no sensitivity, i.e. it doesn't matter what the pixels in the row are, it will be set as a splitpoint.
///     - 255 would be full sensitivity, i.e. all pixels in the row must be exactly the same color for it to be set as a splitpoint.
// pub fn find_splitpoints_debug(
//     image: &mut RgbImage,
//     target_height: usize,
//     scan_interval: usize,
//     sensitivity: u8,
// ) -> Vec<usize> {
//     let target_height = target_height + 1;
//     let limit = u8::MAX - sensitivity;
//     let mut splitpoints = vec![0];
//     let mut cursor = target_height;
//     let ref_image = image.clone();
//     loop {
//         let row_max_pixel_diffs = ref_image
//             .rows()
//             .map(|row| {
//                 row.into_iter()
//                     .tuple_windows::<(_, _)>()
//                     .fold(0, |a, (pixel_a, pixel_b)| {
//                         a.max(pixel_a.to_luma().0[0].abs_diff(pixel_b.to_luma().0[0]))
//                     })
//             })
//             .enumerate()
//             .take(cursor)
//             .rev()
//             .take(target_height)
//             .step_by(scan_interval)
//             .tuple_windows::<(_, _, _)>();
//         let mut min_splitpoint: Option<(usize, u8)> = None;
//         // This is to figure out how the loop exits. If a clean splitpoint (splitpoint which is under threshold) is found,
//         // we won't need to push the min_splitpoint into the splitpoints vector.
//         let mut clean_splitpoint_found = false;
//         for (a, b, c) in row_max_pixel_diffs {
//             // If all three rows' pixel diffs are below the threshold, mark it as a cut point.
//             if a.1 <= limit && b.1 <= limit && c.1 <= limit {
//                 let curr_max = a.1.max(b.1.max(c.1));
//                 let to_mark = (image.width() as f32 * (curr_max as f32 / u8::MAX as f32)) as u32;
//                 for pixel in 0..to_mark {
//                     image.put_pixel(pixel, a.0 as u32, Rgb([53, 81, 92]));
//                 }
//                 splitpoints.push(a.0);
//                 cursor = a.0 + target_height;
//                 clean_splitpoint_found = true;
//                 break;
//             }
//             // Otherwise, keep track of the minimum maximum of the three rows' max pixel diff.
//             let curr_max = a.1.max(b.1.max(c.1));
//             let to_mark = (image.width() as f32 * (curr_max as f32 / u8::MAX as f32)) as u32;
//             for pixel in 0..to_mark {
//                 image.put_pixel(pixel, a.0 as u32, Rgb([255, 0, 0]));
//             }

//             match min_splitpoint {
//                 Some(prev) => {
//                     if prev.1 > curr_max {
//                         min_splitpoint = Some(a)
//                     }
//                 }
//                 None => min_splitpoint = Some(a),
//             }
//         }
//         if !clean_splitpoint_found && min_splitpoint.is_some() {
//             splitpoints.push(min_splitpoint.unwrap().0);
//             cursor = min_splitpoint.unwrap().0 + target_height;
//         }
//         if cursor > image.height() as usize {
//             break;
//         }
//     }
//     splitpoints.push(ref_image.height() as usize);
//     splitpoints
// }

/// A helper function to calculate the number of digits a `usize` number has
fn get_num_digits(num: usize) -> usize {
    // this is safe because the number of digits of a `usize` will always be
    // within the range of a `u32` anyway
    num.checked_ilog10().unwrap_or(0) as usize + 1
}

#[derive(Error, Debug)]
pub enum ImageSplitterError {
    #[error("Could not find the provided directory")]
    DirectoryNotFound,
    #[error("Insufficient permissions within the provided directory")]
    PermissionDenied,

    // upstream errors
    #[error("{0:?}")]
    ImageError(ImageError),
    #[error("{0}")]
    IoError(io::Error),
}

pub enum ImageOutputFormat {
    Png,
    Webp,
    Jpeg(u8),
    Jpg(u8),
}

impl From<ImageError> for ImageSplitterError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<io::Error> for ImageSplitterError {
    fn from(value: io::Error) -> Self {
        use io::ErrorKind as Kind;
        match value.kind() {
            Kind::PermissionDenied => ImageSplitterError::PermissionDenied,
            _ => ImageSplitterError::IoError(value),
        }
    }
}

/// Uses the provided splitpoints, image, and output image filetype to split the image into smaller images
/// and exports those images into the provided output directory.
///
/// Input parameters:
///  - image: A reference to the combined image.
///  - splitpoints: A vector containing the pixel height at which the combined image should be split.
///  - output_directory: The output directory where the split images are to be exported.
///  - output_filetype: The output image filetype along with the quality setting (if applicable).
///
/// Throws an error if:
///  - Any of the split images fails to be exported.
///  - The output directory provided is not a valid directory.
///  - This program does not have adequate permissions to create the images inside the provided directory.
///  - The split images are too large in dimension for the output filetype.
pub fn split_image(
    image: &RgbImage,
    splitpoints: &Vec<Splitpoint>,
    output_directory: impl AsRef<Path>,
    output_filetype: ImageOutputFormat,
    debug: bool,
) -> Result<(), Vec<ImageSplitterError>> {
    dbg!(splitpoints);
    let output_directory = output_directory.as_ref().to_path_buf();
    if !output_directory.is_dir() {
        return Err(vec![ImageSplitterError::DirectoryNotFound]);
    }
    let cut_splitpoints: Vec<_> = splitpoints
        .iter()
        .filter(|splitpoint| splitpoint.is_cut())
        .map(|splitpoint| splitpoint.get())
        .collect();
    let max_digits = get_num_digits(cut_splitpoints.len());
    let output: Vec<Result<(), ImageSplitterError>> = cut_splitpoints
        .windows(2)
        .map(|slice| (slice[0], slice[1] - slice[0]))
        .collect::<Vec<(_, _)>>()
        .par_iter()
        .enumerate()
        .map(|(index, (start, length))| {
            let mut page = image
                .view(
                    0,
                    start.to_owned() as u32,
                    image.width(),
                    length.to_owned() as u32,
                )
                .to_image();
            if debug {
                // Get all the skipped splitpoints on this page
                let skipped: Vec<_> = splitpoints
                    .iter()
                    .skip_while(|splitpoint| splitpoint.get() < *start)
                    .take_while(|splitpoint| splitpoint.get() < start + length)
                    .map(|splitpoint| splitpoint.get() - start)
                    .collect();
                skipped.iter().for_each(|skipped_row| {
                    for i in 0..image.width() {
                        page.put_pixel(i, *skipped_row as u32, Rgb([53, 81, 92]));
                    }
                });
                for i in 0..image.width() {
                    page.put_pixel(i, *length as u32, Rgb([255, 0, 0]));
                }
            }
            let mut output_filepath = output_directory.clone();
            output_filepath.push(format!(
                "{}{}.{}",
                "0".repeat(max_digits - get_num_digits(index + 1)),
                index + 1,
                match output_filetype {
                    ImageOutputFormat::Png => "png",
                    ImageOutputFormat::Jpeg(_) => "jpeg",
                    ImageOutputFormat::Webp => "webp",
                    ImageOutputFormat::Jpg(_) => "jpg",
                }
            ));
            let file = match File::create(output_filepath) {
                Ok(file) => file,
                Err(e) => {
                    return Err(ImageSplitterError::from(e));
                }
            };
            // May be the cause of unknown errors.
            let res = match output_filetype {
                ImageOutputFormat::Png => {
                    page.write_with_encoder(PngEncoder::new(BufWriter::new(file)))
                }
                ImageOutputFormat::Webp => {
                    page.write_with_encoder(WebPEncoder::new_lossless(BufWriter::new(file)))
                }
                ImageOutputFormat::Jpeg(quality) | ImageOutputFormat::Jpg(quality) => page
                    .write_with_encoder(JpegEncoder::new_with_quality(
                        BufWriter::new(file),
                        quality,
                    )),
            };
            match res {
                Ok(_) => Ok(()),
                Err(e) => Err(ImageSplitterError::from(e)),
            }
        })
        .collect();
    let errors: Vec<_> = output.into_iter().filter_map(|out| out.err()).collect();
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}
