use std::{fs, path::PathBuf, error::Error};
use turbojpeg::{image};

// pub struct ImageShrinker;

// size is in bytes
pub fn shrink_path(path: &PathBuf, size: &usize, copy: &bool, recursive: &bool, out_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    let dir_items = fs::read_dir(path)?;

    for item in dir_items {
        let item = item?;
        let item_path = item.path();

        if *recursive && item_path.is_dir() {
            shrink_path(&item_path, size, copy, recursive, out_dir)?;
        }
        
        let result = shrink_img(&item_path, size, copy, out_dir);
        if let Err(ref err) = result {
            eprintln!("{err}");
            // some errors might warrant exiting
        }
    }

    Ok(())
}

// size is in bytes
pub fn shrink_img(img_path: &PathBuf, size: &usize, copy: &bool, out_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    // Load file data
    let img_data = fs::read(img_path)?;

    // Check whether the file is already small enough
    let img_metadata = fs::metadata(img_path)?;
    let img_size = img_metadata.len();

    // Create path for new image
    let new_path = out_dir.join(img_path.file_name().expect("Tried to process a directory in shrink_img"));

    if !new_path.parent().unwrap().is_dir() {
        fs::create_dir_all(new_path.parent().unwrap())?;
    }
    
    if img_size <= (*size).try_into().expect("Encountered invalid size in shrink_img") {
        if *copy {
            fs::write(new_path, img_data)?;
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
            fs::write(new_path, &compressed_img_data)?;
            return Ok(());
        }

        quality -= 1;
    }

    // temp, should be an error
    return Ok(());
}