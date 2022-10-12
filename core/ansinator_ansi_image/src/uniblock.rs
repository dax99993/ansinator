//! Representation of an image in uniblock.
#![allow(dead_code, unused)]

use crate::ansi::{AnsiImage, AnsiImageResult, Ansinator};
use crate::error::AnsiImageError;
use ansinator_image_binarize::Threshold;
use image::{DynamicImage, GrayImage};
use std::default::Default;
use ansi_term::Color;

#[derive(Debug, Clone, Copy)]
pub enum UniblockColor {
    Fixed
}

impl Default for UniblockColor {
   fn default() -> Self {
        Self::Fixed
   }
}

#[derive(Debug, Clone, Copy)]
pub enum UniblockMode {
    ManualThreshold,
    OtsuThreshold,
}

impl Default for UniblockMode {
   fn default() -> Self {
        Self::OtsuThreshold
   }
}

pub type AnsiUniblock = AnsiImage<UniblockMode, UniblockColor>;

impl AnsiUniblock {
    pub fn threshold(&self, value: u8) -> Self {
        Self { mode: UniblockMode::ManualThreshold, has_threshold: true, threshold: value, scale: (2,3), .. *self}
    }
    pub fn otsu_threshold(&self) -> Self {
        Self { mode: UniblockMode::OtsuThreshold, scale: (2,3), .. *self}
    } 

    pub fn get_color(&self) -> ansi_term::Style {
        let (r,g,b) = self.foreground;
        let (br,bg,bb) = self.background;
        match self.color {
            UniblockColor::Fixed => {
                match (self.has_foreground, self.has_background) {
                    (false, false) => {
                        ansi_term::Style::new()
                    },
                    (false, true) => {
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
            UniblockMode::ManualThreshold => {
                luma.threshold(self.threshold);
            },
            UniblockMode::OtsuThreshold => {
                luma.otsu_threshold();
            }
        }
        /* Invert colors */
        if self.invert {
            luma.invert();
        }

        /* Analyze windows and convert */
        let res = self.uniblock(luma);
        Ok(res)
    }

    /// Convert Gray image to a text representation using ansi (24-bit) true color or 256 terminal colors,
    /// using sextant characters.
    fn uniblock<'b>(&self, luma: GrayImage) -> AnsiImageResult<'b> {

        /* Create Result */
        let mut ansi = AnsiImageResult{ data: vec![] };

        let style = self.get_style();
        let style_normal = ansi_term::Style::new();

        let width = luma.width();
        let height = luma.height();

        for y in (0..height).step_by(self.scale.1 as usize) {
            for x in (0..width).step_by(self.scale.0 as usize) {
                /* Get window character */
                let ch = window_analysis(&luma, x, y)
                            .to_string();

                /* Add ansi */
                ansi.data.push(style.paint(ch));
            }
            ansi.data.push(style_normal.paint("\n"));
        }
       
        ansi
    }
}



/// Perform a window analysis on the image to determine appropiate unicode
/// block sextant character offset
///
/// <https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing>
///
/// Read the image with a 2x3 window starting on the
/// top-left coord (x,y)
///
///  The block sextant represent each variation with the
///  following dot numbering
///
/// | C0| C1|
/// |---|---|
/// | 1 | 2 |
/// | 3 | 4 |
/// | 5 | 6 |
///
///  Each position represents a bit in a byte in little-endian order
///
fn window_analysis(win: &GrayImage, x:u32, y:u32) -> char {
    let mut count = 0;
    count += (win.get_pixel(x+0, y+0)[0] / 255) << 0;
    count += (win.get_pixel(x+1, y+0)[0] / 255) << 1;
    count += (win.get_pixel(x+0, y+1)[0] / 255) << 2;
    count += (win.get_pixel(x+1, y+1)[0] / 255) << 3;
    count += (win.get_pixel(x+0, y+2)[0] / 255) << 4;
    count += (win.get_pixel(x+1, y+2)[0] / 255) << 5;

    get_sextant(count)
}

/// Get the unicode block sextant character by means of the unicode offset
///
/// The 6-block cell codes start at the base address 0x1FB00
/// and each variation is an offset from the base address,
/// but theres no code for empty block nor left block nor right block nor full block
/// which correspond to offset 0, 21, 42 and 63 respectively
fn get_sextant(offset: u8) -> char {
    if offset == 0 {
        ' '
    }
    else if offset < 21 {
        std::char::from_u32(offset as u32 - 1 + 0x1FB00).unwrap()
    }
    else if offset == 21 {
       '\u{258C}' 
    }
    else if offset > 21 && offset < 42 {
        std::char::from_u32(offset as u32 - 22 + 0x1FB14).unwrap()
    }
    else if offset == 42 {
       '\u{258C}' 
    }
    else if offset > 42 && offset < 63 {
        std::char::from_u32(offset as u32 - 42 + 0x1FB27).unwrap()
    }
    else{
       '\u{2588}' 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_image_size() -> (u32, u32) {
        return (120,40)
    }
    fn setup_path() -> String {
        "../images/pic4.jpg".to_string()
    }
    
    #[test]
    fn test_otsu_nocolor() {
        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let uniblock = AnsiUniblock::new()
                            .bold()
                            .underline()
                            .otsu_threshold()
                            .size(w, h);

        println!("{:?}", uniblock);

        let result = uniblock.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../uniblock_otsu_nocolor.txt");
    }

    #[test]
    fn test_otsu_fixcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let uniblock = AnsiUniblock::new()
                            .bold()
                            .underline()
                            .contrast(250.0)
                            .otsu_threshold()
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .size(w, h);

        println!("{:?}", uniblock);

        let result = uniblock.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../uniblock_otsu_fixcolor.txt");
    }

    #[test]
    fn test_manual_nocolor() {
        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let uniblock = AnsiUniblock::new()
                            .bold()
                            .underline()
                            .threshold(50)
                            .size(w, h);

        println!("{:?}", uniblock);

        let result = uniblock.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../uniblock_manual_nocolor.txt");
    }

    #[test]
    fn test_manual_fixcolor() {

        let (w,h) = setup_image_size();
        let image_path = setup_path();

        let uniblock = AnsiUniblock::new()
                            .bold()
                            .underline()
                            .threshold(50)
                            .set_foreground((150,50,200))
                            .set_background((50,255,155))
                            .size(w, h);

        println!("{:?}", uniblock);

        let result = uniblock.convert(&image_path)
                            .unwrap();

        result.print();

        result.save("../uniblock_manual_fixcolor.txt");
    }
}
