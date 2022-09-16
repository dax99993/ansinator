//! ansi_image provides. 
//!
//! + AnsiImage: A general representation of an image in ansi.
//!
//!
//! + AnsiAscii: A representation of an image in ascii.
//! + AnsiBlock: A representation of an image in unicode half-block characters and spaces.
//! + AnsiBraile: A representation of an image in 8-dot Braile.
//! + AnsiUniblock: A representation of an image in unicode sextant characters.
pub mod ansi;
pub mod ascii;
pub mod braile;
pub mod block;
pub mod uniblock;
pub mod error;


