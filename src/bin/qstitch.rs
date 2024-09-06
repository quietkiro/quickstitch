use std::path::PathBuf;
use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Sort {
    Normal,
    Lexicographic,
    None,
}

impl Default for Sort {
    fn default() -> Self {
        Sort::Normal
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Sort::Normal => "normal",
            Sort::Lexicographic => "lexicographic",
            Sort::None => "none",
        })
    }
}

/// Quickly stitch a bunch of images.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name = "PATH")]
    /// A list of paths to images or directories of images to stitch.
    paths: Vec<PathBuf>,
    #[arg(short, long, value_enum, value_name = "TYPE", default_value_t = Sort::Normal)]
    /// Sort the provided paths by the given sorting algorithm.
    /// 
    /// By default, paths are sorted according to normal order.
    /// 
    /// ["023.jpeg", "3.jpeg", "12.jpeg"] => ["3.jpeg", "12.jpeg", "023.jpeg"]
    /// 
    /// If "lexicographical", then paths will be sorted accordingly.
    /// 
    /// ["023.jpeg", "3.jpeg", "12.jpeg"] => ["023.jpeg", "12.jpeg", "3.jpeg"]
    /// 
    /// If "none", then paths will be stitched in the provided order.
    /// 
    /// ["023.jpeg", "3.jpeg", "12.jpeg"] => ["023.jpeg", "3.jpeg", "12.jpeg"]
    sort: Sort,
}

fn main() {
    let args = Args::parse();

    println!("Sort: {}", args.sort);
    println!("{:?}", args.paths);
}