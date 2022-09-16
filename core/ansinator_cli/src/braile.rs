//! Image Braile convertion
//!
//! Functions for image ascii convertion with the following features:
//!
//! + Best fitting braile 8-dot character analysis 
//! + RGB coloring (fixed foreground and fixed background)
//! + Bold, Blink ansi styles

use crate::args::Braile;
use ansinator_ansi_image::{braile::AnsiBraile, ansi::Ansinator};
use ansinator_ansi_image::error::AnsiImageError;

//use std::error::Error;

//type MyResult<T> = Result<T, Box<dyn Error>>;
type MyResult<T> = Result<T, AnsiImageError>;

impl Braile {
    pub fn run(&self) -> MyResult<()> {
        let braile = AnsiBraile::new();
        /* Ansi style */
        
        let braile =
        if self.bold {
            braile.bold()
        } else {
            braile
        };
        let braile = 
        if self.blink {
            braile.blink()
        } else {
            braile
        };

        /* Color Mode */
        let braile = 
        if !self.frgdcolor.is_empty() {
            let r = self.frgdcolor[0];
            let g = self.frgdcolor[1];
            let b = self.frgdcolor[2];
            braile.set_foreground((r,g,b))
        } else {
            braile
        };
        let braile = 
        if !self.bkgdcolor.is_empty() {
            let r = self.bkgdcolor[0];
            let g = self.bkgdcolor[1];
            let b = self.bkgdcolor[2];
            braile.set_background((r,g,b))
        } else {
            braile
        };

        /* Set size */
        let braile = 
        if self.fullscreen {
            braile.fullscreen()
        } else {
            braile.size(self.width, self.height)
        };
        /* Selected resampling filter */
        let braile = braile.filter(&self.filter);
        /* Invert image colors */
        let braile = 
        if self.invert {
            braile.invert()
        } else {
            braile
        };
        /* Image transformations */
        let braile = braile.contrast(self.contrast);
        let braile = braile.brighten(self.brightness);

        /* Binarize Method manual threshold or automatic otsu's method */
        let braile = 
        if !self.threshold.is_empty() {
            braile.threshold(self.threshold[0])
        } else {
            braile.otsu_threshold()
        };

        /* Convert image to braile */
        //let ansi_output = braile.convert(&self.image).unwrap();
        let ansi_output = match braile.convert(&self.image) {
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
