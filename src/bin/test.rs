use quickstitch::{Empty, ImageOutputFormat, Stitcher};

struct New {}

fn main() {
    let chapter: Stitcher<Empty> = Stitcher::new();
    let loaded = chapter.load_dir("../sample", None, true).unwrap();
    let stitched = loaded.stitch(10000, 5, 220);
    stitched
        .export("../output", ImageOutputFormat::Jpeg(100))
        .unwrap();
}
