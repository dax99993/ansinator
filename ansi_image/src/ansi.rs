#![allow(dead_code, unused)]

use crate::error::AnsiImageError;

use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;
use ansi_term::{ANSIString, ANSIStrings, Style};
use terminal_size::{terminal_size, Height, Width};
use std::fs::File;
use std::io::Write;


#[derive(Debug)]
pub struct AnsiImageResult<'a> {
    pub data: Vec<ANSIString<'a>>,
}

#[derive(Debug)]
pub struct AnsiImage<T, S> {
    pub invert: bool,
    pub bold: bool,
    pub blink: bool,
    pub underline: bool,
    pub has_foreground: bool,
    pub has_background: bool,
    pub has_threshold: bool,
    pub foreground: (u8,u8,u8),
    pub background: (u8,u8,u8),
    pub threshold: u8,
    pub contrast: f32,
    pub brighten: i32,
    pub filter: FilterType,
    pub size: (u32, u32),
    pub scale: (u32, u32),
    pub color: S,
    pub mode: T,
}

pub trait Ansinator {
    fn new() -> Self;
    fn normal(&self) -> Self;

    fn bold(&self) -> Self;
    fn blink(&self) -> Self;
    fn underline(&self) -> Self;

    fn set_foreground(&self, foreground: (u8,u8,u8) ) -> Self;
    fn set_background(&self, background: (u8,u8,u8) ) -> Self;

    fn invert(&self) -> Self;
    fn brighten(&self, value: i32) -> Self;
    fn contrast(&self, value: f32) -> Self;
    fn filter(&self, filter: &str) -> Self;

    fn fullscreen(&self) -> Self;
    fn size(&self, x: u32, y: u32) -> Self;
}

impl<T, S> Ansinator for AnsiImage<T, S> 
where T: Default + Copy,
      S: Default + Copy,
{
    fn new() -> Self {
        Self { color: S::default(),
               mode: T::default(),
               invert: false,
               bold: false,
               blink: false,
               underline: false,
               has_foreground: false,
               has_background: false,
               has_threshold: false,
               foreground: (255,255,255),
               background: (0,0,0),
               threshold: 127,
               size: (0,0),
               scale: (1,1),
               contrast: 0.0,
               brighten: 0, 
               filter: FilterType::Nearest,
        }
    }

    fn bold(&self) -> Self {
        Self { bold: true, .. *self }
    }
    fn blink(&self) -> Self {
        Self { blink: true, .. *self }
    }
    fn underline(&self) -> Self {
        Self { underline: true, .. *self }
    }
    fn invert(&self) -> Self {
        Self { invert: true, .. *self }
    }
    fn normal(&self) -> Self {
        Self { invert: false,
                bold: false,
                blink: false,
                underline: false,
                has_foreground: false,
                has_background: false,
                has_threshold: false,
                .. *self 
        }
    }


    fn set_foreground(&self, foreground: (u8,u8,u8) ) -> Self {
        Self{ has_foreground: true, foreground, .. *self}
    }

    fn set_background(&self, background: (u8,u8,u8) ) -> Self {
        Self{ has_background: true, background, .. *self}
    }

    fn brighten(&self, value: i32) -> Self {
        Self { brighten: value, .. *self }
    }

    fn contrast(&self, value: f32) -> Self {
        Self { contrast: value, .. *self }
    }

    fn filter(&self, filter: &str) -> Self {
        let filter = 
        match filter {
            "CATMULLROM" => FilterType::CatmullRom,
            "GAUSSIAN" => FilterType::Gaussian,
            "LANCZOS" => FilterType::Lanczos3,
            "NEAREST" => FilterType::Nearest,
            "TRIANGLE" => FilterType::Triangle,
            _ => FilterType::Nearest,
        };

        Self { filter, .. *self }
    }

    fn fullscreen(&self) -> Self {
        /* Get terminal size if possible */
        let (width, height): (u32, u32) =
            if let Some((Width(w), Height(h))) = terminal_size() {
                (w as u32, h as u32)
            } else {
                self.size
            };

        /* Update size */
        self.size(width, height)
    }

    fn size(&self, x: u32, y: u32) -> Self {
        Self { size: (x,y), .. *self }
    } 

}

impl<T, S> AnsiImage<T, S> {

    /// Get the size, accounting aspect ratio of new dimensions
    ///
    /// If new_dimensions = (0,0) returns a new_dimensions
    /// If new_dimensions = (0,_) returns a dimension keeping aspect ratio and given height dimension
    /// If new_dimensions = (_,0) returns a dimension keeping aspect ratio and given width dimension
    /// If new_dimensions = (_,_) returns original dimensions

    pub fn size_aspect_ratio(&self, image_dimensions: (u32,u32)) -> (u32, u32) {

        /* Get aspect ratio of image */
        let (img_w, img_h) = image_dimensions;
        let aspect_ratio: f64 = img_w as f64 / img_h as f64;

        match self.size { 
            // Original image size
            (0, 0) => image_dimensions,
            // Keep aspect ratio of image with specified height
            (0, _) => ( (aspect_ratio * self.size.1 as f64) as u32,
                        self.size.1
                        ),
            // Keep aspect ratio of image with specified width
            (_, 0) => (self.size.0,
                       (1.0 / aspect_ratio * self.size.0 as f64) as u32
                       ),
            // Specified width and height
            (_, _) => self.size,
        }
    }

    pub fn image_resize_with_scale(&self, image: &DynamicImage) -> DynamicImage {
        /* update the size to account for aspect ratio */
        let size = self.size_aspect_ratio(image.dimensions());
        
        /* Calculate new image size for later convertion */
        let new_width = size.0 * self.scale.0;
        let new_height = size.1 * self.scale.1;
        assert_ne!(0, new_width);
        assert_ne!(0, new_height);

        /* Resize as needed with given filter */
        let image = image.resize_exact(new_width, new_height, self.filter);
        assert_eq!(image.dimensions(), (new_width, new_height));

        image
    }

}

impl<'a> AnsiImageResult<'a> {
    pub fn print(&self) {
        let a = ANSIStrings(&self.data);

        println!("{}", a);
    }

    pub fn save(&self, path: &str) -> Result<(),AnsiImageError> {
        //let mut output = File::create(&path).unwrap();
        //write!(output, "{}", ANSIStrings(&self.data)).unwrap();
        let mut output = match File::create(&path) {
            Ok(o) => o,
            Err(e) => return Err(AnsiImageError::FileError(e)),
        };
        match write!(output, "{}", ANSIStrings(&self.data)) {
            Ok(_) => Ok(()),
            Err(e) => return Err(AnsiImageError::WriteError(e)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn create() {

        let w = 2;
        let h = 3;
        let scale_w = 10;
        let scale_h = 5;
        
        let img = image::open("../images/pic1.jpg").unwrap()
                    .resize_exact(w*scale_w, h*scale_h, image::imageops::Nearest)
                    .into_luma8();

        let mut braile = AnsiImage::new();
        braile.bold();
        braile.underline();
        braile.set_frgd(vec![255,0,0]);
        //braile.set_bkgd(vec![0,255,255]);


        braile.print();

        braile.save("../test_uniblock.txt");
        assert_eq!(4, 4);
    }
    */
}
