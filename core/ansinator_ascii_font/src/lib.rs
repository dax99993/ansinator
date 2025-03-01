//! Ascii Font Abstraction
//!
//! Provides an ascii 5x7 font abstraction, providing:
//! + Best fitting character 
//! + Comparing ascii characters

/// Short type alias for font data
type Font = [u8;5*7];

/// Abstraction for Ascii Font 
///
/// Container of Ascii Font, for storing the font data
/// and the character it represents
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsciiFont {
    pub ch: char,
    pub data: Font,
}


impl Default for AsciiFont {
    fn default() -> Self { Self { data:[0;35], ch : ' ' } }
}


impl AsciiFont {
    /// Create an AsciiFont from a given ascii character.
    /// 
    /// If a non ascii character is given as parameter, it returns a
    /// default AsciiFont (Space).
    pub fn from(ch: char) -> Self {
        let mut font = AsciiFont::default();

        //assert!(ch.is_ascii() == true, "{}", "Character set should contain only ascii");
        /* If not an ascii character return a default AsciiFont (space) */
        if !ch.is_ascii() { return font; }

        let ascii = ASCII_FONT[ch as usize - 32];
        for y in 0..7 {
            for x in 0..5 {
                let p =
                if ( ascii[x] & 1<<y ) != 0 {
                    255 
                }
                else {
                    0
                };                                                                                                                                                                             
            font.data[y*5 + x] = p;
            }
        }
        font.ch = ch;

        font
    }

    /// Calculates the quadrance of two AsciiFont to measure similarity
    ///
    /// The quadrance is a quadratic measure to compare how similar two objects are
    /// and is defined by the sum of the squares of the  pairwise differences of each object elements.
    ///
    /// When the two objects are equal the quadrance is equal to zero.
    ///
    /// The maximum value a quadrance can be is when comparing opposite objects (if such thing
    /// exists for the objects)
    /// and can be shown to be the number of elements of the object times the maximum value of the
    /// type of element squared
    /// in this case each element has an maximum equal to 255 and an AsciiFont Font data has 35 element thus the
    /// quadrance of opposite elements = 35 * 255*255.
    fn quadrance(&self, font: &AsciiFont) -> f64 {
        let mut s = 0.0;
        let f1 = self.data;
        let f2 = font.data;

        for (ai, bi) in f1.iter().zip(&f2) {
            s += f64::powi(*ai as f64 - *bi as f64, 2);
        }

        s
    }

    
    fn structural_similarity(&self, font: &AsciiFont) -> f64 {
        let dynamic_range = 255.0 ;
        let c1 = f64::powi(0.01 * dynamic_range, 2);
        let c2 = f64::powi(0.03 * dynamic_range, 2);

        // Calculate statistical metrics
        let ux: f64  = self.data.iter().map(|x| x.clone() as f64).sum::<f64>() / self.data.len() as f64;
        let uy: f64  = font.data.iter().map(|x| x.clone() as f64).sum::<f64>() / font.data.len() as f64;

        let covx: f64 = self.data.iter().map(|x| f64::powi(x.clone() as f64 - ux, 2)).sum::<f64>() / (self.data.len() as f64 - 1.0);
        let covy: f64 = font.data.iter().map(|x| f64::powi(x.clone() as f64 - uy, 2)).sum::<f64>() / (font.data.len() as f64 - 1.0);
        let covxy: f64 = self.data.iter().zip(font.data).map(|(x,y)| (x.clone() as f64 - ux) * (y.clone() as f64 - uy)).sum::<f64>() / (self.data.len() as f64 - 1.0);


        // Simplified case formula (when c3=0.5*c2, alpha=1, beta=1, gamma=1) as shown in:
        // https://en.wikipedia.org/wiki/Structural_similarity_index_measure
        return (2.0 * ux * uy + c1) * (2.0 * covxy + c2) / ((ux*ux + uy*uy + c1) * (covx + covy + c2)); 
    }
}

