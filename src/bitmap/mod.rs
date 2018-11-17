//! [`ColorBuf`]s used with bitmaps.
//!
//! # About
//!
//! This module gives the ability to both create [`ColorBuf`]s from a bitmap and to write a
//! [`ColorBuf`] into a memory region as a bitmap, using a given conversion method.
//!
//! [`ColorBuf`]: ../trait.ColorBuf.html

use crate::{ColorBuf, Color, ColorBufError};
use std::result::Result;

/// Tells the [`BitmapColorBuf`] how the colors are arranged within the bitmap.
///
/// [`BitmapColorBuf`]: struct.BitmapColorBuf.html
pub enum ColorFormat {
    /// Representation where red is the low word, and alpha is the high word.
    RGBA,
    /// Representation where alpha is the low word, and blue is the high word.
    ARGB,
    /// Representation where red is the low word, and blue is the high word.
    RGB,
}

/// Tells the [`BitmapColorBuf`] how many bits are used in the bitmap per channel.
///
/// [`BitmapColorBuf`]: struct.BitmapColorBuf.html
pub enum BitDepth {
    Eight
}

pub struct BitmapColorBuf {
    data: Box<[u8]>,
    format: ColorFormat,
    depth: BitDepth,

    rows: u64,
    pixels_per_row: u64,
    stride: u64
}

impl ColorBuf for BitmapColorBuf {
    fn get_pixel(&self, x: u64, y: u64) -> Result<Color, ColorBufError> {
        if x >= self.pixels_per_row || y >= self.rows {
            return Err(ColorBufError::InvalidCoordinate);
        }
        let index = self.get_offset(x, y);
        let r: f32;
        let g: f32;
        let b: f32;
        let a: f32;

        match self.format {
            ColorFormat::RGBA => {
                match self.depth {
                    BitDepth::Eight => {
                        r = (self.data[index] as f32) / 255f32;
                        g = (self.data[index + 1] as f32) / 255f32;
                        b = (self.data[index + 2] as f32) / 255f32;
                        a = (self.data[index + 3] as f32) / 255f32;
                    }
                }
            },
            ColorFormat::ARGB => {
                match self.depth {
                    BitDepth::Eight => {
                        a = (self.data[index] as f32) / 255f32;
                        r = (self.data[index + 1] as f32) / 255f32;
                        g = (self.data[index + 2] as f32) / 255f32;
                        b = (self.data[index + 3] as f32) / 255f32;
                    }
                }
            },
            ColorFormat::RGB => {
                match self.depth {
                    BitDepth::Eight => {
                        r = (self.data[index] as f32) / 255f32;
                        g = (self.data[index + 1] as f32) / 255f32;
                        b = (self.data[index + 2] as f32) / 255f32;
                        a = 1.0f32;
                    }
                }
            }
        }

        Ok(Color { r: r, g: g, b: b, a: a })
    }

    fn set_pixel(&mut self, x: u64, y: u64, color: &Color) -> Result<(), ColorBufError> {
        if x >= self.pixels_per_row || y >= self.rows {
            return Err(ColorBufError::InvalidCoordinate);
        }
        let index = self.get_offset(x, y);

        // The alpha channel gets ignored in the case of RGB backing, and becomes a dividand
        // to the other color channels before application.
        // XXX: Is this reasonable?

        match self.format {
            ColorFormat::RGBA => {
                match self.depth {
                    BitDepth::Eight => {
                        let r_byte = (color.r * 255f32) as u8;
                        let g_byte = (color.g * 255f32) as u8;
                        let b_byte = (color.b * 255f32) as u8;
                        let a_byte = (color.a * 255f32) as u8;

                        self.data[index] = r_byte;
                        self.data[index + 1] = g_byte;
                        self.data[index + 2] = b_byte;
                        self.data[index + 3] = a_byte;
                    }
                }
            },
            ColorFormat::ARGB => {
                match self.depth {
                    BitDepth::Eight => {
                        let r_byte = (color.r * 255f32) as u8;
                        let g_byte = (color.g * 255f32) as u8;
                        let b_byte = (color.b * 255f32) as u8;
                        let a_byte = (color.a * 255f32) as u8;

                        self.data[index] = a_byte;
                        self.data[index + 1] = r_byte;
                        self.data[index + 2] = g_byte;
                        self.data[index + 3] = b_byte;
                    }
                }
            },
            ColorFormat::RGB => {
                match self.depth {
                    BitDepth::Eight => {
                        let r = color.r / color.a;
                        let g = color.g / color.a;
                        let b = color.b / color.a;

                        let r_byte = (r * 255f32) as u8;
                        let g_byte = (g * 255f32) as u8;
                        let b_byte = (b * 255f32) as u8;

                        self.data[index] = r_byte;
                        self.data[index + 1] = g_byte;
                        self.data[index + 2] = b_byte;
                    }
                }
            }
        }
        Ok(())
    }

    fn get_width(&self) -> u64 {
        self.pixels_per_row
    }

    fn get_height(&self) -> u64 {
        self.rows
    }
}

impl BitmapColorBuf {
    /// Returns a new color buffer for the given bitmap buffer.
    ///
    /// # Arguments
    ///
    /// * `format` - The format the data is in.
    /// * `depth` - The color depth of the data in the format.
    /// * `rows` - How many rows this bitmap image has?
    /// * `pixels_per_row` - The width of the image.
    /// * `stride` - How many bytes are between rows? For tightly packed bitmaps (i.e. no padding),
    /// this is the same as `pixels_per_row`.
    /// * `data` - The bitmap image.
    pub fn new(format: ColorFormat,
               depth: BitDepth,
               rows: u64,
               pixels_per_row: u64,
               stride: u64,
               data: Box<[u8]>) -> BitmapColorBuf {
        BitmapColorBuf {
            data,
            format,
            depth,
            rows,
            pixels_per_row,
            stride
        }
    }

