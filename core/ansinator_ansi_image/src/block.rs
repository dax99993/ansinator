//! Representation of an image in block.
#![allow(dead_code, unused)]

use crate::ansi::{AnsiImage, AnsiImageResult, Ansinator};
use crate::error::AnsiImageError;
use image::{DynamicImage, GenericImageView};
use ansinator_image_window::{Windowing, RgbImageWindow};
use std::default::Default;
use ansi_term::Color;

#[derive(Debug, Clone, Copy)]
pub enum BlockColor{
    Truecolor,
    Terminalcolor,
}

impl Default for BlockColor {
   fn default() -> Self {
        Self::Truecolor
   }
}

#[derive(Debug, Clone, Copy)]
pub enum BlockMode{
    Whole,
    Half,
}

impl Default for BlockMode {
   fn default() -> Self {
        Self::Half
   }
}

pub type AnsiBlock = AnsiImage<BlockMode, BlockColor>;

impl AnsiBlock {
    pub fn true_color(&self) -> Self {
        Self { color: BlockColor::Truecolor, .. *self}
    }
    pub fn terminal_color(&self) -> Self {
        Self { color: BlockColor::Terminalcolor, .. *self}
    }
    pub fn half(&self) -> Self {
        Self { mode: BlockMode::Half, scale: (1,2), .. *self}
    }
    pub fn whole(&self) -> Self {
        Self { mode: BlockMode::Whole, scale: (1,1), .. *self}
    } 

    pub fn get_color(&self, r: u8, g:u8, b:u8, br:u8, bg:u8, bb: u8) -> ansi_term::Style {
        match self.color {
        BlockColor::Truecolor => {
           Color::RGB(r,g,b).on(Color::RGB(br,bg,bb))
        },
        BlockColor::Terminalcolor => {
            let frgd_index = ansinator_terminal_colors::TermColor::from(r, g, b)
                            .index;
            let bkgd_index = ansinator_terminal_colors::TermColor::from(br, bg, bb)
                            .index;
           Color::Fixed(frgd_index).on(Color::Fixed(bkgd_index))
        },
        }
    }
    pub fn get_style(&self, r:u8, g:u8, b:u8, br: u8, bg: u8, bb:u8) -> ansi_term::Style {
        let mut style =  self.get_color(r,g,b,br,bg,bb);
        if self.bold {
            style = style.bold()
        }
        if self.blink {
            style = style.blink()
        }
        if self.underline {
            style = style.underline()
        }

        style
    }

    pub fn convert(&self, image_path: &str) -> Result<AnsiImageResult, AnsiImageError> {
        /* Try opening the image */
        let image = match image::open(image_path) {
            Ok(image) => image,
            Err(e) => return Err(AnsiImageError::ImageError(e)),
        };
    
        /* Resize image to satisfy all internal parameters */
        let image = image.adjust_contrast(self.contrast)
                        .brighten(self.brighten);
        let mut image = self.image_resize_with_scale(&image);
        /* Invert colors */
        if self.invert {
            image.invert();
        }

        //let size = self.size_aspect_ratio(image.dimensions());
        /* Cast image to rgb */
        //let rgb = image.resize_exact(size.0, size.1, self.filter)
        let rgb = image.to_rgb8();

        let res =
        match self.mode {
            BlockMode::Half => {
                let rgb_window = rgb.to_window(1, 2).unwrap();
                self.convertion_half(rgb_window)
            },
            BlockMode::Whole => {
                let rgb_window = rgb.to_window(1, 1).unwrap();
                self.convertion_whole(rgb_window)
            },
        };
        Ok(res)
    }

    /// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// with a proportion of 1:1 image pixel : ansi character
    fn convertion_whole<'b>(&self, rgb: RgbImageWindow) -> AnsiImageResult<'b> {
        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Convert to appropiate color and style */
        let mut style = self.get_style(0,0,0,0,0,0);

        for rgb_rows in rgb.rows().iter() {

            for rgb in rgb_rows.iter() {
                /* Get RGB Color */
                let rgb_pixel = rgb.get_pixel(0,0);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];

                /* Convert to appropiate color and style */
                style = self.get_style(0,0,0,r,g,b);

                let ch = " ".to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style.paint("\n"));
        }
       
        ansi
    }

    /// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// with a proportion of 1:2 image width : image height for each ansi char
    fn convertion_half<'b>(&self, rgb: RgbImageWindow) -> AnsiImageResult<'b> {
        let upper_block = "\u{2580}";
        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Create initial style for later modification */
        let mut style = self.get_style(0,0,0,0,0,0);

        for rgb_rows in rgb.rows().iter() {
            for rgb in rgb_rows.iter() {
                /* Get RGB Color */
                let rgb_pixel = rgb.get_pixel(0,0);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];

                let lower_rgb_pixel = rgb.get_pixel(0,1);
                let br = lower_rgb_pixel[0];
                let bg = lower_rgb_pixel[1];
                let bb = lower_rgb_pixel[2];

                /* Convert to appropiate color and style */
                style = self.get_style(r,g,b,br,bg,bb);


                let ch = upper_block.to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style.paint("\n"));
        }
       
        ansi
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_image_size() -> (u32, u32) {
        return (120,40)
    }
    fn setup_path() -> String {
        "../images/pic2.png".to_string()
    }

    
    #[test]
    fn test_whole_truecolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let block = AnsiBlock::new()
                            .bold()
                            .underline()
                            .true_color()
                            .whole()
                            .size(w, h);

        println!("{:?}", block);

        let result = block.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../block_whole_truecolor.txt");
    }

    #[test]
    fn test_whole_terminalcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let block = AnsiBlock::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .whole()
                            .size(w, h);

        println!("{:?}", block);

        let result = block.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../block_whole_terminalcolor.txt");
    }
    #[test]
    fn test_half_truecolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let block = AnsiBlock::new()
                            .bold()
                            .underline()
                            .true_color()
                            .half()
                            .size(w, h);

        println!("{:?}", block);

        let result = block.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../block_half_truecolor.txt");
    }

    #[test]
    fn test_half_terminalcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let block = AnsiBlock::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .half()
                            .size(w, h);

        println!("{:?}", block);

        let result = block.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../block_half_terminalcolor.txt");
    }

}
