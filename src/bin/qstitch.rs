use anyhow::Result;
use clap::{value_parser, Args, Parser, ValueEnum};
use quickstitch as qs;
use quickstitch::{ImageOutputFormat, Loaded, Stitcher};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ImageFormat {
    Png,
    Webp,
    Jpg,
    Jpeg,
}

/// Quickly stitch raws.
///
/// A list of images can provided as input, or the `--dir` flag can be used
/// instead to specify a directory of images to stitch.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    input: Input,

    /// The output directory to place the stitched images in.
    #[clap(long, short, default_value = "./stitched")]
    output: PathBuf,

    /// The sorting method used to sort the images before stitching (only works with `--dir`).
    ///
    /// Given the images ["9.jpeg", "10.jpeg", "8.jpeg", "11.jpeg"]:
    ///   - Logical: ["10.jpeg", "11.jpeg", 8.jpeg", "9.jpeg"]
    ///   - Natural: ["8.jpeg", "9.jpeg", "10.jpeg", "11.jpeg"]
    #[clap(long, short, default_value_t = qs::Sort::Natural, verbatim_doc_comment)]
    #[arg(value_enum)]
    sort: qs::Sort,

    /// The target height for stitched images.
    ///
    /// Stitched images will aim to be as tall as this parameter,
    /// but they may be shorter if visual elements are in the way.
    #[clap(long, short, default_value_t = 5000)]
    height: usize,

    /// The interval at which lines of pixels are scanned. For example,
    /// a value of 5 means every 5th horizontal line of pixels will be
    /// analyzed.
    #[clap(long, short, default_value_t = 5)]
    scan_interval: usize,

    /// The threshold value between 0 and 255 for determining when a line of 
    /// pixels should not be used as a splitpoint. 0 would allow the line
    /// to be used as a splitpoint regardless of the line's pixels' values,
    /// while 255 would only allow the line to be used as a splitpoint if
    /// all the pixels in the line have the same value.
    #[clap(long, short, default_value_t = 220)]
    #[arg(value_parser(value_parser!(u8).range(0..=255)))]
    sensitivity: u8,

    /// The file extension/type used for exporting the stitched images.
    #[clap(long, short, default_value_t = ImageFormat::Jpg)]
    #[arg(value_enum)]
    format: ImageFormat,

    /// The image quality to aim for when compressing.
    ///
    /// A value from 1 to 100 may be provided to specify the amount 
    /// of compression to be used.
    /// A lower value represents more compression. This flag only takes
    /// effect when `--format` is passed a value of `jpg` (the default value)
    /// or `jpeg`. Otherwise, it will be ignored.
    #[clap(long, short, default_value_t = 100)]
    #[arg(value_parser(value_parser!(u8).range(1..=100)))]
    quality: u8,
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
struct Input {
    /// The images to stitch.
    images: Option<Vec<PathBuf>>,
    /// A directory of images to stitch.
    #[clap(long, short, alias = "dir")]
    dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let stitcher = Stitcher::new();
    let loaded: Stitcher<Loaded> = match (cli.input.images, cli.input.dir) {
        (Some(images), None) => {
            let paths: Vec<&Path> = images.iter().map(PathBuf::as_path).collect();
            stitcher.load(&paths, None, true)?
        }
        (None, Some(dir)) => stitcher.load_dir(&dir, None, true, cli.sort.into())?,
        _ => unimplemented!("arg group rules ensure only one of the two is provided"),
    };
    let stitched = loaded.stitch(cli.height, cli.scan_interval, cli.sensitivity);

    // TODO: handle errors here someday
    std::fs::create_dir_all(&cli.output)?;
    let _ = stitched.export(
        &cli.output,
        match cli.format {
            ImageFormat::Png => ImageOutputFormat::Png,
            ImageFormat::Webp => ImageOutputFormat::Webp,
            ImageFormat::Jpg => ImageOutputFormat::Jpg(cli.quality),
            ImageFormat::Jpeg => ImageOutputFormat::Jpeg(cli.quality),
        },
    );

    Ok(())
}
