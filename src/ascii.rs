//! Image Ascii convertion
//!
//! Functions for image ascii convertion with the following features:
//!
//! + Best fitting character analysis 
//! + RGB coloring
//! + 256 Terminal Colors coloring
//! + Bold, Blink and Underline ansi styles

use crate::args::Ascii;
use crate::utils::{terminal_color, ascii_font};

use ansi_term::Color::{Fixed, RGB};
use ansi_term::{ANSIString, ANSIStrings, Style};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, GrayImage};
use terminal_size::{terminal_size, Height, Width};


use std::error::Error;
use std::fs::File;
use std::io::Write;

type MyResult<T> = Result<T, Box<dyn Error>>;

impl Ascii {
    pub fn run(&self) -> MyResult<()> {
        let img = image::open(&self.image).unwrap();
        let (img_w, img_h) = img.dimensions();

        /* Get aspect ratio of image */
        let aspect_ratio: f64 = img_w as f64 / img_h as f64;

        /* Get apropiate image resize */
        let (width, height): (u32, u32) = if self.fullscreen {
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
        let img = img.resize_exact(5 * width, 7 * height, filter);
        assert_eq!(img.dimensions(), (5 * width, 7 * height));

        /* Apply image color transformations */
        let img = img.adjust_contrast(self.contrast)
                     .brighten(self.brightness);

        /* This option doesnt look that good xD */
        let mut char_set: Vec<ascii_font::AsciiFont> = self.char_set.chars().map(|c| ascii_font::AsciiFont::from(c)).collect();

        //println!("{:?}", char_set);

        if self.invert_char_set {
            char_set.reverse();
        }

        //TODO
        //subpixel Analysis

        let mut ansistr: Vec<ANSIString> = vec![];

        if self.termcolor {
            //256 termcolor
            term2ascii(img, &mut ansistr, &char_set);
        } else if self.rgbcolor {
            //RGB 24bit fullcolor
            rgb2ascii(img, &mut ansistr, &char_set);
        } else {
            // nocolor
            luma2ascii(img, &mut ansistr, &char_set, &self.frgdcolor, &self.bkgdcolor);
        }

        /* Add extra style */
        stylize(&mut ansistr, self.bold, self.blink, self.underline);

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

/// Convert RGB image to a text representation using ansi (24-bit) true color,
/// mapping the luma values of the image to the characters
/// in a given character set.
fn rgb2ascii(img: DynamicImage, ansistr: &mut Vec<ANSIString>, character_set: &Vec<ascii_font::AsciiFont>) {
    let (width, height) = img.dimensions();
    let width = width / 5;
    let height = height / 7;

    let luma = img.to_luma8();
    
    let img = img.resize_exact(width, height, FilterType::CatmullRom);
    let rgb = img.to_rgb8();

    for y in 0..height {
        for x in 0..width {
            let r = rgb[(x, y)][0];
            let g = rgb[(x, y)][1];
            let b = rgb[(x, y)][2];

            //Get character
            let ch = window_anaysis(&luma, x, y, character_set);

            ansistr.push(RGB(r, g, b).paint(ch.to_string()));
        }
        ansistr.push(Style::default().paint("\n"));
    }
}

/// Convert RGB image to a text representation using ansi (8-bit) 256-color,
/// mapping the luma values of the image to the characters
/// in a given character set.
fn term2ascii(img: DynamicImage, ansistr: &mut Vec<ANSIString>, character_set: &Vec<ascii_font::AsciiFont>) {
    let (width, height) = img.dimensions();
    let width = width / 5;
    let height = height / 7;

    let luma = img.to_luma8();
    
    let img = img.resize_exact(width, height, FilterType::CatmullRom);
    let rgb = img.to_rgb8();

    for y in 0..height {
        for x in 0..width {
            let r = rgb[(x, y)][0];
            let g = rgb[(x, y)][1];
            let b = rgb[(x, y)][2];

            //Get character
            let ch = window_anaysis(&luma, x, y, character_set);

            // Find best approximate terminal color
            let tcolor = terminal_color::TermColor::from(r, g, b)
                            .index;

            let colorstr = Fixed(tcolor);
            ansistr.push(colorstr.paint(ch.to_string()));
        }
        ansistr.push(Style::default().paint("\n"));
    }
}

/// Convert Luma image to a text representation
/// mapping the luma values of the image to the characters
/// in a given character set.
fn luma2ascii(img: DynamicImage, ansistr: &mut Vec<ANSIString>, character_set: &Vec<ascii_font::AsciiFont>, frgd: &Vec<u8>, bkgd: &Vec<u8>) {
    let (width, height) = img.dimensions();
    let luma = img.into_luma8();

    let width = width / 5;
    let height = height / 7;

    for y in 0..height {
        for x in 0..width {
            let ch = window_anaysis(&luma, x, y, character_set).to_string();

            ansistr.push(colorize(ch, &frgd, &bkgd));
        }
        ansistr.push(Style::default().paint("\n"));
    }
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

/// Add ansi styles to a vector of ANSIString
fn stylize(ansistr: &mut Vec<ANSIString>, bold: bool, blink: bool, underline: bool) {
    for v in ansistr {
        let style = v.style_ref_mut();
        match (bold, blink, underline) {
            (false, false, false) => break,
            (false, false, true) => *style = (*style).underline(),
            (false, true, false) => *style = (*style).blink(),
            (false, true, true) => *style = (*style).blink().underline(),
            (true, false, false) => *style = (*style).bold(),
            (true, false, true) => *style = (*style).bold().underline(),
            (true, true, false) => *style = (*style).bold().blink(),
            (true, true, true) => *style = (*style).bold().blink().underline(),
        }
    }
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


