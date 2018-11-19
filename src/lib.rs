//! Manipulating 2D buffers of color.
//!
//! # About
//!
//! This crate introduces [`ColorBuf`], which can be used to manipulate a 2D array of pixels. For
//! the user, the pixels in this array are represented as [`Color`]-entries where the colors are
//! represented as floating point numbers between 0 and 1. The co-ordinate system goes from `(0, 0)`
//! to `(width - 1, height - 1)`, where `(0, 0)` corresponds to the top-left corner of the buffer.
//!
//! [`ColorBuf`]: trait.ColorBuf.html
//! [`Color`]: struct.Color.html

use std::result::Result;

#[derive(Debug, PartialEq)]
pub enum ColorBufError {
    InvalidCoordinate,
    InvalidDimensions,
}

/// 2D manipulatable region of pixels.
pub trait ColorBuf {
    /// Gets the color at a given pixel position.
    ///
    /// The `x` and `y` must not go outside of their bounds
    /// `(x < width and y < height)`.
    fn get_pixel(&self, x: u64, y: u64) -> Result<Color, ColorBufError>;

    /// Sets the color at a given pixel position.
    ///
    /// The `x` and `y` must not go outside of their bounds
    /// `(0 <= x < width and 0 <= y < height)`.
    fn set_pixel(&mut self, x: u64, y: u64, color: &Color) -> Result<(), ColorBufError>;

    /// Gets the width of the `ColorBuf`.
    fn get_width(&self) -> u64;

    /// Gets the width of the `ColorBuf`.
    fn get_height(&self) -> u64;
}

/// Color of a pixel.
///
/// This struct represents a single straigh alpha RGBA color value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// The red color channel. Values range from 0 to 1.
    pub r: f32,
    /// The green color channel. Values range from 0 to 1.
    pub g: f32,
    /// The blue color channel. Values range from 0 to 1.
    pub b: f32,
    /// The alpha channel. Values range from 0 to 1.
    pub a: f32,
}

impl Color {
    /// Blends `src` to this color with gamma correcion.
    ///
    /// NOTE: `gamma` is usually `2.2f32`.
    pub fn blend_with_gamma(self, src: Color, gamma: f32) -> Color {
        let out_a = src.a + self.a * (1f32 - src.a);
        let out_r = ((src.r).powf(gamma) * src.a + (self.r).powf(gamma) * (1f32 - src.a))
            .powf(1f32 / gamma);
        let out_g = ((src.g).powf(gamma) * src.a + (self.g).powf(gamma) * (1f32 - src.a))
            .powf(1f32 / gamma);
        let out_b = ((src.b).powf(gamma) * src.a + (self.b).powf(gamma) * (1f32 - src.a))
            .powf(1f32 / gamma);

        Color {
            r: out_r,
            g: out_g,
            b: out_b,
            a: out_a,
        }
    }
}

pub mod bitmap;
pub mod ops;
