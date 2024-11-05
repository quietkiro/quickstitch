use anyhow::Result;
use clap::{Args, Parser};
use quickstitch::{ImageOutputFormat, Loaded, Stitcher};
use std::path::{Path, PathBuf};

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
    output: Option<PathBuf>,
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
        (None, Some(dir)) => stitcher.load_dir(&dir, None, true)?,
        _ => unimplemented!("arg group rules ensure only one of the two is provided"),
    };
    let stitched = loaded.stitch(1000, 5, 220);

    // TODO: handle errors here someday
    match cli.output {
        Some(output) => {
            std::fs::create_dir_all(&output)?;
            let _ = stitched.export(&output, ImageOutputFormat::Jpg(100));
        }
        None => {
            std::fs::create_dir_all("./stitched")?;
            let _ = stitched.export("./stitched/", ImageOutputFormat::Jpg(100));
        }
    }

    Ok(())
}