/// Find best AsciiFont approximation to given vector of AsciiFonts
///
/// Find the AsciiFont that minimizes the asimilarity of an AsciiFont
/// by a exahustive calculation of quadrances, return the AsciiFont which minimizes the quadrance.
pub fn minimize_quadrance(font1: &AsciiFont, font_set: &Vec<AsciiFont> ) -> char {
    let mut min = f64::MAX;
    let mut ch: char = ' ';

    for font in font_set {
        let q = font1.quadrance(&font);    

        if q < min {
            min = q;
            ch = font.ch;
        }
    }

    ch
}


/// Find best AsciiFont approximation to given vector of AsciiFonts
///
/// Find the AsciiFont that maximizes the structural similarity (ssim) of an AsciiFont
/// by a exahustive calculation of ssim, return the AsciiFont which maximizes the ssim.
pub fn maximize_structural_similarity(font1: &AsciiFont, font_set: &Vec<AsciiFont> ) -> char {
    let mut max = -1.0; //ssim in range [-1.0, 1.0]
    let mut ch: char = ' ';

    for font in font_set {
        let ssim = font1.structural_similarity(&font);    

        if ssim > max {
            max = ssim;
            ch = font.ch;
        }
    }

    ch
}


/// ASCII characters 5x7 font.
///
/// Monochron 5x7 Font based from
/// <https://github.com/adafruit/monochron/blob/master/firmware/font5x7.h>
/// With a minor changes on last 2 rows
///
/// Each 5x7 ascii character is encoded
/// as 5 bytes, each byte corresponds to a column ordered from left to right,
/// and each bit position corresponds to a row value,
/// the top row is bit0 up to bit7 at the bottom row.
///
/// | C0 | C1 | C2 | C3 | C4 |
/// |----|----|----|----|----|
/// | v0 | w0 | x0 | y0 | z0 |
/// | v1 | w1 | x1 | y1 | z1 |
/// | v2 | w2 | x2 | y2 | z2 |
/// | v3 | w3 | x3 | y3 | z3 |
/// | v4 | w4 | x4 | y4 | z4 |
/// | v5 | w5 | x5 | y5 | z5 |
/// | v6 | w6 | x6 | y6 | z6 |
/// | v7 | w7 | x7 | y7 | z7 |
///
const ASCII_FONT: [[u8; 5] ; 127-32] = [
    [0x00, 0x00, 0x00, 0x00, 0x00],// (space)
    [0x00, 0x00, 0x5F, 0x00, 0x00],// !
    [0x00, 0x07, 0x00, 0x07, 0x00],// "
    [0x14, 0x7F, 0x14, 0x7F, 0x14],// #
    [0x24, 0x2A, 0x7F, 0x2A, 0x12],// $
    [0x23, 0x13, 0x08, 0x64, 0x62],// %
    [0x36, 0x49, 0x55, 0x22, 0x50],// &
    [0x00, 0x05, 0x03, 0x00, 0x00],// '
    [0x00, 0x1C, 0x22, 0x41, 0x00],// (
    [0x00, 0x41, 0x22, 0x1C, 0x00],// )
    [0x08, 0x2A, 0x1C, 0x2A, 0x08],// *
    [0x08, 0x08, 0x3E, 0x08, 0x08],// +
    [0x00, 0x50, 0x30, 0x00, 0x00],// ,
    [0x08, 0x08, 0x08, 0x08, 0x08],// -
    [0x00, 0x60, 0x60, 0x00, 0x00],// .
    [0x20, 0x10, 0x08, 0x04, 0x02],// /
    [0x3E, 0x51, 0x49, 0x45, 0x3E],// 0
    [0x00, 0x42, 0x7F, 0x40, 0x00],// 1
    [0x42, 0x61, 0x51, 0x49, 0x46],// 2
    [0x21, 0x41, 0x45, 0x4B, 0x31],// 3
    [0x18, 0x14, 0x12, 0x7F, 0x10],// 4
    [0x27, 0x45, 0x45, 0x45, 0x39],// 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30],// 6
    [0x01, 0x71, 0x09, 0x05, 0x03],// 7
    [0x36, 0x49, 0x49, 0x49, 0x36],// 8
    [0x06, 0x49, 0x49, 0x29, 0x1E],// 9
    [0x00, 0x36, 0x36, 0x00, 0x00],// :
    [0x00, 0x56, 0x36, 0x00, 0x00],// ;
    [0x00, 0x08, 0x14, 0x22, 0x41],// <
    [0x14, 0x14, 0x14, 0x14, 0x14],// =
    [0x41, 0x22, 0x14, 0x08, 0x00],// >
    [0x02, 0x01, 0x51, 0x09, 0x06],// ?
    [0x32, 0x49, 0x79, 0x41, 0x3E],// @
    [0x7E, 0x11, 0x11, 0x11, 0x7E],// A
    [0x7F, 0x49, 0x49, 0x49, 0x36],// B
    [0x3E, 0x41, 0x41, 0x41, 0x22],// C
    [0x7F, 0x41, 0x41, 0x22, 0x1C],// D
    [0x7F, 0x49, 0x49, 0x49, 0x41],// E
    [0x7F, 0x09, 0x09, 0x01, 0x01],// F
    [0x3E, 0x41, 0x41, 0x51, 0x32],// G
    [0x7F, 0x08, 0x08, 0x08, 0x7F],// H
    [0x00, 0x41, 0x7F, 0x41, 0x00],// I
    [0x20, 0x40, 0x41, 0x3F, 0x01],// J
    [0x7F, 0x08, 0x14, 0x22, 0x41],// K
    [0x7F, 0x40, 0x40, 0x40, 0x40],// L
    [0x7F, 0x02, 0x04, 0x02, 0x7F],// M
    [0x7F, 0x04, 0x08, 0x10, 0x7F],// N
    [0x3E, 0x41, 0x41, 0x41, 0x3E],// O
    [0x7F, 0x09, 0x09, 0x09, 0x06],// P
    [0x3E, 0x41, 0x51, 0x21, 0x5E],// Q
    [0x7F, 0x09, 0x19, 0x29, 0x46],// R
    [0x46, 0x49, 0x49, 0x49, 0x31],// S
    [0x01, 0x01, 0x7F, 0x01, 0x01],// T
    [0x3F, 0x40, 0x40, 0x40, 0x3F],// U
    [0x1F, 0x20, 0x40, 0x20, 0x1F],// V
    [0x7F, 0x20, 0x18, 0x20, 0x7F],// W
    [0x63, 0x14, 0x08, 0x14, 0x63],// X
    [0x03, 0x04, 0x78, 0x04, 0x03],// Y
    [0x61, 0x51, 0x49, 0x45, 0x43],// Z
    [0x00, 0x00, 0x7F, 0x41, 0x41],// [
    [0x02, 0x04, 0x08, 0x10, 0x20],// "\"
    [0x41, 0x41, 0x7F, 0x00, 0x00],// ]
    [0x04, 0x02, 0x01, 0x02, 0x04],// ^
    [0x40, 0x40, 0x40, 0x40, 0x40],// _
    [0x00, 0x01, 0x02, 0x04, 0x00],// `
    [0x20, 0x54, 0x54, 0x54, 0x78],// a
    [0x7F, 0x48, 0x44, 0x44, 0x38],// b
    [0x38, 0x44, 0x44, 0x44, 0x20],// c
    [0x38, 0x44, 0x44, 0x48, 0x7F],// d
    [0x38, 0x54, 0x54, 0x54, 0x18],// e
    [0x08, 0x7E, 0x09, 0x01, 0x02],// f
    [0x08, 0x14, 0x54, 0x54, 0x3C],// g
    [0x7F, 0x08, 0x04, 0x04, 0x78],// h
    [0x00, 0x44, 0x7D, 0x40, 0x00],// i
    [0x20, 0x40, 0x44, 0x3D, 0x00],// j
    [0x00, 0x7F, 0x10, 0x28, 0x44],// k
    [0x00, 0x41, 0x7F, 0x40, 0x00],// l
    [0x7C, 0x04, 0x18, 0x04, 0x78],// m
    [0x7C, 0x08, 0x04, 0x04, 0x78],// n
    [0x38, 0x44, 0x44, 0x44, 0x38],// o
    [0x7C, 0x14, 0x14, 0x14, 0x08],// p
    [0x08, 0x14, 0x14, 0x18, 0x7C],// q
    [0x7C, 0x08, 0x04, 0x04, 0x08],// r
    [0x48, 0x54, 0x54, 0x54, 0x20],// s
    [0x04, 0x3F, 0x44, 0x40, 0x20],// t
    [0x3C, 0x40, 0x40, 0x20, 0x7C],// u
    [0x1C, 0x20, 0x40, 0x20, 0x1C],// v
    [0x3C, 0x40, 0x30, 0x40, 0x3C],// w
    [0x44, 0x28, 0x10, 0x28, 0x44],// x
    [0x0C, 0x50, 0x50, 0x50, 0x3C],// y
    [0x44, 0x64, 0x54, 0x4C, 0x44],// z
    [0x00, 0x08, 0x36, 0x41, 0x00],// {
    [0x00, 0x00, 0x7F, 0x00, 0x00],// |
    [0x00, 0x41, 0x36, 0x08, 0x00],// }
    [0x08, 0x04, 0x08, 0x10, 0x08],// ~
];

