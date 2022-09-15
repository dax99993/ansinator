#![allow(dead_code, unused)]

use crate::ansi::{AnsiImage, AnsiImageResult, Ansinator};
use crate::error::AnsiImageError;
use image::{DynamicImage, GenericImageView};
use image_window::{Windowing, RgbWindow, RgbImageWindow, GrayWindow, GrayImageWindow};
use ascii_font::AsciiFont;
use std::default::Default;
use ansi_term::Color;

#[derive(Debug, Clone, Copy)]
pub enum AsciiColor {
    Truecolor,
    Terminalcolor,
    Fixed,
}

impl Default for AsciiColor {
   fn default() -> Self {
        Self::Fixed
   }
}

#[derive(Debug, Clone, Copy)]
pub enum AsciiMode {
    Gradient,
    Pattern,
}

impl Default for AsciiMode {
   fn default() -> Self {
        Self::Pattern
   }
}

pub type AnsiAscii = AnsiImage<AsciiMode, AsciiColor>;

impl AnsiAscii {
    pub fn true_color(&self) -> Self {
        Self { color: AsciiColor::Truecolor, .. *self}
    }
    pub fn terminal_color(&self) -> Self {
        Self { color: AsciiColor::Terminalcolor, .. *self}
    }
    fn set_foreground(&self, foreground: (u8,u8,u8) ) -> Self {
        Self{ has_foreground: true, foreground, color: AsciiColor::Fixed, .. *self}
    }
    fn set_background(&self, background: (u8,u8,u8) ) -> Self {
        Self{ has_background: true, background, color: AsciiColor::Fixed, .. *self}
    }

    pub fn gradient(&self) -> Self {
        Self { mode: AsciiMode::Gradient, scale: (1,1), .. *self}
    }
    pub fn pattern(&self) -> Self {
        Self { mode: AsciiMode::Pattern, scale: (5,7), .. *self}
    } 


    fn get_color(&self, r: u8, g:u8, b:u8) -> ansi_term::Style {
            match self.color {
            AsciiColor::Truecolor => {
               Color::RGB(r,g,b).normal()
            },
            AsciiColor::Terminalcolor => {
                let index = terminal_colors::TermColor::from(r, g, b)
                                .index;
               Color::Fixed(index).normal()
            },
            AsciiColor::Fixed => {
                match (self.has_foreground, self.has_background) {
                    (false, false) => {
                        ansi_term::Style::new()
                    },
                    (false, true) => {
                        Color::RGB(0,0,0).on(Color::RGB(r,g,b))
                    },
                    (true, false) => {
                        let (r, g, b) = self.foreground;
                        Color::RGB(r,g,b).normal()
                    },
                    (true, true) => {
                        let (r, g, b) = self.foreground;
                        let (br, bg, bb) = self.background;
                        Color::RGB(r,g,b).on(Color::RGB(br,bg,bb))
                    },
                }
            },
        }
    }
    pub fn get_style(&self, r:u8, g:u8, b:u8) -> ansi_term::Style {
        let mut style =  self.get_color(r,g,b);
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

    pub fn convert(&self, image_path: &str, char_set: &str) -> Result<AnsiImageResult, AnsiImageError>{

        /* Try opening the image */
        let image = match image::open(image_path) {
            Ok(image) => image,
            Err(e) => return Err(AnsiImageError::ImageError(e)),
        };

        /* Create font set */
        let mut font_set = char_set.chars()
                              .map(|c| AsciiFont::from(c))
                              .collect::<Vec<AsciiFont>>();

        /* Get requested size of image (without scaling!!) for later */
        let size = self.size_aspect_ratio(image.dimensions());

        /* Resize image to satisfy all internal parameters */
        let image = image.adjust_contrast(self.contrast)
                        .brighten(self.brighten);
        let mut image = self.image_resize_with_scale(&image);

        /* Invert colors */
        if self.invert {
            image.invert();
        }

        /* Cast to luma with scaled size */
        let luma = image.to_luma8();
        /* Cast image to rgb but resizing to keep proportion rgb:luma => (1:1) : (scale.0 : scale.1) 
         * by utilizing previously compute non scaled size
         * */ 
        let rgb = image.resize_exact(size.0, size.1, self.filter)
                        .to_rgb8();

        assert_eq!(rgb.width() * self.scale.0, luma.width());
        assert_eq!(rgb.height() * self.scale.1, luma.height());

        /* Convert to Window */
        let rgb_window = rgb.into_window(1, 1)
                            .unwrap();
        let luma_window = luma.into_window(self.scale.0, self.scale.1)
                            .unwrap();

        let res =
        match self.mode {
            AsciiMode::Gradient => {
                let map = luma_mapping;
                self.color2ascii(rgb_window, luma_window, &font_set, map)
            },
            AsciiMode::Pattern => {
                /* Dedup font set to increase convertion speed */
                font_set.sort_unstable();
                font_set.dedup();

                let map = window_analysis;
                self.color2ascii(rgb_window, luma_window, &font_set, map)
            },
        };

        Ok(res)
    }


    /// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// mapping the luma values of the image to the characters
    /// in a given character set.
    fn color2ascii<'b, G>(&self, rgb: RgbImageWindow, luma: GrayImageWindow, font_set: &Vec<AsciiFont>, map: G) -> AnsiImageResult<'b>
    where
        G: Fn(&GrayWindow, &Vec<AsciiFont>) -> char,
    {

        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Create initial style for later modification */
        let mut style = self.get_style(0,0,0);

        for (rgb_rows, luma_rows) in rgb.rows().iter().zip(luma.rows()) {
            for (rgb, luma) in rgb_rows.iter().zip(luma_rows) {
                assert!(rgb.width == 1 && rgb.height == 1, "Just works for 1x1 windows");
                /* Get RGB Color */
                let rgb_pixel = rgb.get_pixel(0,0);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];

                /* Convert to appropiate color and style */
                style = self.get_style(r,g,b);

                /* Get window character */
                let ch = map(&luma, &font_set)
                            .to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style.paint("\n"));
        }
       
        ansi
    }

}


