use std::fmt::Error;

use crate::{file::CineFile, lut::LUT_10_TO_12};

pub enum ColorFilterArray {
    Gray,        // 0 - gray sensor
    Vri,         // 1 - gbrg / rggb
    VriV6,       // 2 - bggr / grbg
    Bayer,       // 3 - gb/rg
    BayerFlip,   // 4 - rg/gb
    BayerFlipPb, // 5 - gr/gb
    BayerFlipPh, // 6 - bg/gr

    // High byte carries information about color/gray heads on v6 and v6.2.
    TopLeftGray,     // 0x80000000
    TopRightGray,    // 0x40000000
    BottomLeftGray,  // 0x20000000
    BottomRightGray, // 0x10000000
}

impl ColorFilterArray {
    pub fn get_cfa(value: &u32) -> Result<Self, Error> {
        // Extract CFA type from least significat bytes (0x0000_00FF for u32)
        match value & 0x0000_00FF {
            0 => Ok(Self::Gray),
            1 => Ok(Self::Vri),
            2 => Ok(Self::VriV6),
            3 => Ok(Self::Bayer),
            4 => Ok(Self::BayerFlip),
            5 => Ok(Self::BayerFlipPb),
            6 => Ok(Self::BayerFlipPh),
            _ => Err(Error),
        }
    }

    pub fn get_color_head(value: &u32) -> Result<Self, Error> {
        // Check high byte for color/gray heads
        match value & 0xF000_0000 {
            0b1000_0000 => Ok(Self::TopLeftGray),
            0b0100_0000 => Ok(Self::TopRightGray),
            0b0010_0000 => Ok(Self::BottomLeftGray),
            0b0001_0000 => Ok(Self::BottomRightGray),
            _ => Err(Error),
        }
    }

    pub fn apply_color_array<'a>(&self, pixels: &'a mut Vec<u16>) -> Result<&'a Vec<u16>, Error> {
        // The pixels need a lifetime of "a" because they are a referece from the decompression alog.
        match self {
            Self::Gray => Ok(Self::grayscale_10_to_16bit(pixels)),
            Self::Vri => Err(Error),
            Self::VriV6 => Err(Error),
            Self::Bayer => Err(Error),
            Self::BayerFlip => Err(Error),
            Self::BayerFlipPb => Err(Error),
            Self::BayerFlipPh => Err(Error),
            Self::TopLeftGray => Err(Error),
            Self::TopRightGray => Err(Error),
            Self::BottomLeftGray => Err(Error),
            Self::BottomRightGray => Err(Error),
            _ => Err(Error),
        }
    }

    fn grayscale_10_to_16bit(pixels_10bit: &mut Vec<u16>) -> &Vec<u16> {
        for pixel in pixels_10bit.iter_mut() {
            // 10-bit packed set black level at 64 and white level at 1014
            // if *pixel > 1014 {
            //     *pixel = 1023
            // } else if *pixel < 64 {
            //     *pixel = 0
            // }
            // // Clamp the value to ensure it's a valid index for the LUT
            // let clamped = (*pixel).min(1023);

            // // Overwrite the original value with the new value from the LUT.
            // // converts from 10-bits packed to 12-bits linear
            // *pixel = LUT_10_TO_12[clamped as usize];
            // Convert from 12-bits to 16-bits.
            // Since the most significant 4 bits will always be empty in the 12-bit linear conversion,
            // this remains a linear scaling transformation, ie. pix << n == pix * (2^n).
            *pixel <<= 6;
        }
        pixels_10bit
    }
}

pub fn flip_vertical_16bit(data: &mut [u16], width: u32, height: u32) {
    let row_len: usize = width as usize;
    for y in 0..(height as usize / 2) {
        let top_row: usize = y * row_len;
        let bottom_row: usize = (height as usize - 1 - y) * row_len;
        for x in 0..row_len {
            data.swap(top_row + x, bottom_row + x);
        }
    }
}

pub fn apply_gamma<'a>(cine_file: &CineFile, linear_pixels: &'a mut Vec<u16>) -> &'a Vec<u16> {
    for pixel in linear_pixels.iter_mut() {
        *pixel = (*pixel as f32).powf(1.0 / cine_file.setup.fGamma).round() as u16
    }
    linear_pixels
}
