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
    pub fn get_pixel(&self, x: u32, y:u32) -> &P {
        &self.data[x as usize * (self.width * y) as usize]
    }

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

pub trait Windowing<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    fn into_window(self, width: u32, height: u32) -> Option<ImageWindow<P, Container>>;
}


impl<P, Container> Windowing<P, Container>  for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    fn into_window(self, width: u32, height: u32) -> Option<ImageWindow<P, Container>> {
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


}


/*
impl<P, Container> Iterator for ImageWindow<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    type Item = Window<P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.windows.is_empty() {
            None
        }
        else if self.windows.len() > 1 {
            Some( self.windows.remove(1) )
        } else {
            Some( self.windows.pop().unwrap() )
        } 
    }
}
*/







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

        let imgw = img.into_window(w, h).unwrap();

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

        let windows = img.into_window(w, h).unwrap().windows;

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
        let windows = img.into_window(w, h);

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



        let imgw = img.into_window(w, h).unwrap();
        let imgw2 = img2.into_window(w, h).unwrap();
       

        assert_eq!(imgw.windows, imgw2.windows);
    }

    #[test]
    fn test_creation() {
        let w = 5;
        let h = 7;
        
        let img = image::open("../images/pic1.jpg").unwrap()
                    .into_luma8();

        let imgw = img.into_window(w, h).unwrap();

        assert_eq!(imgw.windows.len() as u32, imgw.windows_per_row * imgw.windows_per_col);
    }
}