/// Analyze image with windows and calculate best fitting character
///
/// Perform a windowing analysis of the image with 5x7 windows, and 
/// calculate best fitting character from available vector of AsciiFont.
fn window_analysis(win: &GrayWindow, font_set: &Vec<AsciiFont>) -> char {
    assert!(win.width == 5 && win.height == 7, "Just works for 5x7 windows");
    let mut font = AsciiFont::default();
    for index in 0..font.data.len() {
        font.data[index] = win.data[index][0]; 
    }
    let ch = ascii_font::minimize(&font, &font_set);
    
    ch
}

/// Map a luma value to a character in a vector of char
///
/// Linear mapping from [0-255] to [0-L], where L is the vector
/// of chars length.
fn luma_mapping(win: &GrayWindow, char_set: &Vec<AsciiFont>) -> char {
    assert!(win.width == 1 && win.height == 1, "Just works for 1x1 windows");
    let p = win.get_pixel(0,0)[0];
    let len = char_set.len();
    let index = p as usize * (len - 1) / 255;

    char_set[ index ].ch
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_image_size() -> (u32, u32) {
        return (120,50)
    }
    fn setup_path() -> String {
        "../images/pic5.jpg".to_string()
    }
    
    #[test]
    fn test_gradient_truecolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            //.true_color()
                            .gradient()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&img, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_gradient_truecolor.txt");
    }

    #[test]
    fn test_gradient_terminalcolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .gradient()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&img, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_gradient_terminalcolor.txt");
    }

    #[test]
    fn test_gradient_fixedcolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .set_foreground((255,0,255))
                            .set_background((0,255,255))
                            .gradient()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&img, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_gradient_fixedcolor.txt");
    }

    #[test]
    fn test_pattern_truecolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .true_color()
                            .pattern()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&img, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_truecolor.txt");
    }

    #[test]
    fn test_pattern_terminalcolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .pattern()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&img, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_terminalcolor.txt");
    }

    #[test]
    fn test_pattern_fixed() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .pattern()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&img, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_terminalcolor.txt");
    }
}
