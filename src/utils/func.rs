//! Useful functions
//! 
//! Function shared by Ascii, Block, Braile & Uniblock, allowing to:
//! 
//! + Colorize a string with given RGB foreground and background
//! + Stylize an ANSIString
//! + Convert RGB color to ansi Color
//! 
use crate::utils::{terminal_color};

use terminal_size::{terminal_size, Height, Width};

use ansi_term::Color::{self, Fixed, RGB};
use ansi_term::{ANSIString, Style};

/// Map an rgb color to a terminal color
///
/// returns the appropiate ansii_term Color variant
pub fn termcolor(r:u8, g:u8, b:u8) -> Color {
    let tcolor = terminal_color::TermColor::from(r, g, b)
                    .index;

    Fixed(tcolor)
}

/// Map an rgb color to a terminal true color
///
/// returns the appropiate ansii_term Color variant
pub fn rgbcolor(r:u8, g:u8, b:u8) -> Color {
    RGB(r,g,b)
}

/// Colorizes the string with a (24-bit) foreground and background color
pub fn colorize<'a>(ch: String, frgd: &Vec<u8>, bkgd: &Vec<u8>) -> ANSIString<'a> {
    /* Select appropiate style and fills the details */
    let style = match (frgd.is_empty(), bkgd.is_empty()) {
        (false, false) => RGB(frgd[0], frgd[1], frgd[2])
            .on(RGB(bkgd[0], bkgd[1], bkgd[2]))
            .paint(ch),
        (true, false) => RGB(255, 255, 255)
            .on(RGB(bkgd[0], bkgd[1], bkgd[2]))
            .paint(ch),
        (false, true) => RGB(frgd[0], frgd[1], frgd[2]).paint(ch),
        (true, true)  => Style::default().paint(ch),
    };

    style
}

/// Add ansi styles to a vector of ANSIString
pub fn stylize(ansistr: &mut Vec<ANSIString>, bold: bool, blink: bool, underline: bool) {
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

/// Get the size of current terminal and scale it or return 
///
/// Tries to get a scaled terminal dimensions, if not possible
/// returns the old dimensions
pub fn get_fullscreen_size(old_dimensions: (u32, u32), scale: (u32, u32)) -> (u32, u32) {
    let (img_w, img_h) = old_dimensions;
    let (scale_w, scale_h) = scale;

    /* Get apropiate image resize */
    let (width, height): (u32, u32) =
        if let Some((Width(w), Height(h))) = terminal_size() {
            (scale_w * w as u32, scale_h * h as u32)
        } else {
            (img_w, img_h)
        };

    (width, height)
}

/// Get the new dimensions, keeping aspect ratio of original if required
///
/// If new_dimensions = (0,0) returns original dimensions
/// If new_dimensions = (0,_) returns a dimension keeping aspect ratio and given height dimension
/// If new_dimensions = (0,_) returns a dimension keeping aspect ratio and given width dimension
/// If new_dimensions = (_,_) returns a new_dimension
pub fn get_actual_size(old_dimensions: (u32, u32), new_dimensions: (u32, u32)) -> (u32, u32) {
    let (img_w, img_h) = old_dimensions;
    let (new_width, new_height) = new_dimensions;

    /* Get aspect ratio of image */
    let aspect_ratio: f64 = img_w as f64 / img_h as f64;

    let (width, height) =
        match (new_width, new_height) {
            // Original image size
            (0, 0) => (img_w, img_h),
            // Keep aspect ratio of image but with specified height
            (0, _) => ( (aspect_ratio * new_height as f64) as u32, new_height),
            // Keep aspect ratio of image but with specified width
            (_, 0) => (new_width, (1.0 / aspect_ratio * new_width as f64) as u32),
            // Specified width and height
            (_, _) => (new_width, new_height),
        };

    (width, height)
}
