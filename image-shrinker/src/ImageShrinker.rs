use std::{fs, path::PathBuf};
use turbojpeg::Compressor;

pub struct ImageShrinker;

impl ImageShrinker {
    pub fn shrink_dir(path: PathBuf, size: f64, recursive: bool) -> Result<Box <dyn Error>> {

    }

    pub fn shrink_img(path: PathBuf, size: f64) -> Result<Box <dyn Error>> {

    }
}