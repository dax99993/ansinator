use std::fmt;

#[derive(Debug)]
pub enum AnsiImageError {
    FileError(std::io::Error),
    WriteError(std::io::Error),
    ImageError(image::ImageError),
}

impl fmt::Display for AnsiImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileError(e) =>  write!(f, "Error creating save file \"{}\"", e),
            Self::WriteError(e) =>  write!(f, "Error writing to save file \"{}\"", e),
            Self::ImageError(e) =>  write!(f, "Error opening image: \"{}\"", e),
        }
    }
}


