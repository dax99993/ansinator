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
    #[clap(short = 'o',
           long,
           value_name = "OUTPUT FILE",
    )]
    pub output: Vec<String>,

    /// Prevent convertion from printing out to stdout
    #[clap(short,
           long,
    )]
    pub noecho: bool,

    /// Use given character set for convertion
    /// (only ascii characters otherwise character is ignored)
    //#[clap(default_value_t = String::from(" .~*:+zM#&@$"))]
    #[clap(short,
           long = "char-set",
           verbatim_doc_comment,
           next_line_help = true,
           value_name = "CHAR SET",
           default_value_t = String::from(" !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~")
    )]
    pub char_set: String,


    /// Select character mode
    #[clap(short = 'm',
           long = "mode",
           verbatim_doc_comment,
           ignore_case = true,
           help_heading = "MODE",
           default_value = "PATTERN_QUADRANCE",
           value_parser = ["GRADIENT", "PATTERN_QUADRANCE", "PATTERN_SSIM", ],
    )]
    pub luma_mode: String,
    /// Use bold style
    #[clap(short = 'b', long,
           help_heading = "ANSI STYLES",
    )]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k',
           long,
           help_heading = "ANSI STYLES",
    )]
    pub blink: bool,
    /// Use underline style
    #[clap(short,
           long,
           help_heading = "ANSI STYLES",
    )]
    pub underline: bool,


    /// Set foreground color RGB
    /// [0-255 each channel]
    #[clap(short = 'F',
           long,
           verbatim_doc_comment,
           help_heading = "COLORING",
           number_of_values = 3,
           conflicts_with_all = &["termcolor", "rgbcolor"],
           value_names = &["R", "G", "B"],
    )]
    pub frgdcolor: Vec<u8>,

    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'B',
           long,
           verbatim_doc_comment,
           number_of_values = 3,
           help_heading = "COLORING",
           conflicts_with_all = &["termcolor", "rgbcolor"],
           value_names = &["R", "G", "B"],
    )]
    pub bkgdcolor: Vec<u8>,

    /// Use true color (24-bit) color space
    #[clap(short,
           long,
           help_heading = "COLORING",
           conflicts_with = "termcolor"
    )]
    pub rgbcolor: bool,

    /// Use 256 terminal colors (8-bit) color space
    #[clap(short,
           long,
           help_heading = "COLORING",
    )]
    pub termcolor: bool,


    /// Invert image colors
    #[clap(short = 'i',
           long = "invert",
           help_heading = "IMAGE PROCESSING",
    )]
    pub invert: bool,

    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',
           long = "set-contrast",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0.0
    )]
    pub contrast: f32,

    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S',
           long = "set-brightness",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0,
    )]
    pub brightness: i32,


    /// Resize image to fit in current terminal size
    #[clap(short,
           long,
           help_heading = "RESIZING",
    )]
    pub fullscreen: bool,
    /// Resize image width
    /// [-W 0  keeps horizontal aspect ratio]
    #[clap(short = 'W',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub width: u32,

    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub height: u32,


    /// Select resampling filter
    #[clap(short = 'R',
           long = "filter",
           verbatim_doc_comment,
           ignore_case = true,
           help_heading = "RESIZING",
           default_value = "LANCZOS",
           value_parser = ["CATMULLROM", "GAUSSIAN", "LANCZOS", "NEAREST", "TRIANGLE"],
    )]
    pub filter: String,
}


#[derive(Debug, Args)]
pub struct Block {
    /// Input image
    pub image: String,

    /// Save convertion to file
    #[clap(short = 'o',
           long,
           value_name = "OUTPUT FILE",
    )]
    pub output: Vec<String>,

    /// Prevent convertion from printing out to stdout
    #[clap(short,
           long,
    )]
    pub noecho: bool,

    /// Select character mode
    #[clap(short = 'm',
           long = "mode",
           verbatim_doc_comment,
           ignore_case = true,
           help_heading = "MODE",
           default_value = "HALF",
           value_parser = ["HALF", "WHOLE", ],
    )]
    pub block_mode: String,


    /// Use bold style
    #[clap(short = 'b', long,
           help_heading = "ANSI STYLES",
    )]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k',
           long,
           help_heading = "ANSI STYLES",
    )]
    pub blink: bool,


    /// Use 256 terminal colors (8-bit) color space
    /// 
    /// (otherwise utilizes true color (24-bit)
    #[clap(short,
           long,
           verbatim_doc_comment,
           help_heading = "COLORING",
    )]
    pub termcolor: bool,


    /// Invert image colors
    #[clap(short = 'i',
           long = "invert",
           help_heading = "IMAGE PROCESSING",
    )]
    pub invert: bool,

    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',
           long = "set-contrast",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0.0
    )]
    pub contrast: f32,

    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S',
           long = "set-brightness",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0,
    )]
    pub brightness: i32,


    /// Resize image to fit in current terminal size
    #[clap(short,
           long,
           help_heading = "RESIZING",
    )]
    pub fullscreen: bool,

    /// Resize image width
    /// [-W 0  keeps vertical aspect ratio]
    #[clap(short = 'W',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub width: u32,

    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub height: u32,


    /// Select resampling filter
    #[clap(short = 'R',
           long = "filter",
           ignore_case = true,
           help_heading = "RESIZING",
           default_value = "NEAREST",
           value_parser = ["CATMULLROM", "GAUSSIAN", "LANCZOS", "NEAREST", "TRIANGLE"],
    )]
    pub filter: String,
}

