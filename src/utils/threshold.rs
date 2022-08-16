//! GrayImage Threshold Trait. 
//!
//! Provide the Threshold Trait for GrayImage, implementing:
//! + Otsu's binarization threshold 
//! + Manual binarization threshold 
//! + Image pixel invertion

use image::GrayImage;

/// Related threshold functions for binarization
pub trait Threshold {
    /// Binarize the image with the given threshold value
    fn threshold(&mut self, threshold: u8);
    /// Binarize the image with the threshold value calculated with Otsu's method
    ///
    /// Otsu's method allows for automatic image binarization based on statistical
    /// analysis with the image's histogram.
    ///
    /// The code was copy shamelessly from tai
    /// <https://github.com/MustafaSalih1993/tai/blob/master/src/operations/otsu_threshold.rs>
    ///
    /// Thanks for the code and information about
    /// automatic threshold!!.
    fn otsu_threshold(&mut self);
    /// Create histogram of the pixel values
    fn get_histogram(&mut self) -> [usize; 256];
    /// Analyze image histogram and calculated best threshold value
    fn get_otsu_value(&mut self) -> u8;
    /// Invert image pixels values
    fn invert(&mut self);
}

impl Threshold for GrayImage {
    fn threshold(&mut self, threshold: u8) {
        self.iter_mut()
            .for_each(|p| *p = if *p > threshold { 255 } else { 0 });
    }

    fn get_histogram(&mut self) -> [usize; 256] {
        let mut out = [0; 256];
        self.iter().for_each(|p| {
            out[*p as usize] += 1;
        });
        out
    }

    fn get_otsu_value(&mut self) -> u8 {
        let img_histogram: [usize; 256] = self.get_histogram();
        let total_weight = self.width() as f64 * self.height() as f64;
        let mut bg_sum = 0.0;
        let mut bg_weight = 0.0;
        let mut max_variance = 0.0;
        let mut best_threshold = 0;
        let sum_intensity: f64 = img_histogram
            .iter()
            .enumerate()
            .fold(0f64, |acu, (t, c)| acu + (t * c) as f64);

        for (threshold, count) in img_histogram.iter().enumerate() {
            let fg_weight = total_weight - bg_weight;
            if fg_weight > 0.0 && bg_weight > 0.0 {
                let fg_mean = (sum_intensity - bg_sum) / fg_weight;
                //let val = (bg_weight * fg_weight * ((bg_sum / bg_weight) - fg_mean)).powi(2);
                let val = (bg_weight * fg_weight * ((bg_sum / bg_weight) - fg_mean)).powi(2);
                if val >= max_variance {
                    best_threshold = threshold as u8;
                    max_variance = val;
                }
            }
            bg_weight += *count as f64;
            bg_sum += (threshold * count) as f64;
        }

        best_threshold
    }

    fn otsu_threshold(&mut self) {
        let threshold = self.get_otsu_value();
        //println!("{}", threshold);
        self.iter_mut()
            .for_each(|p| *p = if *p > threshold { 255 } else { 0 });
    }

    fn invert(&mut self) {
        self.iter_mut().for_each(|p| *p = 255 - *p);
    }
}
