use std::path::PathBuf;
use std::time::Instant;

use quickstitch::gui;
use quickstitch::stitcher::splitter::split_image;
use quickstitch::stitcher::{image_loader, splitter};

fn main() {
    // dbg!(image_loader::find_images("./testing/sample"));
    // let mut image = image_loader::load_images("./testing/sample", None).unwrap();
    // let splitpoints = splitter::find_splitpoints_debug(&mut image, 5000, 5, 240);
    let now = Instant::now();
    let image = image_loader::load_images("./testing/sample", Some(800)).unwrap();
    println!("Images loaded in {:.2?}", now.elapsed());
    let now = Instant::now();
    let splitpoints = splitter::find_splitpoints(&image, 5000, 5, 242);
    println!("Found splitpoints in {:.2?}", now.elapsed());
    let now = Instant::now();
    split_image(&image, &splitpoints, PathBuf::from("./testing/output"), 100);
    println!("Split image in {:.2?}", now.elapsed());
}
