//! Image Braile convertion
//!
//! Functions for image ascii convertion with the following features:
//!
//! + Best fitting braile 8-dot character analysis 
//! + RGB coloring (fixed foreground and fixed background)
//! + Bold, Blink ansi styles


use crate::args::Braile;
use crate::utils::threshold::Threshold;

use ansi_term::Color::RGB;
use ansi_term::{ANSIString, ANSIStrings, Style};
use image::imageops::FilterType;
use image::{GenericImageView, GrayImage};
use terminal_size::{terminal_size, Height, Width};

use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Braile {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();
        let (img_w, img_h) = img.dimensions();

        /* Get aspect ratio of image */
        let aspect_ratio: f64 = img_w as f64 / img_h as f64;

        /* Get apropiate image resize */
        let (width, height): (u32, u32) = if self.fullscreen {
            if let Some((Width(w), Height(h))) = terminal_size() {
                (2 * w as u32, 4 * h as u32)
            } else {
                (img_w, img_h)
            }
        } else {
            match (self.width, self.height) {
                // Original image size
                (0, 0) => (img_w, img_h),
                // Keep aspect ratio of image but with specified height
                (0, _) => (
                    (aspect_ratio * 2.0 * self.height as f64) as u32,
                    4 * self.height,
                ),
                // Keep aspect ratio of image but with specified width
                (_, 0) => (
                    2 * self.width,
                    (4.0 / aspect_ratio * self.width as f64) as u32,
                ),
                // Specified width and height
                (_, _) => (self.width * 2, self.height * 4),
            }
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
        for y in (0..height - 4).step_by(4) {
            for x in (0..width - 2).step_by(2) {
                let ch = window_anaysis(&img, x, y)
                            .to_string();

                ansistr.push(colorize(ch, &self.frgdcolor, &self.bkgdcolor));
            }
            ansistr.push(Style::new().paint("\n"));
        }

        /* Add extra style */
        stylize(&mut ansistr, self.bold, self.blink);

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
/// | A | B |
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

/// Colorizes the string with a (24-bit) foreground and background color
fn colorize<'a>(ch: String, frgd: &Vec<u8>, bkgd: &Vec<u8>) -> ANSIString<'a> {
    /* Select appropiate style and fills the details */
    let style = match (frgd.is_empty(), bkgd.is_empty()) {
        (false, false) => RGB(frgd[0], frgd[1], frgd[2])
            .on(RGB(bkgd[0], bkgd[1], bkgd[2]))
            .paint(ch),
        (true, false) => RGB(255, 255, 255)
            .on(RGB(bkgd[0], bkgd[1], bkgd[2]))
            .paint(ch),
        (false, true) => RGB(frgd[0], frgd[1], frgd[2]).paint(ch),
        (true, true) => Style::default().paint(ch),
    };

    style
}

/// Add ansi styles to a vector of ANSIString
fn stylize(ansistr: &mut Vec<ANSIString>, bold: bool, blink: bool) {
    for v in ansistr {
        let style = v.style_ref_mut();
        match (blink, bold) {
            (false, false) => break,
            (false, true) => *style = (*style).bold(),
            (true, false) => *style = (*style).blink(),
            (true, true) => *style = (*style).bold().blink(),
        }
    }
}
