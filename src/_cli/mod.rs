//! # QuickStitch CLI Usage Guide
//!
//! The `qstitch` command is a quick and easy way to use QuickStitch from the command line.
//!
//! ## Installation
//!
//! Currently, the only way to install `qstitch` is by building and installing the project
//! yourself. These instructions assume you have `cargo` installed and setup on your system.
//!
//! First, clone this repository.
//!
//! ```HELP!!-kobo-kanaeru
//! git clone https://github.com/quietkiro/quickstitch
//! ```
//!
//! Then, install with the `cargo install` command.
//!
//! ```sh
//! cargo install --path quickstitch --bin qstitch --features=cli
//! ```
//!
//! Now, you should be able to run `qstitch --help` from the command line and see the default help
//! output:
//!
//! ```ascii
//! Quickly stitch raws
//!
//! Usage: qstitch [OPTIONS] <IMAGES|--dir <DIR>>
//!
//! Arguments:
//!   [IMAGES]...
//!           The images to stitch
//!
//! Options:
//!   -d, --dir <DIR>
//!           A directory of images to stitch
//!
//!   -o, --output <OUTPUT>
//!           The output directory to place the stitched images in
//!
//!   -s, --sort <SORT>
//!           The sorting method used to sort the images before stitching (only works with `--dir`).
//!           
//!           Given the images ["9.jpeg", "10.jpeg", "8.jpeg", "11.jpeg"]:
//!             - Logical: ["10.jpeg", "11.jpeg", 8.jpeg", "9.jpeg"]
//!             - Natural: ["8.jpeg", "9.jpeg", "10.jpeg", "11.jpeg"]
//!           
//!           [default: natural]
//!
//!           Possible values:
//!           - logical: Sorts files lexicographically, treating numbers as strings of digits and not as atomic numbers.
//!           - natural: Treats numbers in the file name atomically, sorting them by numerical value.
//!
//!   -h, --help
//!           Print help (see a summary with '-h')
//!
//!   -V, --version
//!           Print version
//!  
//! ```
//!
//! ## Stitching Images
//!
//! Stitching images with `qstitch` is quite simple. Let's say you have a `images` directory filled
//! with JPEG images.
//!
//! ```im-a-tree
//! images
//! ├── 1.jpg
//! ├── 10.jpg
//! ├── 11.jpg
//! ├── 12.jpg
//! ├── 13.jpg
//! ├── 14.jpg
//! ├── 15.jpg
//! ├── 2.jpg
//! ├── 3.jpg
//! ├── 4.jpg
//! ├── 5.jpg
//! ├── 6.jpg
//! ├── 7.jpg
//! ├── 8.jpg
//! └── 9.jpg
//! ```
//!
//! Now, all you have to do is pass these images to `qstitch`. You can either use file globbing
//!
//! ```sh
//! qstitch images/*
//! ```
//!
//! or you can use the `--dir` flag.
//!
//! ```sh
//! qstitch --dir images
//! ```
//!
//! Either way, your images will get stitched.

// TODO: talk about sorting and more details about controlling output
