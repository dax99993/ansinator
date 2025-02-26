//! Representation of an image in ascii.

#![allow(dead_code, unused)]

use crate::ansi::{AnsiImage, AnsiImageResult, Ansinator};
use crate::error::AnsiImageError;
use ansinator_ascii_font::AsciiFont;
use image::{DynamicImage, GenericImageView, RgbImage, GrayImage};
use std::default::Default;
use ansi_term::Color;

/// Ascii coloring method
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

/// Ascii convertion method 
#[derive(Debug, Clone, Copy)]
pub enum AsciiMode {
    Gradient,
    PatternQuadrance,
    PatternSsim,
}

impl Default for AsciiMode {
   fn default() -> Self {
        Self::PatternQuadrance
   }
}

pub type AnsiAscii = AnsiImage<AsciiMode, AsciiColor>;

impl AnsiAscii {
    /// Coloring with true color (RGB8)
    pub fn true_color(&self) -> Self {
        Self { color: AsciiColor::Truecolor, .. *self}
    }
    /// Coloring with terminal colors (256 terminal color)
    pub fn terminal_color(&self) -> Self {
        Self { color: AsciiColor::Terminalcolor, .. *self}
    }
    /// Set fixed RGB foreground
    fn set_foreground(&self, foreground: (u8,u8,u8) ) -> Self {
        Self{ has_foreground: true, foreground, color: AsciiColor::Fixed, .. *self}
    }
    /// Set fixed RGB background 
    fn set_background(&self, background: (u8,u8,u8) ) -> Self {
        Self{ has_background: true, background, color: AsciiColor::Fixed, .. *self}
    }

    /// Set unicode gradient convertion mode
    pub fn gradient(&self) -> Self {
        Self { mode: AsciiMode::Gradient, scale: (1,1), .. *self}
    }
    /// Set ascii pattern (quadrance metric) convertion mode
    pub fn pattern_quadrance(&self) -> Self {
        Self { mode: AsciiMode::PatternQuadrance, scale: (5,7), .. *self}
    } 
    /// Set ascii pattern (structural similarity) convertion mode
    pub fn pattern_ssim(&self) -> Self {
        Self { mode: AsciiMode::PatternSsim, scale: (5,7), .. *self}
    } 

