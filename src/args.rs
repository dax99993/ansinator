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
    /// Convert image to braile 8-dot representation
    Braile(Braile),
    /// Convert image to approximate low resolution blocks
    Block(Block),
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
    #[clap(short, long = "char-set")]
    #[clap(value_name = "CHAR SET")]
    #[clap(default_value_t = String::from(" .~*:+zM#&@$"))]
    pub char_set: String,
    /// Invert character set
    #[clap(short = 'i', long = "invert-char-set")]
    pub invert_char_set: bool,
    /// Use bold style
    #[clap(short = 'b', long)]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k', long)]
    pub blink: bool,
    /// Use underline style
    #[clap(short, long)]
    pub underline: bool,
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
    /// Resize image width
    /// [-W 0  keeps vertical aspect ratio]
    #[clap(short = 'W', long)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    #[clap(long = "filter-lanczos")]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool,
    #[clap(long = "filter-nearest")]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
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
    /// Invert image colors
    #[clap(short = 'i', long)]
    pub invert: bool,
    /// Use bold style
    #[clap(short = 'b', long)]
    pub bold: bool,
    /// Use blink style
    #[clap(short = 'k', long)]
    pub blink: bool,
    /// Set foreground color RGB [0-255 each channel]
    #[clap(short = 'F', long)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    pub frgdcolor: Vec<u8>,
    /// Set background color RGB [0-255 each channel]
    #[clap(short = 'B', long)]
    #[clap(number_of_values = 3)]
    #[clap(value_names = &["R", "G", "B"])]
    pub bkgdcolor: Vec<u8>,
    /// Resize image to fit in current terminal size
    #[clap(short, long)]
    pub fullscreen: bool,
    /// Prevent convertion from printing out to stdout
    #[clap(short, long)]
    pub noecho: bool,
    /// Set manual threshold image [0-255]
    #[clap(short = 't', long = "set-threshold")]
    pub threshold: Vec<u8>,
    /// Resize image width
    /// [-W 0  keeps horizontal aspect ratio]
    #[clap(short = 'W', long)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    #[clap(long = "filter-lanczos")]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool,
    #[clap(long = "filter-nearest")]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
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
    #[clap(long)]
    pub wholeblock: bool,
    /// Use 256 terminal colors (8-bit) color space
    #[clap(short, long)]
    pub termcolor: bool,
    /// Resize image width
    /// [-W 0  keeps horizontal aspect ratio]
    #[clap(short = 'W', long)]
    #[clap(default_value_t = 0)]
    pub width: u32,
    /// Resize image height
    /// [-H 0  keeps vertical aspect ratio]
    #[clap(short = 'H', long)]
    #[clap(default_value_t = 0)]
    pub height: u32,
    /// Perform resampling with catmullrom filter
    #[clap(long = "filter-catmullrom")]
    #[clap(conflicts_with_all = &["filter-gaussian", "filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_catmullrom: bool,
    #[clap(long = "filter-gaussian")]
    #[clap(conflicts_with_all = &["filter-lanczos", 
                                  "filter-nearest", "filter-triangle"])]
    pub filter_gaussian: bool,
    #[clap(long = "filter-lanczos")]
    #[clap(conflicts_with_all = &["filter-nearest", "filter-triangle"])]
    pub filter_lanczos: bool,
    #[clap(long = "filter-nearest")]
    #[clap(conflicts_with_all = &["filter-triangle"])]
    pub filter_nearest: bool,
    #[clap(long = "filter-triangle")]
    pub filter_triangle: bool,
}
