//! Image Uniblock (Sextant) convertion
//!
//! Functions for image uniblock (sextant) convertion with the following features:
//!
//! + Best fitting character analysis 
//! + RGB coloring (fixed foreground and fixed background)
//! + Bold and Blink ansi styles

use crate::args::Uniblock;
use ansi_image::{uniblock::AnsiUniblock, ansi::Ansinator, error::AnsiImageError};

//use std::error::Error;

//type MyResult<T> = Result<T, Box<dyn Error>>;
type MyResult<T> = Result<T, AnsiImageError>;

impl Uniblock {
    pub fn run(&self) -> MyResult<()> {
        let uniblock = AnsiUniblock::new();
        /* Ansi style */
        
        let uniblock =
        if self.bold {
            uniblock.bold()
        } else {
            uniblock
        };
        let uniblock = 
        if self.blink {
            uniblock.blink()
        } else {
            uniblock
        };

        /* Color Mode */
        let uniblock = 
        if !self.frgdcolor.is_empty() {
            let r = self.frgdcolor[0];
            let g = self.frgdcolor[1];
            let b = self.frgdcolor[2];
            uniblock.set_foreground((r,g,b))
        } else {
            uniblock
        };
        let uniblock = 
        if !self.bkgdcolor.is_empty() {
            let r = self.bkgdcolor[0];
            let g = self.bkgdcolor[1];
            let b = self.bkgdcolor[2];
            uniblock.set_background((r,g,b))
        } else {
            uniblock
        };

        /* Set size */
        let uniblock = 
        if self.fullscreen {
            uniblock.fullscreen()
        } else {
            uniblock.size(self.width, self.height)
        };
        /* Selected resampling filter */
        let uniblock = uniblock.filter(&self.filter);
        /* Invert image colors */
        let uniblock = 
        if self.invert {
            uniblock.invert()
        } else {
            uniblock
        };
        /* Image transformations */
        let uniblock = uniblock.contrast(self.contrast);
        let uniblock = uniblock.brighten(self.brightness);

        /* Binarize Method manual threshold or automatic otsu's method */
        let uniblock = 
        if !self.threshold.is_empty() {
            uniblock.threshold(self.threshold[0])
        } else {
            uniblock.otsu_threshold()
        };

        /* Convert image to uniblock */
       // let ansi_output = uniblock.convert(&self.image);
        let ansi_output = match uniblock.convert(&self.image) {
            Ok(a) => a,
            Err(e) => return Err(e),
        };

        /* Print to stdout */
        if !self.noecho {
            ansi_output.print();
        }

        /*Save to output file*/
        if !self.output.is_empty() {
            ansi_output.save(&self.output[0]).unwrap();
        }

        Ok(())
    }
}
