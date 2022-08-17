//! Image Uniblock (Sextant) convertion
//!
//! Functions for image uniblock (sextant) convertion with the following features:
//!
//! + Best fitting character analysis 
//! + RGB coloring (fixed foreground and fixed background)
//! + Bold and Blink ansi styles

use crate::args::Uniblock;
use crate::utils::threshold::Threshold;
use crate::utils::func;

use ansi_term::{ANSIString, ANSIStrings};

use image::imageops::FilterType;
use image::{GenericImageView, GrayImage};

use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Uniblock {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();

        /* Get apropiate image resize */
        let img_dim = img.dimensions();

        let (width, height): (u32, u32) = if self.fullscreen {
            func::get_fullscreen_size(img_dim, (2, 3))
        } else {
            func::get_actual_size(img_dim, (2 * self.width, 3 * self.height))
        };

        /* Get selected resampling filter */
        let mut filter = FilterType::Lanczos3;

        if self.filter_catmullrom {
            filter = FilterType::CatmullRom;
        }
        if self.filter_gaussian {
            filter = FilterType::Gaussian;
        }
        if self.filter_lanczos {
            filter = FilterType::Lanczos3;
        }
        if self.filter_nearest {
            filter = FilterType::Nearest;
        }
        if self.filter_triangle {
            filter = FilterType::Triangle;
        }

        /* Resize as needed with given filter */
        let img = img.resize_exact(width, height, filter);
        assert_eq!(img.dimensions(), (width, height));

        /* Apply image color transformations */
        let mut img = img.adjust_contrast(self.contrast)
                         .brighten(self.brightness)
                         .into_luma8();


        /* Binarize with manual threshold or automatic otsu's method */
        if !self.threshold.is_empty() {
            img.threshold(self.threshold[0]);
        } else {
            img.otsu_threshold();
        }

        /* Invert binarization if required */
        if self.invert {
            img.invert();
        }

        let mut ansistr: Vec<ANSIString> = vec![];

        /* Analize the image by a 2x4 windowing */
        for y in (0..height - 3).step_by(3) {
            for x in (0..width - 2).step_by(2) {
                let ch = window_anaysis(&img, x, y)
                            .to_string();

                ansistr.push(func::colorize(ch, &self.frgdcolor, &self.bkgdcolor));
            }
            ansistr.push(func::colorize('\n'.to_string(), &self.frgdcolor, &self.bkgdcolor));
        }

        /* Add extra style */
        func::stylize(&mut ansistr, self.bold, self.blink, false);

        /* Print to stdout*/
        if !self.noecho {
            println!("{}", ANSIStrings(&ansistr));
        }

        /* Save to output file*/
        if !self.output.is_empty() {
            let mut output = File::create(&self.output[0])?;
            write!(output, "{}", ANSIStrings(&ansistr))?;
        }

        Ok(())
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
fn window_anaysis(img: &GrayImage, x: u32, y: u32) -> char {

    let mut count = 0;
    count += (img[(x + 0, y + 0)][0] / 255) << 0;
    count += (img[(x + 1, y + 0)][0] / 255) << 1;
    count += (img[(x + 0, y + 1)][0] / 255) << 2;
    count += (img[(x + 1, y + 1)][0] / 255) << 3;
    count += (img[(x + 0, y + 2)][0] / 255) << 4;
    count += (img[(x + 1, y + 2)][0] / 255) << 5;

    let ch = get_sextant(count);
    
    ch
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
