use clap::Parser;
use std::{path::PathBuf};

mod image_shrinker;

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

fn main() {
    let cli = Cli::parse();

    // println!("input_path: {:?}", cli.input_path);
    // println!("output_dir: {:?}", cli.output_dir);
    // println!("size: {:?}", cli.size);
    // println!("recursive: {:?}", cli.recursive);
    // println!("copy: {:?}", cli.copy);

    let size: usize = (cli.size * 1000000.0).trunc() as usize;

    match cli.input_path.is_dir() {
        true => image_shrinker::shrink_path(&cli.input_path, &size, &cli.copy, &cli.recursive, &cli.output_dir).unwrap(),
        false => image_shrinker::shrink_img(&cli.input_path, &size, &cli.copy, &cli.output_dir).unwrap(),
    }

    // let paths = fs::read_dir(cli.input_path).unwrap();

    // for path in paths {
    //     println!("Name: {}", path.as_ref().unwrap().path().display());
    //     println!("Size: {}", path.as_ref().unwrap().metadata().unwrap().len());
    // }
}