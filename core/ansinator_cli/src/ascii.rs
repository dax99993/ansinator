//! Image Ascii convertion
//!
//! Functions for image ascii convertion with the following features:
//!
//! + Best fitting character analysis 
//! + RGB coloring
//! + 256 Terminal Colors coloring
//! + Bold, Blink and Underline ansi styles
//! + Gradient(unicode) and Pattern(ascii) convertion methods

use crate::args::Ascii;
use ansinator_ansi_image::{ascii::AnsiAscii, error::AnsiImageError, ansi::Ansinator};

//use std::error::Error;

//type MyResult<T> = Result<T, Box<dyn Error>>;
type MyResult<T> = Result<T, AnsiImageError>;

impl Ascii {
    pub fn run(&self) -> MyResult<()> {
        let ascii = AnsiAscii::new();
        /* Ansi style */
        
        let ascii =
        if self.bold {
            ascii.bold()
        } else {
            ascii
        };
        let ascii = 
        if self.blink {
            ascii.blink()
        } else {
            ascii
        };
        let ascii =
        if self.underline {
            ascii.underline()
        } else {
            ascii
        };

        /* Color Mode */
        let ascii = 
        if self.rgbcolor {
            ascii.true_color()
        } else {
            ascii
        };

        let ascii = 
        if self.termcolor {
            ascii.terminal_color()
        } else {
            ascii
        };

        let ascii = 
        if !self.frgdcolor.is_empty() {
            let r = self.frgdcolor[0];
            let g = self.frgdcolor[1];
            let b = self.frgdcolor[2];
            ascii.set_foreground((r,g,b))
        } else {
            ascii
        };
        let ascii = 
        if !self.bkgdcolor.is_empty() {
            let r = self.bkgdcolor[0];
            let g = self.bkgdcolor[1];
            let b = self.bkgdcolor[2];
            ascii.set_background((r,g,b))
        } else {
            ascii
        };

        /* Set size */
        let ascii = 
        if self.fullscreen {
            ascii.fullscreen()
        } else {
            ascii.size(self.width, self.height)
        };
        /* Selected resampling filter */
        let ascii = ascii.filter(&self.filter);
        /* Invert image colors */
        let ascii = 
        if self.invert {
            ascii.invert()
        } else {
            ascii
        };
        /* Image transformations */
        let ascii = ascii.contrast(self.contrast);
        let ascii = ascii.brighten(self.brightness);

        /* Convertion Method */        
        let ascii = 
        match &self.luma_mode[..] {
            "GRADIENT" => ascii.gradient(),
            "PATTERN_QUADRANCE" =>  ascii.pattern_quadrance(),
            "PATTERN_SSIM" =>  ascii.pattern_ssim(),
            _ =>  ascii.pattern_quadrance(),
        };


        /* Convert image to ascii */
        //let ansi_output = ascii.convert(&self.image, &self.char_set).unwrap();
        let ansi_output = match ascii.convert(&self.image, &self.char_set) {
            Ok(a) => a,
            Err(e) => return Err(e),
        };

        /* Print to stdout */
        if !self.noecho {
            ansi_output.print();
        }

        /*Save to output file*/
        if !self.output.is_empty() {
            if let Err(e) = ansi_output.save(&self.output[0]) {
                return Err(e);
            }
        }

        Ok(())
    }
}