    fn get_offset(&self, x: u64, y: u64) -> usize {
        (y * self.stride + (get_bpp_factor(&self.format, &self.depth) * x)) as usize
    }
}

fn get_bpp_factor(format: &ColorFormat, _depth: &BitDepth) -> u64 {
    let ret: u64;

    match &format {
        ColorFormat::RGBA => {
            ret = 4;
        },
        ColorFormat::ARGB => {
            ret = 4;
        },
        ColorFormat::RGB => {
            ret = 3;
        }
    }

    ret
}

#[derive(Debug, PartialEq)]
pub enum BitmapError {
    ByteArrayTooSmall
}

/// Writes the given [`ColorBuf`] to a bitmap
///
/// This function will write the contents of a given [`ColorBuf`] into
/// `output` using a certain `format` and `depth`, returning the `stride`
/// of the output.
///
/// User must make sure that the given byte slice is aligned to, e.g. 32-bit boundary, if required
/// by the consumer of the resulting bitmap.
///
/// Using the `output` if this function failed is a programmer error.
pub fn to_bitmap<'a, B>(buf: B,
                        format: ColorFormat,
                        depth: BitDepth,
                        stride: &mut u64,
                        output: &'a mut [u8]) -> std::result::Result<(), BitmapError>
    where
        B: ColorBuf
{
    // We often want this stuff to be aligned at 32-bit boundary.
    // FIXME: Do this better
    *stride = 4 * buf.get_width();

    let req_bitmap_len: usize = buf.get_height() as usize * (*stride as usize);
    if req_bitmap_len > output.len() {
        return Err(BitmapError::ByteArrayTooSmall);
    }

    for y in 0..buf.get_height() {
        for x in 0..buf.get_width() {
            let color: Color = buf.get_pixel(x, y).unwrap();
            let index: usize = ((y * (* stride) + (get_bpp_factor(&format, &depth) * x))) as usize;

            match depth {
                BitDepth::Eight => {
                    let r_byte = (color.r * 255f32) as u8;
                    let g_byte = (color.g * 255f32) as u8;
                    let b_byte = (color.b * 255f32) as u8;
                    let a_byte = (color.a * 255f32) as u8;

                    match format {
                        ColorFormat::RGBA => {
                            output[index] = r_byte;
                            output[index+1] = g_byte;
                            output[index+2] = b_byte;
                            output[index+3] = a_byte;
                        },
                        ColorFormat::ARGB => {
                            output[index] = a_byte;
                            output[index+1] = r_byte;
                            output[index+2] = g_byte;
                            output[index+3] = b_byte;
                        },
                        ColorFormat::RGB => {
                            output[index] = r_byte;
                            output[index+1] = g_byte;
                            output[index+2] = b_byte;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        // RGBA. First pixel is white, and second red
        let orig_bitmap = [0xFF, 0xFF, 0xFF, 0xFF,
                           0xFF, 0x00, 0x00, 0xFF ];

        let colorbuf = BitmapColorBuf::new(ColorFormat::RGBA, BitDepth::Eight,
                                           1, 2, 8, Box::new(orig_bitmap.clone()));
        let mut new_bitmap: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut stride: u64 = 0;
        to_bitmap(colorbuf, ColorFormat::RGBA, BitDepth::Eight, &mut stride, &mut new_bitmap)
            .unwrap();

        assert_eq!(8, stride);
        assert_eq!(orig_bitmap, new_bitmap);
    }

    #[test]
    fn modification() {
        // RGBA. First pixel is white, and second red
        let orig_bitmap = [0xFF, 0xFF, 0xFF, 0xFF,
                           0xFF, 0x00, 0x00, 0xFF ];

        // RGBA. We try to make the first pixel blue instead of white
        let expected_bitmap = [0x00, 0x00, 0xFF, 0xFF,
                               0xFF, 0x00, 0x00, 0xFF ];

        let mut colorbuf = BitmapColorBuf::new(ColorFormat::RGBA, BitDepth::Eight,
                                               1, 2, 8, Box::new(orig_bitmap));
        colorbuf.set_pixel(0, 0, &Color {r: 0f32, g: 0f32, b: 1f32, a: 1f32}).unwrap();

        let mut new_bitmap: [u8; 8] = [0; 8];
        let mut stride = 0;
        to_bitmap(colorbuf, ColorFormat::RGBA, BitDepth::Eight, &mut stride, &mut new_bitmap)
            .unwrap();

        assert_eq!(8, stride);
        assert_eq!(expected_bitmap, new_bitmap);
    }

    #[test]
    fn two_line_roundtrip() {
        // RGBA. 2x2 image with first pixel being red, second green, third blue, and fourth white
        let orig_bitmap = [0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
                           0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF ];
        let colorbuf = BitmapColorBuf::new(ColorFormat::RGBA, BitDepth::Eight,
                                           2, 2, 8, Box::new(orig_bitmap.clone()));
        let mut new_bitmap: [u8; 16] = [0x00u8; 16];
        let mut stride = 0;
        to_bitmap(colorbuf, ColorFormat::RGBA, BitDepth::Eight, &mut stride, &mut new_bitmap)
            .unwrap();

        assert_eq!(8, stride);
        assert_eq!(orig_bitmap, new_bitmap);
    }
}
