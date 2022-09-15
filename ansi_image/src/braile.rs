#![allow(dead_code, unused)]

use crate::ansi::{AnsiImage, AnsiImageResult, Ansinator};
use crate::error::AnsiImageError;
use image::DynamicImage;
use image_window::{Windowing, GrayWindow, GrayImageWindow};
use image_binarize::Threshold;
use std::default::Default;
use ansi_term::Color;


#[derive(Debug, Clone, Copy)]
pub enum BraileColor {
    Fixed
}

impl Default for BraileColor {
   fn default() -> Self {
        Self::Fixed
   }
}

#[derive(Debug, Clone, Copy)]
pub enum BraileMode {
    ManualThreshold,
    OtsuThreshold,
}

impl Default for BraileMode {
   fn default() -> Self {
        Self::OtsuThreshold
   }
}

pub type AnsiBraile = AnsiImage<BraileMode, BraileColor>;

impl AnsiBraile {
    pub fn threshold(&self, value: u8) -> Self {
        Self { mode: BraileMode::ManualThreshold, has_threshold: true, threshold: value, scale: (2,4), .. *self}
    }
    pub fn otsu_threshold(&self) -> Self {
        Self { mode: BraileMode::OtsuThreshold, scale: (2,4), .. *self}
    } 

    pub fn get_color(&self) -> ansi_term::Style {
        let (r,g,b) = self.foreground;
        let (br,bg,bb) = self.background;
        match self.color {
            BraileColor::Fixed => {
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
    pub fn get_style(&self) -> ansi_term::Style {
        let mut style =  self.get_color();
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
        let image = self.image_resize_with_scale(&image);

        /* Cast image to luma */
        let mut luma = image.to_luma8();

        /* Binarize */
        match self.mode {
            BraileMode::ManualThreshold => {
                luma.threshold(self.threshold);
            },
            BraileMode::OtsuThreshold => {
                luma.otsu_threshold();
            }
        }
        /* Invert colors */
        if self.invert {
            luma.invert();
        }
        /* Convert to Windows */
        let luma_window = luma.into_window(2, 4).unwrap();

        /* Analyze windows and convert */
        let res = self.braile(luma_window);
        Ok(res)
    }

    /// Convert Gray image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// using braile characters.
    fn braile<'b>(&self, luma: GrayImageWindow) -> AnsiImageResult<'b> {

        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        /* Convert to appropiate color and style */
        let style = self.get_style();

        for luma_rows in luma.rows().iter() {
            for luma in luma_rows.iter() {
                /* Get window character */
                let ch = window_analysis(luma)
                            .to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style.paint("\n"));
        }
       
        ansi
    }
}


/// Perform a window analysis on the image to determine appropiate braile character
///
/// Calculate appropiate Braile 8-dot character offset
/// <https://en.wikipedia.org/wiki/Braille_Patterns>
///
/// Read the image with a 2x4 window starting on the
/// top-left coord (x,y)
///
/// The 8-dot cell represent each variation with the
/// following dot numbering
///
/// | C0| C1|
/// |---|---|
/// | 1 | 4 |
/// | 2 | 5 |
/// | 3 | 6 |
/// | 7 | 8 |
///
/// Each position represents a bit in a byte in little-endian order.
///
pub fn window_analysis(win: &GrayWindow) -> char {
    //assert(win.width == 2 && win.height == 4, "Just works for 2x4 windows") */

    let mut count = 0;
    count += (win.get_pixel(0, 0)[0] / 255) << 0;
    count += (win.get_pixel(0, 1)[0] / 255) << 1;
    count += (win.get_pixel(0, 2)[0] / 255) << 2;
    count += (win.get_pixel(1, 0)[0] / 255) << 3;
    count += (win.get_pixel(1, 1)[0] / 255) << 4;
    count += (win.get_pixel(1, 2)[0] / 255) << 5;
    count += (win.get_pixel(0, 3)[0] / 255) << 6;
    count += (win.get_pixel(1, 3)[0] / 255) << 7;

    let ch = get_braile(count);

    ch
}

/// Get the braile 8-dot character by means of the unicode offset
///
/// The 8 dot-cell codes start at the base address 0x2800
/// and each variation is an offset from the base address
/// 
fn get_braile(offset: u8) -> char {

    std::char::from_u32(offset as u32 + 0x2800).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_image_size() -> (u32, u32) {
        return (120,40)
    }
    fn setup_path() -> String {
        "../images/pic3.webp".to_string()
    }
    
    #[test]
    fn test_otsu_nocolor() {
        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let braile = AnsiBraile::new()
                            .bold()
                            .underline()
                            .otsu_threshold()
                            .size(w, h);

        println!("{:?}", braile);

        let result = braile.convert(&img)
                            .unwrap();

        result.print();

        result.save("../braile_otsu_nocolor.txt");
    }

    #[test]
    fn test_otsu_fixcolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let braile = AnsiBraile::new()
                            .bold()
                            .underline()
                            .otsu_threshold()
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .size(w, h);

        println!("{:?}", braile);

        let result = braile.convert(&img)
                            .unwrap();

        result.print();

        result.save("../braile_otsu_fixcolor.txt");
    }

    #[test]
    fn test_manual_nocolor() {
        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let braile = AnsiBraile::new()
                            .bold()
                            .underline()
                            .threshold(50)
                            .size(w, h);

        println!("{:?}", braile);

        let result = braile.convert(&img)
                            .unwrap();

        result.print();

        result.save("../braile_manual_nocolor.txt");
    }

    #[test]
    fn test_manual_fixcolor() {

        let (w,h) = setup_image_size();
        let path = setup_path();
        let img = image::open(path).unwrap();

        let braile = AnsiBraile::new()
                            .bold()
                            .underline()
                            .threshold(50)
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .size(w, h);

        println!("{:?}", braile);

        let result = braile.convert(&img)
                            .unwrap();

        result.print();

        result.save("../braile_manual_fixcolor.txt");
    }
}
