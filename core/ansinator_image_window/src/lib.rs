//! ImageWindow. 
//!
//! Provide the Windowing Trait for ImageBuffer, implementing:
//! + Spliting ImageBuffer into windows
//!
//! Window struct implementing:
//! + Unchecked access to Window pixels 
//! + Checked access to Window pixels 
#![allow(dead_code)]

use image::{Pixel, ImageBuffer, Luma, Rgb};
use std::marker::PhantomData;
use std::ops::Deref;
use std::iter::Iterator;

pub type RgbWindow = Window<Rgb<u8>>;
pub type GrayWindow = Window<Luma<u8>>;
pub type RgbImageWindow = ImageWindow<Rgb<u8>, Vec<u8>>;
pub type GrayImageWindow = ImageWindow<Luma<u8>, Vec<u8>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Window<P> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<P>,
}

impl<P> Window<P> {

    /// Gets a reference to the pixel at location (x, y)
    ///
    /// # Panics
    ///
    /// Panics if `(x, y)` is out of the bounds `(width, height)`.
    pub fn get_pixel(&self, x: u32, y:u32) -> &P {
        assert!(x < self.width && y < self.height, 
                "Image index {:?} out of bounds {:?}",
                (x, y),
                (self.width, self.height)
                );
        &self.data[x as usize * (self.width * y) as usize]
    }

    /// Gets a reference to the pixel at location `(x, y)` or returns `None` if
    /// the index is out of the bounds `(width, height)`.
    pub fn get_pixel_checked(&self, x: u32, y:u32) -> Option<&P> {
        if x < self.width && y < self.height {
            Some(self.get_pixel(x,y))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ImageWindow<P: Pixel, Container> {
    windows_per_row: u32,
    windows_per_col: u32,
    image_width: u32,
    image_height: u32,
    _phantom: PhantomData<P>,
    _phantom1: PhantomData<Container>,
    pub windows: Vec<Window<P>>,
}

/// Methods to split an image into windows for individual analysis.
pub trait Windowing<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    /// Split this image into windows of given `width` and `height`. Returns an ImageWindow 
    /// or returns `None` if
    /// the image dimensions are not exactly divisible into `width` and `height` windows
    /// or 
    /// the windows `width` and `height` are bigger than image dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of windows.
    /// * `height` - Height of windows 
    ///
    /// # Examples
    ///
    /// ```
    /// use image::imageops::FilterType;
    ///
    /// let width: u32 = 8;
    /// let height: u32 = 12;
    ///
    /// let image = image::open("test.jpg").unwrap()
    ///                 .resize(width * 100, height * 70, FilterType::Nearest)
    ///                 .into_luma8();
    ///
    /// let img_win = image.to_window(width, height).unwrap();
    /// ```
    fn to_window_exact(self, width: u32, height: u32) -> Option<ImageWindow<P, Container>>;
    fn to_window(self, width: u32, height: u32) -> Option<ImageWindow<P, Container>>;
}


impl<P, Container> Windowing<P, Container>  for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    /// Split this image into windows of given `width` and `height`. Returns an ImageWindow 
    /// or returns `None` if
    /// the image dimensions are not exactly divisible into `width` and `height` windows
    /// or 
    /// the windows `width` and `height` are bigger than image dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of windows.
    /// * `height` - Height of windows 
    ///
    /// # Examples
    ///
    /// ```
    /// use image::imageops::FilterType;
    ///
    /// let width: u32 = 8;
    /// let height: u32 = 12;
    ///
    /// let image = image::open("test.jpg").unwrap()
    ///                 .resize(width * 100, height * 70, FilterType::Nearest)
    ///                 .into_luma8();
    ///
    /// let img_win = image.to_window(width, height).unwrap();
    /// ```
    fn to_window_exact(self, width: u32, height: u32) -> Option<ImageWindow<P, Container>> {
        let image_width = self.width();
        let image_height = self.height();

        /* Verify window size is smaller than actual image size */
        if width <= image_width && height <= image_height {
            let mut windows = vec![];
            for y in (0..image_height).step_by(height as usize) {
                for x in (0..image_width).step_by(width as usize) {
                    let mut data = vec![];
                    for j in 0..height {
                        for i in 0..width {
                            let pixel = match self.get_pixel_checked(x+i,y+j) {
                               Some(p) => p.clone(), 
                               None => return None,
                            };
                            data.push(pixel);
                        }
                    }
                    windows.push( Window { width, height, data });
                }
            }
            let windows_per_row = (image_width) / width;
            
            Some(ImageWindow {
                windows_per_col: windows.len() as u32 / windows_per_row,
                windows_per_row,
                image_width,
                image_height,
                _phantom: PhantomData,
                _phantom1: PhantomData,
                windows,
            })
        } else {
            None
        }
    }

    /// Split this image into windows of given `width` and `height`. Returns an ImageWindow
    /// or returns None if
    /// the windows `width` and `height` are bigger than image dimensions.
    ///
    /// The original size might change
    /// at most `width - 1` columns and `height - 1` rows in order to fit the windows size.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of windows.
    /// * `height` - Height of windows 
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let image = image::open("test.jpg").unwrap()
    ///                 .into_rgb8();
    ///
    /// let width: u32 = 8;
    /// let height: u32 = 12;
    ///
    /// let img_win = image.to_window(width, height).unwrap();
    /// ```
    fn to_window(self, width: u32, height: u32) -> Option<ImageWindow<P, Container>> {
        let image_width = self.width();
        let image_height = self.height();

        /* Verify window size is smaller than actual image size */
        if width <= image_width && height <= image_height {
            let mut windows = vec![];
            for y in (0..image_height - height).step_by(height as usize) {
                for x in (0..image_width - width).step_by(width as usize) {
                    //println!("{:?}", (x,y));
                    let mut data = vec![];
                    for j in 0..height {
                        for i in 0..width {
                            data.push(self.get_pixel(x+i,y+j).clone());
                        }
                    }
                    windows.push( Window { width, height, data });
                }
            }
            let windows_per_row = (image_width - width) / width;
            
            Some(ImageWindow {
                windows_per_col: windows.len() as u32 / windows_per_row,
                windows_per_row,
                image_width,
                image_height,
                _phantom: PhantomData,
                _phantom1: PhantomData,
                windows,
            })
        } else {
            None
        }
    }

}

impl<P, Container> ImageWindow<P, Container> 
where 
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    /// Returns an vector containing the all the windows,
    /// that fit in a row of the original image width, independent of the
    /// window height.
    ///
    /// # Examples
    ///
    /// ```
    /// use image::imageops::FilterType;
    ///
    /// let width: u32 = 8;
    /// let height: u32 = 12;
    ///
    /// let image = image::open("test.jpg").unwrap()
    ///                 .resize(width * 100, height * 70, FilterType::Nearest)
    ///                 .into_luma8();
    ///
    /// let img_win = image.to_window(width, height).unwrap();
    ///
    /// for rows in img_win.rows().iter() {
    ///     for window in rows.iter() {
    ///         let pixel = window.get_pixel(4,6)[0];
    ///             ...
    ///     }
    /// }
    /// ```
    pub fn rows(&self) -> Vec<Vec<&Window<P>>> {
        let mut rows = vec![];
        let mut current = vec![];

        for win in self.windows.iter() {
            if current.len() >= self.windows_per_row as usize {
                rows.push(current);
                current = vec![];
            }
            //current.push(win.clone());
            current.push(win);
        }
        rows.push(current);

        rows
    }

