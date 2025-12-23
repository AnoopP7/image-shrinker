use std::{fs, fs::metadata, path::PathBuf, error::Error};
use turbojpeg::{image, decompress_image};

// pub struct ImageShrinker;


pub fn shrink_path(path: &PathBuf, size: &u64, copy: &bool, recursive: &bool, out_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    let dir_items = fs::read_dir(path)?;

    for item in dir_items {
        let item = item?;
        let item_path = item.path();

        if *recursive && path.is_dir() {
            self::shrink_path(path, size, copy, recursive, out_dir)?;
        }
        
        let result = self::shrink_img(&item_path, size, copy, out_dir);
        if let Err(ref _err) = result {
            return result; // should actually check what the error was
        }
    }

    Ok(())
}

pub fn shrink_img(img_path: &PathBuf, size: &u64, copy: &bool, out_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    // Load file data
    let img_data = fs::read(img_path)?;

    // Check whether the file is already small enough
    let img_metadata = fs::metadata(img_path)?;
    let img_size = img_metadata.len();
    if img_size <= *size {
        if *copy {
            fs::write(img_path.join(out_dir), img_data)?;
        }
        return Ok(());
    }

    // Convert to image type, checking whether it's an image 
    let image: image::RgbImage = turbojpeg::decompress_image(&img_data)?;

    // temp
    return Ok(());
}