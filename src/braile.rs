//! Image Braile convertion
//!
//! Functions for image ascii convertion with the following features:
//!
//! + Best fitting braile 8-dot character analysis 
//! + RGB coloring (fixed foreground and fixed background)
//! + Bold, Blink ansi styles

use crate::args::Braile;
use crate::utils::threshold::Threshold;
use crate::utils::func;

use ansi_term::{ANSIString, ANSIStrings};

use image::imageops::FilterType;
use image::{GenericImageView, GrayImage};

use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Braile {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();

        /* Get apropiate image resize */
        let img_dim = img.dimensions();

        let (width, height): (u32, u32) = if self.fullscreen {
            func::get_fullscreen_size(img_dim, (2, 4))
        } else {
            func::get_actual_size(img_dim, (2 * self.width, 4 * self.height))
        };

        /* Get selected resampling filter */
        let filter = 
        match &self.filter[..] {
            "CATMULLROM" => FilterType::CatmullRom,
            "GAUSSIAN" => FilterType::Gaussian,
            "LANCZOS" => FilterType::Lanczos3,
            "NEAREST" => FilterType::Nearest,
            "TRIANGLE" => FilterType::Triangle,
            _ => FilterType::CatmullRom,
        };

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
        for y in (0..height - 4).step_by(4) {
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
fn window_anaysis(img: &GrayImage, x: u32, y: u32) -> char {

    let mut count = 0;
    count += (img[(x + 0, y + 0)][0] / 255) << 0;
    count += (img[(x + 0, y + 1)][0] / 255) << 1;
    count += (img[(x + 0, y + 2)][0] / 255) << 2;
    count += (img[(x + 1, y + 0)][0] / 255) << 3;
    count += (img[(x + 1, y + 1)][0] / 255) << 4;
    count += (img[(x + 1, y + 2)][0] / 255) << 5;
    count += (img[(x + 0, y + 3)][0] / 255) << 6;
    count += (img[(x + 1, y + 3)][0] / 255) << 7;

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
