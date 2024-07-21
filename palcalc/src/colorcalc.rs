use std::path::Path;

use anyhow::Result;
use image::io::Reader as ImageReader;

pub struct ColorData(Vec<Vec<Vec<u64>>>);

impl ColorData {
    pub fn new() -> ColorData {
        ColorData(vec![vec![vec![0u64; 256]; 256]; 256])
    }

    pub fn add(&mut self, filename: &Path) -> Result<()> {
        let img = ImageReader::open(filename)?.decode()?.to_rgb8();
        for pixel in img.enumerate_pixels() {
            let color = pixel.2;
            self.0[color[0] as usize][color[1] as usize][color[2] as usize] += 1;
        }
        Ok(())
    }
}
