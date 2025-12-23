use std::{fs, fs::metadata, path::PathBuf, error::Error};
use turbojpeg::{image, decompress_image};

// pub struct ImageShrinker;


pub fn shrink_path(path: &PathBuf, size: &usize, copy: &bool, recursive: &bool, out_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    let dir_items = fs::read_dir(path)?;

    for item in dir_items {
        let item = item?;
        let item_path = item.path();

        if *recursive && path.is_dir() {
            self::shrink_path(path, size, copy, recursive, out_dir)?;
        }
        
        // This likely needs to be changed since item_path is probably wrong
        let result = self::shrink_img(&item_path, size, copy, path, out_dir);
        if let Err(ref _err) = result {
            return result; // should actually check what the error was
        }
    }

    Ok(())
}

// size is in bytes
pub fn shrink_img(img_path: &PathBuf, size: &usize, copy: &bool, in_dir: &PathBuf, out_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    // Load file data
    let img_data = fs::read(in_dir.join(img_path))?;

    // Check whether the file is already small enough
    let img_metadata = fs::metadata(img_path)?;
    let img_size = img_metadata.len();
    if img_size <= (*size).try_into().unwrap() {
        if *copy {
            fs::write(out_dir.join(img_path), img_data)?;
        }
        return Ok(());
    }

    // Convert to image type, checking whether it's an image 
    let img_data: image::RgbImage = turbojpeg::decompress_image(&img_data)?;

    // Compress image with worse quality iteratively 
    let mut quality = 100;
    while quality > 0 {
        // Compress the image
        let compressed_img_data = turbojpeg::compress_image(&img_data, quality, turbojpeg::Subsamp::Sub2x2)?;

        // If the image is small enough, save it 
        if compressed_img_data.len() <= *size {
            fs::write(out_dir.join(img_path), &compressed_img_data)?;
            return Ok(());
        }

        quality -= 1;
    }

    // temp, should be an error
    return Ok(());
}