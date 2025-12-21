use std::{fs, path::PathBuf};
use turbojpeg::Compressor;
use image::RgbImage;

pub struct ImageShrinker;

impl ImageShrinker {
    pub fn shrink_dir(dir_path: &PathBuf, size: &f64, recursive: bool, out_dir: &PathBuf) -> Result<Box <dyn Error>> {
        let dir_items = fs::read_dir(dir_path)?;

        for item in dir_items {
            let item = item?;
            let item_path = item.path();

            if recursive && path.is_dir() {
                self::shrink_dir(&path, size, recursive)
            }
            
            let result = self::shrink_img(&item_path, size, out_dir);
            if let Err(err) = result {
                return result; // should actually check what the error was
            }
        }
    }

    pub fn shrink_img(img_path: &PathBuf, size: &f64, out_dir: &PathBuf) -> Result<Box <dyn Error>> {
        // Check whether the file is already small enough
        let img_metadata = fs::metadata(img_path)?;
        let img_size = metadata.len();
        if img_size <= size {
            return Ok(());
        }

        // Load file data
        let img_data = fs::read(img_path)?;
        
        // Convert to image type, checking whether it's an image 
        let image: image::RgbImage = turbojpeg::decompress_image(&img_data)?;

        
    }
}