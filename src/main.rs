use std::path::PathBuf;

use quickstitch::gui;
use quickstitch::stitcher::splitter::split_image;
use quickstitch::stitcher::{image_loader, splitter};

fn main() {
    dbg!(image_loader::find_images("./testing/sample"));
    let mut image = image_loader::load_images("./testing/sample", None).unwrap();
    let splitpoints = splitter::find_splitpoints_debug(&mut image, 5000, 5, 240);
    // let image = image_loader::load_images("./testing/sample", None).unwrap();
    // let splitpoints = splitter::find_splitpoints(&image, 15000, 5, 240);
    dbg!(&splitpoints);
    split_image(&image, &splitpoints, PathBuf::from("./testing/output"), 100);
}
