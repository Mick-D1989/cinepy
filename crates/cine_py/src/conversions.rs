// This file contains the color correction algorithims described in
// "Phantom SDK Cine File Format Manual Version 3.11.11.806"

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

    // TODO: This is for multi-head cameras. Since I don't have any to test,
    // this will go unimplimented. Keep the pattern here incase I need it one day.
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

    pub fn apply_color_array<'a>(&self, pixels: &'a mut Vec<u16>) -> Result<&'a [u16], Error> {
        // The pixels need a lifetime of "a" because they are a referece from the decompression alog.
        match self {
            Self::Gray => Ok(Self::grayscale_10_to_16bit(pixels)),
            Self::Vri => Ok(Self::vri(pixels)),
            Self::VriV6 => Ok(Self::vri_v6(pixels)),
            Self::Bayer => Ok(Self::bayer(pixels)),
            Self::BayerFlip => Ok(Self::bayer_flip(pixels)),
            Self::BayerFlipPb => Ok(Self::bayer_flip_pb(pixels)),
            Self::BayerFlipPh => Ok(Self::bayer_flip_ph(pixels)),
            Self::TopLeftGray => Ok(Self::top_left_grey(pixels)),
            Self::TopRightGray => Ok(Self::top_right_grey(pixels)),
            Self::BottomLeftGray => Ok(Self::bottom_left_grey(pixels)),
            Self::BottomRightGray => Ok(Self::bottom_right_grey(pixels)),
            _ => Err(Error),
        }
    }

    fn grayscale_10_to_16bit(pixels_10bit: &mut [u16]) -> &[u16] {
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

    // gbrg/rggb sensor
    fn vri(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    // bggr/grbg sensor
    fn vri_v6(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    // gb/rg sensor
    fn bayer(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    // rg/gb sensor
    fn bayer_flip(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    // gr/gb sensor
    fn bayer_flip_pb(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    // bg/gr sensor

    fn bayer_flip_ph(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }

    // TODO: This is for multi-head cameras. Since I don't have any to test,
    // this will go unimplimented.
    fn top_right_grey(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }

    fn top_left_grey(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    fn bottom_right_grey(pixels_10bit: &mut [u16]) -> &[u16] {
        pixels_10bit
    }
    fn bottom_left_grey(pixels_10bit: &mut [u16]) -> &[u16] {
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
