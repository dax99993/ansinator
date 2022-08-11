use crate::args::Block;
use crate::utils::terminal_color;

use ansi_term::Color::{Fixed, RGB};
use ansi_term::{ANSIString, ANSIStrings, Style};
use image::imageops::FilterType;
use image::{GenericImageView, RgbImage};
use terminal_size::{terminal_size, Height, Width};

use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Block {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();
        let (img_w, img_h) = img.dimensions();

        // Get aspect Ratio of image
        let aspect_ratio: f64 = img_w as f64 / img_h as f64;

        /* Get apropiate image resize */
        let (width, mut height): (u32, u32) = if self.fullscreen {
            if let Some((Width(w), Height(h))) = terminal_size() {
                (w as u32, h as u32)
            } else {
                (img_w, img_h)
            }
        } else {
            match (self.width, self.height) {
                // Original image size
                (0, 0) => (img_w, img_h),
                // Keep aspect ratio of image but with specified height
                (0, _) => ((aspect_ratio * self.height as f64) as u32, self.height),
                // Keep aspect ratio of image but with specified width
                (_, 0) => (self.width, (1.0 / aspect_ratio * self.width as f64) as u32),
                // Specified width and height
                (_, _) => (self.width, self.height),
            }
        };

        /* Need twice the height on half block mode */
        if !self.wholeblock {
            height *= 2;
        }

        /* Get selected resampling filter */
        let mut filter = FilterType::Nearest;

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

        let img = img.adjust_contrast(self.contrast)
                         .brighten(self.brightness)
                         .into_rgb8();

        let mut ansistr: Vec<ANSIString> = vec![];

        /* Whole block mode - each pixel correspond to a single terminal colored cell */
        if self.wholeblock {
            rgb2whole(&img, &mut ansistr, self.termcolor);
        } else {
            rgb2half(&img, &mut ansistr, self.termcolor);
        }

        /* Add extra style */
        if self.blink {
            for v in &mut ansistr {
                let style = v.style_ref_mut();
                *style = (*style).blink();
            }
        }

        /*Print to stdout*/
        if !self.noecho {
            println!("{}", ANSIStrings(&ansistr));
        }

        /*Save to output file*/
        if !self.output.is_empty() {
            let mut output = File::create(&self.output[0])?;
            write!(output, "{}", ANSIStrings(&ansistr))?;
        }

        Ok(())
    }
}

/// Convert RGB image to a text representation using ansi (8-bit) or (24-bit) color,
/// mapping the each pixel of the image to a single terminal character block
fn rgb2whole(img: &RgbImage, ansistr: &mut Vec<ANSIString>, termcolor: bool) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let r = img[(x, y)][0];
            let g = img[(x, y)][1];
            let b = img[(x, y)][2];

            if termcolor {
                let tcolor = terminal_color::minimize(r, g, b);

                ansistr.push(Fixed(0).on(Fixed(tcolor)).paint(" "));
            } else {
                ansistr.push(RGB(0, 0, 0).on(RGB(r, g, b)).paint(" "));
            }
        }
        ansistr.push(Style::default().paint("\n"));
    }
}

/// Convert RGB image to a text representation using ansi (8-bit) or (24-bit) color,
/// mapping the two pixels of the image to a single terminal character block
/* The mapping uses the upper pixel with the unicode upper block character and
 * the pixel below is mapped to a simple colored background */
fn rgb2half(img: &RgbImage, ansistr: &mut Vec<ANSIString>, termcolor: bool) {
    let upper_block = "\u{2580}";

    /* Analize the image by a 1x2 windowing with half block mode */
    for y in (0..img.height() - 2).step_by(2) {
        for x in 0..img.width() {
            /* Upper pixel color */
            let ur = img[(x, y)][0];
            let ug = img[(x, y)][1];
            let ub = img[(x, y)][2];

            /* Lower pixel color */
            let lr = img[(x, y + 1)][0];
            let lg = img[(x, y + 1)][1];
            let lb = img[(x, y + 1)][2];

            if termcolor {
                let utcolor = terminal_color::minimize(ur, ug, ub);
                let ltcolor = terminal_color::minimize(lr, lg, lb);

                ansistr.push(Fixed(utcolor).on(Fixed(ltcolor)).paint(upper_block));
            } else {
                ansistr.push(RGB(ur, ug, ub).on(RGB(lr, lg, lb)).paint(upper_block));
            }
        }
        ansistr.push(Style::default().paint("\n"));
    }
}
