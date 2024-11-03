use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The images to stitch.
    ///
    /// If a directory is provided, then the images contained in that directory will be stitched.
    images: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli.images);
}
