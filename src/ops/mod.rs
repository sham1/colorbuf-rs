//! This module defined a bunch of operations for [`ColorBuf`]s.
//!
//! # About
//!
//! This module defines a bunch of useful manipulations for [`ColorBuf`]s.
//!
//! [`ColorBuf`]: ../struct.ColorBuf.html

use super::*;

type Result<T> = std::result::Result<T, ColorBufError>;

/// Subregion.
///
/// This [`ColorBuf`] is used to manipulate regions that are parts of other [`ColorBuf`]s.
///
/// [`ColorBuf`]: ../stuct.ColorBuf.html
pub struct SubRegionColorBuf<'a, B>
    where B: 'a + ColorBuf
{
    backing: &'a mut B,
    reg_x: u64,
    reg_y: u64,

    width: u64,
    height: u64
}

impl <'a, B> SubRegionColorBuf<'a, B>
    where B: 'a + ColorBuf
{
    pub fn new(backing: &'a mut B,
               start_x: u64,
               start_y: u64,
               width: u64,
               height: u64) -> Result<SubRegionColorBuf<'a, B>> {
        if (start_x + width) >= backing.get_width() || (start_y + height) >= backing.get_height() {
            return Err(ColorBufError::InvalidDimensions);
        }
        Ok(SubRegionColorBuf { backing, reg_x: start_x, reg_y: start_y, width, height })
    }
}

impl <'a, B> ColorBuf for SubRegionColorBuf<'a, B>
    where B: 'a + ColorBuf
{
    fn get_pixel(&self, x: u64, y: u64) -> Result<Color> {
        if x >= self.width || y >= self.height {
            return Err(ColorBufError::InvalidCoordinate);
        }
        self.backing.get_pixel(self.reg_x + x, self.reg_y + y)
    }

    fn set_pixel(&mut self, x: u64, y: u64, color: &Color) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err(ColorBufError::InvalidCoordinate);
        }
        self.backing.set_pixel(self.reg_x + x, self.reg_y + y, color)
    }

    fn get_width(&self) -> u64 {
        self.width
    }

    fn get_height(&self) -> u64 {
        self.height
    }
}
