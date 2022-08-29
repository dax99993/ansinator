//! Image Ascii convertion
//!
//! Functions for image ascii convertion with the following features:
//!
//! + Best fitting character analysis 
//! + RGB coloring
//! + 256 Terminal Colors coloring
//! + Bold, Blink and Underline ansi styles

use crate::args::Ascii;
use crate::utils::{ascii_font, func};

use ansi_term::{ANSIString, ANSIStrings};

use image::imageops::FilterType;
use image::{GenericImageView, GrayImage, RgbImage};

use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Ascii {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();

        /* Get apropiate image resize */
        let img_dim = img.dimensions();
        let scale = (5,7);

        let (width, height): (u32, u32) = if self.fullscreen {
            func::get_fullscreen_size(img_dim, scale)
        } else {
            func::get_actual_size(img_dim, (scale.0 * self.width, scale.1 * self.height))
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

        /* Apply image color transformations */
        let img = img.adjust_contrast(self.contrast)
                     .brighten(self.brightness);

        /* Resize as needed with given filter */
        let mut img = img.resize_exact(width, height, filter);
        assert_eq!(img.dimensions(), (width, height));

        
        /* Collect, order and remove duplicates from character set */
        let mut char_set = self.char_set.chars()
                              .map(|c| ascii_font::AsciiFont::from(c))
                              .collect::<Vec<ascii_font::AsciiFont>>();

        char_set.sort_unstable();
        char_set.dedup();

        //println!("{:?}", char_set);

        /* Invert image colors */
        if self.invert {
            img.invert();
        }

        /* Create Rgb and GrayImage */
        let luma = img.to_luma8();
        let rgb = img.resize_exact(width / scale.0, height / scale.1, filter)
                     .into_rgb8();

        /* Convert image to ascii */
        let mut ansistr: Vec<ANSIString> =
        if self.rgbcolor {
            //RGB 24bit fullcolor
            color2ascii(rgb, luma, &char_set, func::rgbcolor)
        }
        else if self.termcolor {
            //256 termcolor
            color2ascii(rgb, luma, &char_set, func::termcolor)
        } else {
            // nocolor
            luma2ascii(luma, &char_set, &self.frgdcolor, &self.bkgdcolor)
        };

        /* Add extra style */
        func::stylize(&mut ansistr, self.bold, self.blink, self.underline);

        let ansi_output = ANSIStrings(&ansistr);

        /*Print to stdout*/
        if !self.noecho {
            println!("{}", ansi_output);
        }

        /*Save to output file*/
        if !self.output.is_empty() {
            let mut output = File::create(&self.output[0])?;
            write!(output, "{}", ansi_output)?;
        }

        Ok(())
    }
}


/// Convert RGB image to a text representation using ansi (24-bit) true color or 256 terminal colors,
/// mapping the luma values of the image to the characters
/// in a given character set.
fn color2ascii<'a, F>(rgb: RgbImage, luma: GrayImage, character_set: &Vec<ascii_font::AsciiFont>, f: F) -> Vec<ANSIString<'a>>
where
    F: Fn(u8, u8, u8) -> ansi_term::Color
{
    let mut ansistr: Vec<ANSIString> = vec![];

    let (width, height) = rgb.dimensions();
    
    for y in 0..height {
        let mut color = f(0,0,0);
        for x in 0..width {
            let r = rgb[(x, y)][0];
            let g = rgb[(x, y)][1];
            let b = rgb[(x, y)][2];

            //Get character
            let ch = window_anaysis(&luma, x, y, character_set)
                        .to_string();

            //Create appropiate Color
            color = f(r,g,b);

            ansistr.push(color.paint(ch));
        }
        ansistr.push(color.paint("\n"));
    }

    ansistr
}


/// Convert Luma image to a text representation
/// mapping the luma values of the image to the characters
/// in a given character set.
fn luma2ascii<'a>(luma: GrayImage, character_set: &Vec<ascii_font::AsciiFont>, frgd: &Vec<u8>, bkgd: &Vec<u8>) -> Vec<ANSIString<'a>> {
    let mut ansistr: Vec<ANSIString> = vec![];

    let (width, height) = luma.dimensions();

    let width = width / 5;
    let height = height / 7;

    for y in 0..height {
        for x in 0..width {
            let ch = window_anaysis(&luma, x, y, character_set)
                        .to_string();

            ansistr.push(func::colorize(ch, &frgd, &bkgd));
        }
        ansistr.push(func::colorize('\n'.to_string(), &frgd, &bkgd));
    }

    ansistr
}

/// Analyze image with windows and calculate best fitting character
///
/// Perform a windowing analysis of the image with 5x7 windows, and 
/// calculate best fitting character from available vector of AsciiFont.
fn window_anaysis(img: &GrayImage, x: u32, y:u32, font_set: &Vec<ascii_font::AsciiFont>) -> char {
    let mut win = ascii_font::AsciiFont::default();
    for j in 0..7 {
        for i in 0..5 {
            let p = img[(5*x+i as u32, 7*y+j as u32)][0];
            win.font[5*j + i] = p; 
        }
    }
    let ch = ascii_font::minimize(&win, &font_set);
    
    ch
}
