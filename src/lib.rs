// lib.rs
// Image Shrinker
// Implements functions for applying JPEG compression to images within a directory or individually
// to get them below a specified size while preserving directory structure
// Author: Anoop Prasad

use std::{fs, path::PathBuf, error::Error};
use turbojpeg::{image};
use rayon::prelude::*;

// shrink_path
// Applies JPEG compression to each of the images at the given path to get them below the given size and saves to output directory
// Parameters:
// path: Path to the directory
// size: Maximum size in bytes for compression
// copy: If true, copies all images (regardless of whether they were already small enough) to output directory
// recursive: If true, recursively traverses all nested directories
// out_dir: Path to the output directory
// base_dir: Highest level parent directory for processing traversal (if not recursive, this will always be identical to path)
// Return:
// Result containing nothing if successful and an error if not
pub fn shrink_path(path: &PathBuf, size: &usize, copy: &bool, recursive: &bool, out_dir: &PathBuf, base_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    println!("Processing directory: {}", path.display());

    // Check for invalid size
    if *size <= 0 {
        let err = format!("Invalid size: {}", *size);
        Err(err)?;
    }

    // Get list of files in directory
    match fs::read_dir(path) {
        // Directory exists
        Ok(dir_items) => {
            // Collect items in directory to a vector
            let items: Vec<PathBuf> = dir_items.filter_map(|result| result.ok()).map(|entry| entry.path()).collect();

            // Iterate through each path in the directory in parallel
            items.par_iter().for_each(|item_path| {
                // If recursive mode was specified, run recursively on directories
                if *recursive && item_path.is_dir() {
                    let result = shrink_path(&item_path, size, copy, recursive, out_dir, base_dir);
                    if let Err(ref err) = result {
                        eprintln!("{err}");
                        // TODO: Some types of errors might warrant exiting
                    }
                // If not a directory, shrink the image
                } else if !item_path.is_dir() {
                    // Shrink an image
                    let result = shrink_img(&item_path, size, copy, out_dir, base_dir);
                    if let Err(ref err) = result {
                        eprintln!("{err}");
                        // TODO: Some types of errors might warrant exiting
                    }
                }     
            });
        },
        // Directory does not exist
        Err(err) => eprintln!("{err}"),
    }

    println!("Finished directory:   {}", path.display());
    Ok(())
}

// shrink_img
// Applies JPEG compression to the image at the given path to get it below the given size and saves to output directory
// Parameters:
// img_path: Path to the image
// size: Maximum size in bytes for compression
// copy: If true, copies image (regardless of whether they were already small enough) to output directory
// out_dir: Path to the output directory
// base_dir: Path to the directory in which the image is contained
// Return:
// Result containing nothing if successful and an error if not
pub fn shrink_img(img_path: &PathBuf, size: &usize, copy: &bool, out_dir: &PathBuf, base_dir: &PathBuf) -> Result<(), Box <dyn Error>> {
    println!("Processing file:      {}", img_path.display());

    // Check for invalid size
    if *size <= 0 {
        let err = format!("Invalid size: {}", *size);
        Err(err)?;
    }

    // Check for invalid file path
    if !img_path.exists() {
        eprintln!("File does not exist: {}", img_path.display());
        return Ok(());
    }

    // Check for invalid file
    let extension = img_path.extension();
    match extension {
        // Valid file name
        Some(ext) => {
            // Check file format
            let format = image::ImageFormat::from_extension(ext);
            match format {
                // Invalid format
                None => {
                    let err = format!("Invalid file type: {}", ext.to_str().expect("Failed to read file extension."));
                    Err(err)?;
                },
                // Valid format
                Some(img_form) => {
                    // Check if format can be read
                    if !img_form.can_read() {
                        let err = format!("Cannot read file type: {}", ext.to_str().expect("Failed to read file extension."));
                        Err(err)?;
                    }
                }
            }
        },
        // Invalid file name
        None => {
            let err = format!("Invalid file name: {}", img_path.display());
            Err(err)?;
        }
    }
    
    // Load file data
    let img_data = fs::read(img_path)?;
    let img_metadata = fs::metadata(img_path)?;
    let img_size = img_metadata.len();

    // Create path for new image
    // Get relative path for recursive case to match directory structure
    let relative_path = img_path.strip_prefix(base_dir)?.parent().expect("Failed to save to output folder");
    
    // Concatenate output directory with relative path and file name
    let mut new_path = out_dir.join(relative_path.join(img_path.file_name().expect("Tried to process a directory in shrink_img")));
    
    // Check whether the file is already small enough
    if img_size <= (*size).try_into().expect("Failed to compare size in shrink_img") {
        // If the user specifies to copy unchanged files to the output directory, copy the image
        if *copy {
            // If the specified directory doesn't exist, create it
            if !new_path.parent().unwrap().is_dir() {
                fs::create_dir_all(new_path.parent().unwrap())?;
            }
            fs::write(new_path, img_data)?;
        }
        return Ok(());
    }

    // If the specified directory doesn't exist, create it
    if !new_path.parent().unwrap().is_dir() {
        fs::create_dir_all(new_path.parent().unwrap())?;
    }

    // Convert to image type, checking whether it's an image 
    let img_data = load_image(img_path)?;

    // Compress image with worse quality iteratively 
    let mut quality = 100;
    while quality > 0 {
        // Compress the image
        let compressed_img_data = turbojpeg::compress_image(&img_data, quality, turbojpeg::Subsamp::Sub2x2)?;
        let new_name = new_path.display().to_string();
        let new_size = compressed_img_data.len();

        // If the image is small enough, save it 
        if new_size <= *size {
            // Set the file extension to JPG 
            match new_path.set_extension("JPG") {
                // Failed to set extension, throw an error
                false => {
                    let err = format!("Failed to save JPEG to {}", new_path.display());
                    Err(err)?;
                },
                // Succeeded in setting extension, save the image
                _ => {
                    fs::write(new_path, &compressed_img_data)?;
                    println!("Finished file:        {} with size {}", new_name, new_size);
                    return Ok(());
                }
            }
        }

        // Reduce the image quality
        quality -= 1;
    }

    // Throw an error if we couldn't make the image small enough
    let err = format!("Failed to shrink {}", img_path.display());
    Err(err)?
}

// load_image
// Helper function to load an image as an RgbImage using the image crate
// Parameters:
// path: Path to the image
// Return:
// ImageResult containing RgbImage if successful
fn load_image(path: &PathBuf) -> image::ImageResult<image::RgbImage> {
    let img = image::ImageReader::open(path)?.decode()?;
    Ok(img.into_rgb8())
}