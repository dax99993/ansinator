//! Handle Clap Arguments
//!
//! Defines the following programs:
//!
//! + Ascii
//! + Braile
//! + Block
//! + Uniblock


use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author = "Dax99993 <Dax99993@gmail.com>",
    version,
    about = "Convert image to text representation using ansi escape sequence",
    long_about = "A collection of programs to convert images to various character representation,
as ascii, braile 8-dot, and a block representation, all supporting 
ansi coloring and certain styles like bold, blink or underline!."
)]
#[clap(propagate_version = true)]
pub struct AnsinatorArgs {
    #[clap(subcommand)]
    pub command: AnsinatorCommands,
}

#[derive(Debug, Subcommand)]
pub enum AnsinatorCommands {
    /// Convert image to ascii representation
    Ascii(Ascii),
    /// Convert image to approximate low resolution blocks
    Block(Block),
    /// Convert image to braile 8-dot representation
    Braile(Braile),
    /// Convert image to unicode blocks
    Uniblock(Uniblock),
}

#[derive(Debug, Args)]
pub struct Ascii {
    /// Input image
    pub image: String,
    /// Save convertion to file
    #[clap(short = 'o', long)]
    #[clap(value_name = "OUTPUT FILE")]
    pub output: Vec<String>,
    /// Use given character set for convertion
    /// (only ascii characters otherwise character is ignored)
    #[clap(short, long = "char-set", verbatim_doc_comment)]
    #[clap(value_name = "CHAR SET")]
    //#[clap(default_value_t = String::from(" .~*:+zM#&@$"))]
    #[clap(default_value_t = String::from(" !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"))]
    pub char_set: String,
    /// Invert image colors
    #[clap(short = 'i', long = "invert")]
    pub invert: bool,
    /// Use bold style
    #[clap(short = 'b', long)]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k', long)]
    pub blink: bool,
    /// Use underline style
    #[clap(short, long)]
    pub underline: bool,
    /// Set foreground color RGB
    /// [0-255 each channel]
    #[clap(short = 'F', long, verbatim_doc_comment)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    #[clap(conflicts_with = "termcolor")]
    #[clap(conflicts_with = "rgbcolor")]
    pub frgdcolor: Vec<u8>,
    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'B', long, verbatim_doc_comment)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    #[clap(conflicts_with = "termcolor")]
    #[clap(conflicts_with = "rgbcolor")]
    pub bkgdcolor: Vec<u8>,
    /// Use true color (24-bit) color space
    #[clap(short, long)]
    #[clap(conflicts_with = "termcolor")]
    pub rgbcolor: bool,
    /// Use 256 terminal colors (8-bit) color space
    #[clap(short, long)]
    pub termcolor: bool,
    //subpixel : bool,
    /// Resize image to fit in current terminal size
    #[clap(short, long)]
    pub fullscreen: bool,
    /// Prevent convertion from printing out to stdout
    #[clap(short, long)]
    pub noecho: bool,
    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',long = "set-contrast", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0.0)]
    pub contrast: f32,
    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S', long = "set-brightness", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0)]
    pub brightness: i32,
    /// Resize image width
    /// [-W 0  keeps vertical aspect ratio]
    #[clap(short = 'W', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    /// Perform resampling with gaussian filter
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    /// Perform resampling with lanczos filter
    /// [Lanczos is used by default] 
    #[clap(long = "filter-lanczos", verbatim_doc_comment)]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool, 
    /// Perform resampling with nearest filter
    #[clap(long = "filter-nearest")]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
    /// Perform resampling with triangle filter
    #[clap(long = "filter-triangle")]
    pub filter_triangle: bool,
}


#[derive(Debug, Args)]
pub struct Block {
    /// Input image
    pub image: String,
    /// Save convertion to file
    #[clap(short = 'o', long)]
    #[clap(value_name = "OUTPUT FILE")]
    pub output: Vec<String>,
    /// Use blink style
    #[clap(short = 'k', long)]
    pub blink: bool,
    /// Resize image to fit in current terminal size
    #[clap(short, long)]
    pub fullscreen: bool,
    /// Print out convertion to stdout
    #[clap(short, long)]
    pub noecho: bool,
    /// Use one terminal cell per image pixel
    #[clap(short, long)]
    pub wholeblock: bool,
    /// Use 256 terminal colors (8-bit) color space
    #[clap(short, long)]
    pub termcolor: bool,
    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',long = "set-contrast", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0.0)]
    pub contrast: f32,
    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S', long = "set-brightness", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0)]
    pub brightness: i32,
    /// Resize image width
    /// [-W 0  keeps horizontal aspect ratio]
    #[clap(short = 'W', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    /// Perform resampling with gaussian filter
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    /// Perform resampling with lanczos filter
    #[clap(long = "filter-lanczos")]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool, 
    /// Perform resampling with nearest filter
    /// [Nearest is used by default] 
    #[clap(long = "filter-nearest", verbatim_doc_comment)]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
    /// Perform resampling with triangle filter
    #[clap(long = "filter-triangle")]
    pub filter_triangle: bool,
}

