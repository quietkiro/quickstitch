use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.paths);
}