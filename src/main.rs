use quickstitch::gui;
use quickstitch::stitcher::image_loader;

fn main() {
    dbg!(image_loader::find_images("./testing/sample"));
    image_loader::load_images("./testing/sample", None).unwrap();
}