#[derive(Debug, Args)]
pub struct Braile {
    /// Input image
    pub image: String,
    /// Save convertion to file
    #[clap(short = 'o', long)]
    #[clap(value_name = "OUTPUT FILE")]
    pub output: Vec<String>,
    /// Invert image luma colors
    #[clap(short = 'i', long)]
    pub invert: bool,
    /// Use bold style
    #[clap(short = 'b', long)]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k', long)]
    pub blink: bool,
    /// Set foreground color RGB
    /// [0-255 each channel]
    #[clap(short = 'F', long, verbatim_doc_comment)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    pub frgdcolor: Vec<u8>,
    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'B', long, verbatim_doc_comment)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    pub bkgdcolor: Vec<u8>,
    /// Resize image to fit in current terminal size
    #[clap(short, long)]
    pub fullscreen: bool,
    /// Prevent convertion from printing out to stdout
    #[clap(short, long)]
    pub noecho: bool,
    /// Set image threshold manually [0-255].
    /// If not set, then Otsu's binarization method is used.
    #[clap(short = 't', long = "set-threshold", verbatim_doc_comment)]
    pub threshold: Vec<u8>,
    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',long = "set-contrast", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0.0)]
    pub contrast: f32,
    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S', long = "set-brightness", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0)]
    pub brightness: i32,
    /// Resize image width
    /// [-W 0  keeps horizontal aspect ratio]
    #[clap(short = 'W', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    /// Perform resampling with gaussian filter
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    /// Perform resampling with lanczos filter
    /// [Lanczos is used by default] 
    #[clap(long = "filter-lanczos", verbatim_doc_comment)]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool, 
    /// Perform resampling with nearest filter
    #[clap(long = "filter-nearest")]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
    /// Perform resampling with triangle filter
    #[clap(long = "filter-triangle")]
    pub filter_triangle: bool,
}

#[derive(Debug, Args)]
pub struct Uniblock {
    /// Input image
    pub image: String,
    /// Save convertion to file
    #[clap(short = 'o', long)]
    #[clap(value_name = "OUTPUT FILE")]
    pub output: Vec<String>,
    /// Invert image luma colors
    #[clap(short = 'i', long)]
    pub invert: bool,
    /// Use bold style
    #[clap(short = 'b', long)]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k', long)]
    pub blink: bool,
    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'F', long, verbatim_doc_comment)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    pub frgdcolor: Vec<u8>,
    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'B', long, verbatim_doc_comment)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    pub bkgdcolor: Vec<u8>,
    /// Resize image to fit in current terminal size
    #[clap(short, long)]
    pub fullscreen: bool,
    /// Prevent convertion from printing out to stdout
    #[clap(short, long)]
    pub noecho: bool,
    /// Set image threshold manually [0-255]
    /// If not set, then Otsu's binarization method is used.
    #[clap(short = 't', long = "set-threshold", verbatim_doc_comment)]
    pub threshold: Vec<u8>,
    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',long = "set-contrast", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0.0)]
    pub contrast: f32,
    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S', long = "set-brightness", verbatim_doc_comment)]
    #[clap(allow_hyphen_values= true)]
    #[clap(default_value_t = 0)]
    pub brightness: i32,
    /// Resize image width
    /// [-W 0  keeps horizontal aspect ratio]
    #[clap(short = 'W', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long, verbatim_doc_comment)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    /// Perform resampling with gaussian filter
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    /// Perform resampling with lanczos filter
    /// [Lanczos is used by default] 
    #[clap(long = "filter-lanczos", verbatim_doc_comment)]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool, 
    /// Perform resampling with nearest filter
    #[clap(long = "filter-nearest")]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
    /// Perform resampling with triangle filter
    #[clap(long = "filter-triangle")]
    pub filter_triangle: bool,
}