    /*
    pub fn to_image(&self) -> ImageBuffer<P, Container> {
        let container = vec![];
        for window in self.windows.iter() {
            for i in 0..window.width {
                container.push(
            }
        }

       let buf = ImageBuffer::from_raw(3, 2, container).unwrap();

       buf
    }
    */


}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map() {
        let w = 2;
        let h = 4;
        let scale_w = 13;
        let scale_h = 7;
        
        let img = image::open("../images/pic1.jpg").unwrap()
                    .resize_exact(w*scale_w, h*scale_h, image::imageops::Nearest)
                    .into_luma8();

        let imgw = img.to_window(w, h).unwrap();

        let firsts = imgw.windows.iter().
                        map(|w| w.get_pixel(0, 0)[0]).collect::<Vec<u8>>();
        println!("{:?}", firsts);

        //assert_eq!(total as u32, (scale_w-1)*(scale_h-1) - 1);
    }

    #[test]
    fn test_enum() {
        let w = 2;
        let h = 4;
        let scale_w = 13;
        let scale_h = 7;
        
        let img = image::open("../images/pic1.jpg").unwrap()
                    .resize_exact(w*scale_w, h*scale_h, image::imageops::Nearest)
                    .into_luma8();

        let windows = img.to_window(w, h).unwrap().windows;

        let mut total = 0;
        for (n, _win) in windows.iter().enumerate() {
            total = n;
        }

        assert_eq!(total as u32, (scale_w-1)*(scale_h-1) - 1);
    }

    #[test]
    fn test_window_too_big() {
        let w = 5000;
        let h = 700;
        
        let img = image::open("../images/pic1.jpg").unwrap()
                    .into_rgb32f();
        let windows = img.to_window(w, h);

        assert_eq!(true, windows.is_none());
    }

    #[test]
    fn test_eq() {
        let w = 5;
        let h = 7;
        
        let img = image::open("../images/pic1.jpg").unwrap();
        let img2 = image::open("../images/pic1.jpg").unwrap();
        let img = img.resize_exact(w*3, h*4, image::imageops::Nearest)
                    .into_rgb8();
        let img2 = img2.resize_exact(w*3, h*4, image::imageops::Nearest)
                    .into_rgb8();



        let imgw = img.to_window(w, h).unwrap();
        let imgw2 = img2.to_window(w, h).unwrap();
       

        assert_eq!(imgw.windows, imgw2.windows);
    }

    #[test]
    fn test_creation() {
        let w = 5;
        let h = 7;
        
        let img = image::open("../images/pic1.jpg").unwrap()
                    .into_luma8();

        let imgw = img.to_window(w, h).unwrap();

        assert_eq!(imgw.windows.len() as u32, imgw.windows_per_row * imgw.windows_per_col);
    }
}