#[cfg(test)]
mod tests {
    use crate::{maximize_structural_similarity, minimize_quadrance};

    use super::AsciiFont;

    #[test]
    fn quadrance_equal() {
        let f1 = AsciiFont::from('a');
        let f2 = AsciiFont::from('a');

        assert_eq!(f1.quadrance(&f2), 0.0);
    }

    #[test]
    fn quadrance_non_equal() {
        let f1 = AsciiFont::from('a');
        let f2 = AsciiFont::from('A');

        assert_ne!(f1.quadrance(&f2), 0.0);
    }

    #[test]
    fn quadrance_size_comparison() {
        let f1 = AsciiFont::from('.');
        let f2 = AsciiFont::from('#');
        let f3 = AsciiFont::from(',');

        let q1 = f1.quadrance(&f2);
        /* More similar fonts, hence smaller value */
        let q2 = f1.quadrance(&f3);

        assert!(q1-q2 > 0.0);
    }

    #[test]
    fn font_quadrance_minimization() {
        let f1 = AsciiFont::from('.');
        let f2 = AsciiFont::from('#');
        let f3 = AsciiFont::from(',');
        let f4 = AsciiFont::from('?');

        let fontset : Vec::<AsciiFont> = vec![f2, f3, f4];

        let closest_ch = minimize_quadrance(&f1, &fontset);

        assert_eq!(closest_ch, ',');
    }

