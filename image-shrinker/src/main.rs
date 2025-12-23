use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(help = "Path to file or directory to process")]
    input_path: PathBuf,
    #[arg(default_value = "shrink", help = "Directory to store processed images")]
    output_dir: PathBuf,
    #[arg(short, long, help = "The maximum size for shrinking images in MB")]
    size: f64,
    #[arg(short, long, help = "Recursively process child directories if the path is a directory")]
    recursive: bool,
    #[arg(short, long, help = "Copy unchanged images to the output directory if they were already small enough")]
    copy: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("input_path: {:?}", cli.input_path);
    println!("output_dir: {:?}", cli.output_dir);
    println!("recursive: {:?}", cli.recursive);

    let paths = fs::read_dir(cli.input_path).unwrap();

    for path in paths {
        println!("Name: {}", path.as_ref().unwrap().path().display());
        println!("Size: {}", path.as_ref().unwrap().metadata().unwrap().len());
    }
}