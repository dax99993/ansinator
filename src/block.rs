//! Image Block convertion
//!
//! Functions for image block convertion with the following features:
//!
//! + Half block unicode mode
//! + Whole block mode
//! + RGB coloring
//! + 256 Terminal Colors coloring

use crate::args::Block;
use crate::utils::{func};

use ansi_term::{ANSIString, ANSIStrings};

use image::imageops::FilterType;
use image::{GenericImageView, RgbImage};

use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Block {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();

        /* Get apropiate image resize */
        let img_dim = img.dimensions();
        let scale = if self.wholeblock {
            (1,1)
        }
        else {
            (1,2)
        };

        let (width, height): (u32, u32) = if self.fullscreen {
            func::get_fullscreen_size(img_dim, scale)
        } else {
            func::get_actual_size(img_dim, ( scale.0 * self.width, scale.1 * self.height))
        };

        /* Get selected resampling filter */
        let filter = 
        match &self.filter[..] {
            "CATMULLROM" => FilterType::CatmullRom,
            "GAUSSIAN" => FilterType::Gaussian,
            "LANCZOS" => FilterType::Lanczos3,
            "NEAREST" => FilterType::Nearest,
            "TRIANGLE" => FilterType::Triangle,
            _ => FilterType::Nearest,
        };


        /* Resize as needed with given filter */
        let img = img.resize_exact(width, height, filter);
        assert_eq!(img.dimensions(), (width, height));

        let img = img.adjust_contrast(self.contrast)
                         .brighten(self.brightness)
                         .into_rgb8();


        let f = 
        if self.termcolor {
            func::termcolor
        }
        else {
            func::rgbcolor
        };

        let mut ansistr: Vec<ANSIString> =
        /* Whole block mode - each pixel correspond to a single terminal colored cell */
        if self.wholeblock {
            rgb2whole(&img, f)
        } else {
            rgb2half(&img, f)
        };

        /* Add extra style */
        func::stylize(&mut ansistr, false, self.blink, false);

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
fn rgb2whole<'a, F>(img: &RgbImage, f: F) -> Vec<ANSIString<'a>> 
where
    F: Fn(u8, u8, u8) -> ansi_term::Color
{
    let mut ansistr: Vec<ANSIString> = vec![];

    for y in 0..img.height() {
        let mut color = f(0,0,0).on(f(0,0,0));
        for x in 0..img.width() {
            let r = img[(x, y)][0];
            let g = img[(x, y)][1];
            let b = img[(x, y)][2];

            let tcolor = f(r, g, b);
            let frgd = f(0, 0, 0);
            
            color = frgd.on(tcolor);

            ansistr.push(color.paint(" "));
        }
        ansistr.push(color.paint("\n"));
    }

    ansistr
}

/// Convert RGB image to a text representation using ansi (8-bit) or (24-bit) color,
/// mapping the two pixels of the image to a single terminal character block.
/// 
/// The mapping uses the upper pixel with the unicode upper block character and
/// the pixel below is mapped to a simple colored background.
fn rgb2half<'a, F>(img: &RgbImage, f: F) -> Vec<ANSIString<'a>> 
where
    F: Fn(u8, u8, u8) -> ansi_term::Color
{
    let upper_block = "\u{2580}";
    let mut ansistr: Vec<ANSIString> = vec![];

    /* Analize the image by a 1x2 windowing with half block mode */
    for y in (0..img.height() - 2).step_by(2) {
        let mut color = f(0,0,0).on(f(0,0,0));
        for x in 0..img.width() {
            /* Upper pixel color */
            let ur = img[(x, y)][0];
            let ug = img[(x, y)][1];
            let ub = img[(x, y)][2];

            /* Lower pixel color */
            let lr = img[(x, y + 1)][0];
            let lg = img[(x, y + 1)][1];
            let lb = img[(x, y + 1)][2];

            /* ansi Color*/
            let utcolor = f(ur, ug, ub);
            let ltcolor = f(lr, lg, lb);

            color = utcolor.on(ltcolor);

            ansistr.push(color.paint(upper_block));
        }
        ansistr.push(color.paint("\n"));
    }

    ansistr
}
