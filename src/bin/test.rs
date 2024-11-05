use quickstitch::{Empty, ImageOutputFormat, Stitcher};

fn main() {
    let chapter: Stitcher<Empty> = Stitcher::new();
    let loaded = chapter
        .load_dir("../sample", None, true, quickstitch::Sort::Natural)
        .unwrap();
    let stitched = loaded.stitch(10000, 5, 220);
    stitched
        .export("../output", ImageOutputFormat::Jpeg(100))
        .unwrap();
}
