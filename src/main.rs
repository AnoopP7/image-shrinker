// main.rs
// Gets command line arguments and calls image_shrinker functions appropriately
// to facilitate compressing a directory or individual image
// Author: Anoop Prasad

use clap::Parser;
use std::path::PathBuf;

// Cli struct for getting command line arguments using clap
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value = ".", help = "Path to file or directory to process")]
    input_path: PathBuf,
    #[arg(default_value = "shrink", help = "Directory to store processed images")]
    output_dir: PathBuf,
    #[arg(default_value_t = 10.0, short, long, help = "The maximum size for shrinking images in MB")]
    size: f64,
    #[arg(default_value_t = false, short, long, help = "Recursively process child directories if the path is a directory")]
    recursive: bool,
    #[arg(default_value_t = false, short, long, help = "Copy unchanged images to the output directory if they were already small enough")]
    copy: bool,
}

// Gets command line arguments and calls an image_shrinker function based on the arguments
fn main() {
    // Get command line arguments
    let cli = Cli::parse();

    // Convert size from MB to B
    let size: usize = (cli.size * 1000000.0).trunc() as usize;

    // Check whether the user specified a directory or a single image and perform the work accordingly
    match cli.input_path.is_dir() {
        true => image_shrinker::shrink_path(&cli.input_path, &size, &cli.copy, &cli.recursive, &cli.output_dir, &cli.input_path).unwrap(),
        false => image_shrinker::shrink_img(&cli.input_path, &size, &cli.copy, &cli.output_dir, &cli.input_path.parent().unwrap().to_path_buf()).unwrap(),
    }
}