use crate::args::Ascii;
use crate::utils::terminal_color;

use ansi_term::Color::{Fixed, RGB};
use ansi_term::{ANSIString, ANSIStrings, Style};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
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
        let img = img.resize_exact(width, height, filter);
        assert_eq!(img.dimensions(), (width, height));

        /* Invert image colors if required */
        /* This option doesnt look that good xD */
        let mut char_set: Vec<char> = self.char_set.chars().collect();

        if self.invert_char_set {
            char_set.reverse();
        }

        //TODO
        //subpixel Analysis
        //add optional fixed foreground color and background color

        let mut ansistr: Vec<ANSIString> = vec![];

        if self.termcolor {
            //256 termcolor
            term2ascii(&img, &mut ansistr, &char_set);
        } else if self.rgbcolor {
            //RGB 24bit fullcolor
            rgb2ascii(&img, &mut ansistr, &char_set);
        } else {
            // nocolor
            luma2ascii(&img, &mut ansistr, &char_set);
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
fn rgb2ascii(img: &DynamicImage, ansistr: &mut Vec<ANSIString>, character_set: &Vec<char>) {
    let (width, height) = img.dimensions();
    let luma = img.to_luma8();
    let rgb = img.to_rgb8();

    for y in 0..height {
        for x in 0..width {
            let index: usize = (luma[(x, y)][0] as usize) * (character_set.len() - 1) / 0xFF;
            let r = rgb[(x, y)][0];
            let g = rgb[(x, y)][1];
            let b = rgb[(x, y)][2];

            ansistr.push(RGB(r, g, b).paint(character_set[index].to_string()));
        }
        ansistr.push(Style::default().paint("\n"));
    }
}

/// Convert RGB image to a text representation using ansi (8-bit) 256-color,
/// mapping the luma values of the image to the characters
/// in a given character set.
fn term2ascii(img: &DynamicImage, ansistr: &mut Vec<ANSIString>, character_set: &Vec<char>) {
    let (width, height) = img.dimensions();
    let luma = img.to_luma8();
    let rgb = img.to_rgb8();

    for y in 0..height {
        for x in 0..width {
            let index: usize = (luma[(x, y)][0] as usize) * (character_set.len() - 1) / 0xFF;
            let r = rgb[(x, y)][0];
            let g = rgb[(x, y)][1];
            let b = rgb[(x, y)][2];

            // Find best approximate terminal color
            let tcolor = terminal_color::minimize(r, g, b);

            let colorstr = Fixed(tcolor);
            ansistr.push(colorstr.paint(character_set[index].to_string()));
        }
        ansistr.push(Style::default().paint("\n"));
    }
}

/// Convert Luma image to a text representation
/// mapping the luma values of the image to the characters
/// in a given character set.
fn luma2ascii(img: &DynamicImage, ansistr: &mut Vec<ANSIString>, character_set: &Vec<char>) {
    let (width, height) = img.dimensions();
    let luma = img.to_luma8();

    for y in 0..height {
        for x in 0..width {
            let index: usize = (luma[(x, y)][0] as usize) * (character_set.len() - 1) / 0xFF;

            ansistr.push(Style::default().paint(character_set[index].to_string()));
        }
        ansistr.push(Style::default().paint("\n"));
    }
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
