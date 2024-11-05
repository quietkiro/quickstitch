use std::path::PathBuf;
use clap::{Parser, Args};

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
    #[clap(long, short)]
    output: PathBuf,
}

#[derive(Debug, Clone, Args)]
#[group(required = true, multiple = false)]
struct Input {
    /// The images to stitch.
    images: Option<Vec<PathBuf>>,
    /// A directory of images to stitch.
    #[clap(long, short, alias = "dir")]
    dir: Option<Vec<PathBuf>>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?} {:?}", cli.input.images, cli.input.dir);
}
