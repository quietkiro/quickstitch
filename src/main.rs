use quickstitch::stitcher::image_loader;

fn main() {
    image_loader::load_images("./combined", Some(800)).unwrap();
}
