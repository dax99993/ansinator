//! Image Block convertion
//!
//! Functions for image block convertion with the following features:
//!
//! + Half block unicode mode
//! + Whole block mode
//! + RGB coloring
//! + 256 Terminal Colors coloring

use crate::args::Block;
use ansinator_ansi_image::{block::AnsiBlock, ansi::Ansinator};
use ansinator_ansi_image::error::AnsiImageError;

//use std::error::Error;

//type MyResult<T> = Result<T, Box<dyn Error>>;
type MyResult<T> = Result<T, AnsiImageError>;

impl Block {
    pub fn run(&self) -> MyResult<()> {

        let block = AnsiBlock::new();
        /* Ansi style */
        
        /*
        let block =
        if self.bold {
            block.bold()
        } else {
            block
        };
        */
        let block = 
        if self.blink {
            block.blink()
        } else {
            block
        };

        /* Color Mode */
        let block = 
        if self.termcolor {
            block.terminal_color()
        } else {
            block.true_color()
        };

        /* Set size */
        let block = 
        if self.fullscreen {
            block.fullscreen()
        } else {
            block.size(self.width, self.height)
        };
        /* Selected resampling filter */
        let block = block.filter(&self.filter);
        /* Image transformations */
        let block = block.contrast(self.contrast);
        let block = block.brighten(self.brightness);

        /* Convertion Method */        
        let block = 
        match &self.block_mode[..] {
            "HALF" => block.half(),
            "WHOLE" =>  block.whole(),
            _ =>  block.half(),
        };


        /* Convert image to block */
        //let ansi_output = block.convert(&self.image).unwrap();
        let ansi_output = match block.convert(&self.image) {
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