    #[test]
    fn ssim_equal() {
        let f1 = AsciiFont::from('a');
        let f2 = AsciiFont::from('a');

        let abs_difference = (f1.structural_similarity(&f2) - 1.0).abs();
        println!("Structural Similarity = {:?}", abs_difference);
        assert!(abs_difference <= f64::EPSILON)
    }

    #[test]
    fn ssim_non_equal() {
        let f1 = AsciiFont::from('a');
        let f2 = AsciiFont::from('A');

        let ssim = (f1.structural_similarity(&f2) - 1.0).abs();
        assert!(f64::abs(ssim) >= 0.0);
    }

    #[test]
    fn ssim_size_comparison() {
        let f1 = AsciiFont::from('B');
        let f2 = AsciiFont::from('8');
        let f3 = AsciiFont::from('.');

        let ssim1 = f64::abs(f1.structural_similarity(&f2));
        /* More similar fonts, hence smaller value */
        let ssim2 = f64::abs(f1.structural_similarity(&f3));

        assert!(ssim1-ssim2 > 0.0);
    }

    #[test]
    fn font_ssim_maximization() {
        let f1 = AsciiFont::from('B');
        let f2 = AsciiFont::from('.');
        let f3 = AsciiFont::from('8');
        let f4 = AsciiFont::from('|');

        let fontset : Vec::<AsciiFont> = vec![f2, f3, f4];

        let closest_ch = maximize_structural_similarity(&f1, &fontset);

        assert_eq!(closest_ch, '8');
    }
}
