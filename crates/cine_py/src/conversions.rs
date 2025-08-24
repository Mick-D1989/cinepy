// This file contains the color correction algorithims described in
// "Phantom SDK Cine File Format Manual Version 3.11.11.806"

use std::fmt::Error;

use crate::{file::CineFile, lut::LUT_10_TO_12};
use pyo3::PyErr;
use pyo3::conversion::IntoPyObject;
use pyo3::prelude::*;
use pyo3::types::PyList;

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

// pub type CFAType<'a> = Cow<'a, [u16]>;

pub enum CFAType<'a> {
    Gray(&'a [u16]),
    Color(Vec<u16>),
}

impl<'a> CFAType<'a> {
    pub fn unwrap(self) -> Vec<u16> {
        match self {
            CFAType::Gray(x) => x.to_owned(),
            CFAType::Color(x) => x,
        }
    }
}

impl<'a, 'py> IntoPyObject<'py> for CFAType<'a> {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        match self {
            CFAType::Gray(slice) => {
                // Expose grayscale as a Python list of ints
                Ok(PyList::new(py, slice).unwrap().into_any())
            }
            CFAType::Color(vec) => {
                // Each pixel is [u16; 3], Python will see it as a list of tuples/lists
                Ok(PyList::new(py, vec).unwrap().into_any())
            }
        }
    }
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
            0x8000_0000 => Ok(Self::TopLeftGray),
            0x4000_0000 => Ok(Self::TopRightGray),
            0x2000_0000 => Ok(Self::BottomLeftGray),
            0x1000_0000 => Ok(Self::BottomRightGray),
            _ => Err(Error),
        }
    }

    pub fn apply_color_array<'a>(&self, pixels: &'a mut [u16]) -> Result<CFAType<'a>, Error> {
        // The pixels need a lifetime of "a" because they are a referece from the decompression alog.
        match self {
            Self::Gray => Ok(Self::grayscale_10_to_16bit(pixels)?),
            Self::Vri => Ok(Self::vri(pixels)?),
            Self::VriV6 => Ok(Self::vri_v6(pixels)?),
            Self::Bayer => Ok(Self::bayer(pixels)?),
            Self::BayerFlip => Ok(Self::bayer_flip(pixels)?),
            Self::BayerFlipPb => Ok(Self::bayer_flip_pb(pixels)?),
            Self::BayerFlipPh => Ok(Self::bayer_flip_ph(pixels)?),
            Self::TopLeftGray => Ok(Self::top_left_grey(pixels)?),
            Self::TopRightGray => Ok(Self::top_right_grey(pixels)?),
            Self::BottomLeftGray => Ok(Self::bottom_left_grey(pixels)?),
            Self::BottomRightGray => Ok(Self::bottom_right_grey(pixels)?),
            _ => Err(Error),
        }
    }

    fn grayscale_10_to_16bit<'a>(pixels_10bit: &'a mut [u16]) -> Result<CFAType<'a>, Error> {
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
        Ok(CFAType::Gray(pixels_10bit))
    }

    // gbrg/rggb sensor
    fn vri<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }
    // bggr/grbg sensor
    fn vri_v6<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }
    // gb/rg sensor
    fn bayer<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        let width = 2048;
        let height = 1080;
        let mut rgb_data: Vec<u16> = vec![0u16; (width * height * 3) as usize];

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx: usize = (y * width + x) as usize;
                let r: u16;
                let g: u16;
                let b: u16;

                if y % 2 == 0 {
                    if x % 2 == 0 {
                        // G pixel on green-blue row
                        r = (pixels_10bit[idx - 1] + pixels_10bit[idx + 1]) / 2;
                        g = pixels_10bit[idx];
                        b = (pixels_10bit[idx - width as usize]
                            + pixels_10bit[idx + width as usize])
                            / 2;
                    } else {
                        // B pixel
                        r = (pixels_10bit[idx - width as usize - 1]
                            + pixels_10bit[idx - width as usize + 1]
                            + pixels_10bit[idx + width as usize - 1]
                            + pixels_10bit[idx + width as usize + 1])
                            / 4;
                        g = (pixels_10bit[idx - 1]
                            + pixels_10bit[idx + 1]
                            + pixels_10bit[idx - width as usize]
                            + pixels_10bit[idx + width as usize])
                            / 4;
                        b = pixels_10bit[idx];
                    }
                } else if x % 2 == 0 {
                    // R pixel
                    r = pixels_10bit[idx];
                    g = (pixels_10bit[idx - 1]
                        + pixels_10bit[idx + 1]
                        + pixels_10bit[idx - width as usize]
                        + pixels_10bit[idx + width as usize])
                        / 4;
                    b = (pixels_10bit[idx - width as usize - 1]
                        + pixels_10bit[idx - width as usize + 1]
                        + pixels_10bit[idx + width as usize - 1]
                        + pixels_10bit[idx + width as usize + 1])
                        / 4;
                } else {
                    // G pixel on red-green row
                    r = (pixels_10bit[idx - width as usize] + pixels_10bit[idx + width as usize])
                        / 2;
                    g = pixels_10bit[idx];
                    b = (pixels_10bit[idx - 1] + pixels_10bit[idx + 1]) / 2;
                }

                let i: usize = idx * 3;
                rgb_data[i] = r << 6;
                rgb_data[i + 1] = g << 6;
                rgb_data[i + 2] = b << 2;
            }
        }
        Ok(CFAType::Color(rgb_data))
    }

    // rg/gb sensor
    fn bayer_flip<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }
    // gr/gb sensor
    fn bayer_flip_pb<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }
    // bg/gr sensor

    fn bayer_flip_ph<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }

    // TODO: This is for multi-head cameras. Since I don't have any to test,
    // this will go unimplimented.
    fn top_right_grey<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }

    fn top_left_grey<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }
    fn bottom_right_grey<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
    }
    fn bottom_left_grey<'a>(pixels_10bit: &'a [u16]) -> Result<CFAType<'a>, Error> {
        unimplemented!()
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

#[cfg(test)]
mod tests {

    #[test]

    fn test_tests() {
        let a = 1;
        let b = 5;
        assert_eq!(a * b, 5);
    }
}
