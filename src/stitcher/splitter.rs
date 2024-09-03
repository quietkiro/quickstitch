//! This module consists of functions related to the splitting of the combined image.

use image::{Pixel, Rgb, RgbImage};
use itertools::Itertools;

/// Finds all the rows of pixels which should be cut.
///
/// Input parameters:
///  - `image` - A reference to the combined image.
///  - `scan_interval` - The interval at which rows of pixels will be scanned.
///  - `sensitivity` - A value between 0 and 255, determining the threshold at which a row can be marked as a splitpoint.
///     - 0 would be no sensitivity, i.e. it doesn't matter what the pixels in the row are, it will be set as a splitpoint.
///     - 255 would be full sensitivity, i.e. all pixels in the row must be exactly the same color for it to be set as a splitpoint.
pub fn find_splitpoints(
    image: &RgbImage,
    target_height: usize,
    scan_interval: usize,
    sensitivity: u8,
) -> Vec<usize> {
    let limit = u8::MAX - sensitivity;
    let mut splitpoints = vec![];
    let mut cursor = target_height;
    loop {
        let row_max_pixel_diffs = image
            .rows()
            .map(|row| {
                row.into_iter()
                    .tuple_windows::<(_, _)>()
                    .fold(u8::MAX, |a, (pixel_a, pixel_b)| {
                        a.max(pixel_a.to_luma().0[0].abs_diff(pixel_b.to_luma().0[0]))
                    })
            })
            .enumerate()
            .take(cursor)
            .rev()
            .take(target_height)
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
                splitpoints.push(a.0);
                clean_splitpoint_found = true;
                break;
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
        if !clean_splitpoint_found && min_splitpoint.is_some() {
            splitpoints.push(min_splitpoint.unwrap().0)
        }
        cursor += target_height;
        if cursor > image.height() as usize {
            break;
        }
    }
    splitpoints.push(image.height() as usize);
    splitpoints
}

/// Does exactly the same thing as the `find_splitpoints` method, but each scan line in the image is visually
/// marked red (if max pixel diff exceeds threshold) or sky blue (if max pixel diff is below threshold)
/// to indicate the max pixel diff.
///
/// As a copy of the image must be created, this method may be slower than `find_splitpoints`.
///
/// Input parameters:
///  - `image` - A mutable reference to the combined image.
///  - `scan_interval` - The interval at which rows of pixels will be scanned.
///  - `sensitivity` - A value between 0 and 255, determining the threshold at which a row can be marked as a splitpoint.
///     - 0 would be no sensitivity, i.e. it doesn't matter what the pixels in the row are, it will be set as a splitpoint.
///     - 255 would be full sensitivity, i.e. all pixels in the row must be exactly the same color for it to be set as a splitpoint.
pub fn find_splitpoints_debug(
    image: &mut RgbImage,
    target_height: usize,
    scan_interval: usize,
    sensitivity: u8,
) -> Vec<usize> {
    let limit = u8::MAX - sensitivity;
    let mut splitpoints = vec![];
    let mut cursor = target_height;
    let ref_image = image.clone();
    loop {
        let row_max_pixel_diffs = ref_image
            .rows()
            .map(|row| {
                row.into_iter()
                    .tuple_windows::<(_, _)>()
                    .fold(u8::MAX, |a, (pixel_a, pixel_b)| {
                        a.max(pixel_a.to_luma().0[0].abs_diff(pixel_b.to_luma().0[0]))
                    })
            })
            .enumerate()
            .take(cursor)
            .rev()
            .take(target_height)
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
                let curr_max = a.1.max(b.1.max(c.1));
                let to_mark = image.width() * (curr_max as f32 / u8::MAX as f32) as u32;
                for pixel in 0..to_mark {
                    image.put_pixel(pixel, a.0 as u32, Rgb([53, 81, 92]));
                }
                splitpoints.push(a.0);
                clean_splitpoint_found = true;
                break;
            }
            // Otherwise, keep track of the minimum maximum of the three rows' max pixel diff.
            let curr_max = a.1.max(b.1.max(c.1));
            let to_mark = image.width() * (curr_max as f32 / u8::MAX as f32) as u32;
            for pixel in 0..to_mark {
                image.put_pixel(pixel, a.0 as u32, Rgb([255, 0, 0]));
            }

            match min_splitpoint {
                Some(prev) => {
                    if prev.1 > curr_max {
                        min_splitpoint = Some(a)
                    }
                }
                None => min_splitpoint = Some(a),
            }
        }
        if !clean_splitpoint_found && min_splitpoint.is_some() {
            splitpoints.push(min_splitpoint.unwrap().0)
        }
        cursor += target_height;
        if cursor > image.height() as usize {
            break;
        }
    }
    splitpoints.push(ref_image.height() as usize);
    splitpoints
}