#[derive(Debug, Args)]
pub struct Braile {
    /// Input image
    pub image: String,


    /// Save convertion to file
    #[clap(short = 'o',
           long,
           value_name = "OUTPUT FILE",
    )]
    pub output: Vec<String>,

    /// Prevent convertion from printing out to stdout
    #[clap(short,
           long,
    )]
    pub noecho: bool,

    /// Set image threshold manually [0-255].
    /// If not set, then Otsu's binarization method is used.
    #[clap(short = 't',
           long = "set-threshold",
           verbatim_doc_comment,
    )]
    pub threshold: Vec<u8>,


    /// Use bold style
    #[clap(short = 'b', long,
           help_heading = "ANSI STYLES",
    )]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k',
           long,
           help_heading = "ANSI STYLES",
    )]
    pub blink: bool,


    /// Set foreground color RGB
    /// [0-255 each channel]
    #[clap(short = 'F',
           long,
           verbatim_doc_comment,
           help_heading = "COLORING",
           number_of_values = 3,
           value_names = &["R", "G", "B"],
    )]
    pub frgdcolor: Vec<u8>,

    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'B',
           long,
           verbatim_doc_comment,
           number_of_values = 3,
           help_heading = "COLORING",
           value_names = &["R", "G", "B"],
    )]
    pub bkgdcolor: Vec<u8>,


    /// Invert image luma colors
    #[clap(short = 'i',
           long = "invert",
           help_heading = "IMAGE PROCESSING",
    )]
    pub invert: bool,

    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',
           long = "set-contrast",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0.0
    )]
    pub contrast: f32,

    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S',
           long = "set-brightness",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0,
    )]
    pub brightness: i32,


    /// Resize image to fit in current terminal size
    #[clap(short,
           long,
           help_heading = "RESIZING",
    )]
    pub fullscreen: bool,

    /// Resize image width
    /// [-W 0  keeps vertical aspect ratio]
    #[clap(short = 'W',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub width: u32,

    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub height: u32,


    /// Select resampling filter
    #[clap(short = 'R',
           long = "filter",
           ignore_case = true,
           help_heading = "RESIZING",
           default_value = "LANCZOS",
           value_parser = ["CATMULLROM", "GAUSSIAN", "LANCZOS", "NEAREST", "TRIANGLE"],
    )]
    pub filter: String,
}

#[derive(Debug, Args)]
pub struct Uniblock {
    /// Input image
    pub image: String,


    /// Save convertion to file
    #[clap(short = 'o',
           long,
           value_name = "OUTPUT FILE",
    )]
    pub output: Vec<String>,

    /// Prevent convertion from printing out to stdout
    #[clap(short,
           long,
    )]
    pub noecho: bool,

    /// Set image threshold manually [0-255].
    /// If not set, then Otsu's binarization method is used.
    #[clap(short = 't',
           long = "set-threshold",
           verbatim_doc_comment,
    )]
    pub threshold: Vec<u8>,


    /// Use bold style
    #[clap(short = 'b', long,
           help_heading = "ANSI STYLES",
    )]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k',
           long,
           help_heading = "ANSI STYLES",
    )]
    pub blink: bool,


    /// Set foreground color RGB
    /// [0-255 each channel]
    #[clap(short = 'F',
           long,
           verbatim_doc_comment,
           help_heading = "COLORING",
           number_of_values = 3,
           value_names = &["R", "G", "B"],
    )]
    pub frgdcolor: Vec<u8>,

    /// Set background color RGB
    /// [0-255 each channel]
    #[clap(short = 'B',
           long,
           verbatim_doc_comment,
           number_of_values = 3,
           help_heading = "COLORING",
           value_names = &["R", "G", "B"],
    )]
    pub bkgdcolor: Vec<u8>,


    /// Invert image luma colors
    #[clap(short = 'i',
           long = "invert",
           help_heading = "IMAGE PROCESSING",
    )]
    pub invert: bool,

    /// Adjust the contrast of image. 
    /// Negative values decrease the contrast and positive values increase it.
    #[clap(short = 'C',
           long = "set-contrast",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0.0
    )]
    pub contrast: f32,

    /// Brighten the pixels of image.
    /// Negative values decrease the brightness and positive values increase it.
    #[clap(short = 'S',
           long = "set-brightness",
           verbatim_doc_comment,
           help_heading = "IMAGE PROCESSING",
           allow_hyphen_values= true,
           default_value_t = 0,
    )]
    pub brightness: i32,


    /// Resize image to fit in current terminal size
    #[clap(short,
           long,
           help_heading = "RESIZING",
    )]
    pub fullscreen: bool,

    /// Resize image width
    /// [-W 0  keeps vertical aspect ratio]
    #[clap(short = 'W',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub width: u32,

    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H',
           long,
           verbatim_doc_comment,
           help_heading = "RESIZING",
           default_value_t = 0,
    )]
    pub height: u32,


    /// Select resampling filter
    #[clap(short = 'R',
           long = "filter",
           ignore_case = true,
           help_heading = "RESIZING",
           default_value = "LANCZOS",
           value_parser = ["CATMULLROM", "GAUSSIAN", "LANCZOS", "NEAREST", "TRIANGLE"],
    )]
    pub filter: String,

}