    /// get appropiate color for current convertion mode
    fn get_color(&self, r: u8, g:u8, b:u8) -> ansi_term::Style {
            match self.color {
            AsciiColor::Truecolor => {
               Color::RGB(r,g,b).normal()
            },
            AsciiColor::Terminalcolor => {
                let index = ansinator_terminal_colors::TermColor::from(r, g, b)
                                .index;
               Color::Fixed(index).normal()
            },
            AsciiColor::Fixed => {
                match (self.has_foreground, self.has_background) {
                    (false, false) => {
                        ansi_term::Style::new()
                    },
                    (false, true) => {
                        let (br, bg, bb) = self.background;
                        Color::RGB(0,0,0).on(Color::RGB(br,bg,bb))
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
    /// get appropiate color along style for current convertion mode
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

    /// Convert image file to ascii representation
    pub fn convert(&self, image_path: &str, char_set: &str) -> Result<AnsiImageResult, AnsiImageError>{

        /* Try opening the image */
        let image = match image::open(image_path) {
            Ok(image) => image,
            Err(e) => return Err(AnsiImageError::ImageError(e)),
        };


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

        let res =
        match self.mode {
            AsciiMode::Gradient => {
                let char_set = char_set.chars()
                                    .collect::<Vec<char>>();

                self.ascii_gradient(rgb, luma, &char_set)
            },
            AsciiMode::PatternQuadrance => {
                /* Create font set */
                let mut ascii_font_set = char_set.chars()
                                      .map(|c| AsciiFont::from(c))
                                      .collect::<Vec<AsciiFont>>();
                /* Dedup font set to increase convertion speed */
                ascii_font_set.sort_unstable();
                ascii_font_set.dedup();

                self.ascii_pattern_quadrance(rgb, luma, &ascii_font_set)
            },
            AsciiMode::PatternSsim => {
                /* Create font set */
                let mut ascii_font_set = char_set.chars()
                                      .map(|c| AsciiFont::from(c))
                                      .collect::<Vec<AsciiFont>>();
                /* Dedup font set to increase convertion speed */
                ascii_font_set.sort_unstable();
                ascii_font_set.dedup();

                self.ascii_pattern_ssim(rgb, luma, &ascii_font_set)
            },
        };

        Ok(res)
    }


    /// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// mapping the the pattern (quadrance metric) of a window of luma values to ascii
    /// in a given ascii character set.
    fn ascii_pattern_quadrance<'b>(&self, rgb: RgbImage, luma: GrayImage, font_set: &Vec<AsciiFont>) -> AnsiImageResult<'b> {
        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Create initial style for later modification */
        let mut style = self.get_style(0,0,0);
        let style_normal = ansi_term::Style::new();

        /* Get image dimensions */
        let width = rgb.width();
        let height = rgb.height();

        for y in (0..height) {
            for x in (0..width) {
                /* Get RGB Color */
                let rgb_pixel = rgb.get_pixel(x+0,y+0);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];

                /* Convert to appropiate color and style */
                style = self.get_style(r,g,b);

                /* Get window character */
                let ch = window_analysis_quadrance(&luma, x, y, &font_set)
                            .to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style_normal.paint("\n"));
        }
       
        ansi
    }

    /// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// mapping the the pattern (structural similarity metric) of a window of luma values to ascii
    /// in a given ascii character set.
    fn ascii_pattern_ssim<'b>(&self, rgb: RgbImage, luma: GrayImage, font_set: &Vec<AsciiFont>) -> AnsiImageResult<'b> {
        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Create initial style for later modification */
        let mut style = self.get_style(0,0,0);
        let style_normal = ansi_term::Style::new();

        /* Get image dimensions */
        let width = rgb.width();
        let height = rgb.height();

        for y in (0..height) {
            for x in (0..width) {
                /* Get RGB Color */
                let rgb_pixel = rgb.get_pixel(x+0,y+0);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];

                /* Convert to appropiate color and style */
                style = self.get_style(r,g,b);

                /* Get window character */
                let ch = window_analysis_ssim(&luma, x, y, &font_set)
                            .to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style_normal.paint("\n"));
        }
       
        ansi
    }

    /// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// mapping the luma values of the image to the characters
    /// in a given character set.
    fn ascii_gradient<'b>(&self, rgb: RgbImage, luma: GrayImage, char_set: &Vec<char>) -> AnsiImageResult<'b> {

        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Create initial style for later modification */
        let mut style = self.get_style(0,0,0);
        let style_normal = ansi_term::Style::new();

        /* Get image dimensions */
        let width = rgb.width();
        let height = rgb.height();

        for y in (0..height) {
            for x in (0..width) {
                /* Get window character */
                let ch = luma_mapping(&luma, x, y, &char_set)
                            .to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style_normal.paint("\n"));
        }
       
        ansi
    }

}


/// Analyze image with windows and calculate best fitting character (quadrance metric)
///
/// Perform a windowing analysis of the image with 5x7 windows, and 
/// calculate best fitting character from available vector of AsciiFont.
fn window_analysis_quadrance(win: &GrayImage, x:u32, y:u32, font_set: &Vec<AsciiFont>) -> char {
    let mut font = AsciiFont::default();
    for j in 0..7 {
        for i in 0..5 {
            let index = j*5 + i;
            /* Grayimage is 5:7 to rgb image (x,y) coords */
            font.data[index] = win.get_pixel(5*x + i as u32, 7*y + j as u32)[0]; 
        }
    }
    
    ansinator_ascii_font::minimize_quadrance(&font, &font_set)
}

/// Analyze image with windows and calculate best fitting character (structural similarity metric)
///
/// Perform a windowing analysis of the image with 5x7 windows, and 
/// calculate best fitting character from available vector of AsciiFont.
fn window_analysis_ssim(win: &GrayImage, x:u32, y:u32, font_set: &Vec<AsciiFont>) -> char {
    let mut font = AsciiFont::default();
    for j in 0..7 {
        for i in 0..5 {
            let index = j*5 + i;
            /* Grayimage is 5:7 to rgb image (x,y) coords */
            font.data[index] = win.get_pixel(5*x + i as u32, 7*y + j as u32)[0]; 
        }
    }
    
    ansinator_ascii_font::maximize_structural_similarity(&font, &font_set)
}

/// Map a luma value to a character in a vector of char
///
/// Linear mapping from [0-255] to [0-L], where L is the vector
/// of chars length.
fn luma_mapping(luma: &GrayImage, x:u32, y:u32, char_set: &Vec<char>) -> char {
    let p = luma.get_pixel(x+0,y+0)[0];
    let len = char_set.len();
    let index = p as usize * (len - 1) / 255;

    char_set[ index ]
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_image_size() -> (u32, u32) {
        return (120,50)
    }
    fn setup_path() -> String {
        "../../tests/images/pic5.jpg".to_string()
    }
    
    #[test]
    fn test_gradient_truecolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .true_color()
                            .gradient()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_gradient_truecolor.txt");
    }

    #[test]
    fn test_gradient_terminalcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .gradient()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_gradient_terminalcolor.txt");
    }

    #[test]
    fn test_gradient_fixedcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .set_foreground((255,0,255))
                            .set_background((0,255,255))
                            .gradient()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_gradient_fixedcolor.txt");
    }

    #[test]
    fn test_pattern_quadrance_truecolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .true_color()
                            .pattern_quadrance()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_quadrance_truecolor.txt");
    }

    #[test]
    fn test_pattern_ssim_truecolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .true_color()
                            .pattern_ssim()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_ssim_truecolor.txt");
    }

    #[test]
    fn test_pattern_quadrance_terminalcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .pattern_quadrance()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_quadrance_terminalcolor.txt");
    }

    #[test]
    fn test_pattern_ssim_terminalcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .terminal_color()
                            .pattern_ssim()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_ssim_terminalcolor.txt");
    }

    #[test]
    fn test_pattern_quadrance_fixed() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .pattern_quadrance()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_quadrance_terminalcolor.txt");
    }

    #[test]
    fn test_pattern_ssim_fixed() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let ascii = AnsiAscii::new()
                            .bold()
                            .underline()
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .pattern_ssim()
                            .size(w, h);

        println!("{:?}", ascii);

        let result = ascii.convert(&image_path, "012345789")
                            .unwrap();

        result.print();

        result.save("../ascii_pattern_ssim_terminalcolor.txt");
    }
}
