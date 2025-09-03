// This file contains the color correction algorithims described in
// "Phantom SDK Cine File Format Manual Version 3.11.11.806"

use crate::errors::{CineError, CineResult};
use crate::file::CineFile;

#[derive(Clone, PartialEq)]
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
    pub fn get_cfa(value: &u32) -> CineResult<Self> {
        // Extract CFA type from least significat bytes (0x0000_00FF for u32)
        match value & 0x0000_00FF {
            0 => Ok(Self::Gray),
            1 => Ok(Self::Vri),
            2 => Ok(Self::VriV6),
            3 => Ok(Self::Bayer),
            4 => Ok(Self::BayerFlip),
            5 => Ok(Self::BayerFlipPb),
            6 => Ok(Self::BayerFlipPh),
            other => Err(CineError::Unsupported(crate::errors::FileTypeError {
                file_type: format!("Unknown CFA type {other:#x}"),
            })),
        }
    }

    // TODO: This is for multi-head cameras. Since I don't have any to test,
    // this will go unimplimented. Keep the pattern here incase I need it one day.
    pub fn get_color_head(value: &u32) -> CineResult<Self> {
        // Check high byte for color/gray heads
        match value & 0xF000_0000 {
            0x8000_0000 => Ok(Self::TopLeftGray),
            0x4000_0000 => Ok(Self::TopRightGray),
            0x2000_0000 => Ok(Self::BottomLeftGray),
            0x1000_0000 => Ok(Self::BottomRightGray),
            other => Err(CineError::Unsupported(crate::errors::FileTypeError {
                file_type: format!("Unknown CFA head {other:#x}"),
            })),
        }
    }

    pub fn apply_color_array(cine_file: &mut CineFile) -> CineResult<()> {
        // The pixels need a lifetime of "a" because they are a referece from the decompression alog.
        match cine_file.cfa {
            Self::Gray => Self::grayscale_10_to_16bit(cine_file),
            Self::Vri => Self::vri(cine_file),
            Self::VriV6 => Self::vri_v6(cine_file),
            Self::Bayer => Self::bayer(cine_file),
            Self::BayerFlip => Self::bayer_flip(cine_file),
            Self::BayerFlipPb => Self::bayer_flip_pb(cine_file),
            Self::BayerFlipPh => Self::bayer_flip_ph(cine_file),
            Self::TopLeftGray => Self::top_left_grey(cine_file),
            Self::TopRightGray => Self::top_right_grey(cine_file),
            Self::BottomLeftGray => Self::bottom_left_grey(cine_file),
            Self::BottomRightGray => Self::bottom_right_grey(cine_file),
        }
    }

    // pub fn get_buffer_size(&self, )

    fn grayscale_10_to_16bit(cine_file: &mut CineFile) -> CineResult<()> {
        for (i, &pixel) in cine_file.pixel_buffer.iter().enumerate() {
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
            cine_file.pixels[i] = pixel << 6;
        }
        Ok(())
    }

    // gbrg/rggb sensor
    fn vri(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }
    // bggr/grbg sensor
    fn vri_v6(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }
    // gb/rg sensor
    fn bayer(cine_file: &mut CineFile) -> CineResult<()> {
        let width = cine_file.bitmap_info_header.bi_width as u32;
        let height = cine_file.bitmap_info_header.bi_height as u32;
        // let mut rgb_data: Vec<u16> = vec![0u16; (width * height * 3) as usize];

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let idx: usize = (y * width + x) as usize;
                let r: u16;
                let g: u16;
                let b: u16;

                if y % 2 == 0 {
                    if x % 2 == 0 {
                        // G pixel on green-blue row
                        r = (cine_file.pixel_buffer[idx - 1] + cine_file.pixel_buffer[idx + 1]) / 2;
                        g = cine_file.pixel_buffer[idx];
                        b = (cine_file.pixel_buffer[idx - width as usize]
                            + cine_file.pixel_buffer[idx + width as usize])
                            / 2;
                    } else {
                        // B pixel
                        r = (cine_file.pixel_buffer[idx - width as usize - 1]
                            + cine_file.pixel_buffer[idx - width as usize + 1]
                            + cine_file.pixel_buffer[idx + width as usize - 1]
                            + cine_file.pixel_buffer[idx + width as usize + 1])
                            / 4;
                        g = (cine_file.pixel_buffer[idx - 1]
                            + cine_file.pixel_buffer[idx + 1]
                            + cine_file.pixel_buffer[idx - width as usize]
                            + cine_file.pixel_buffer[idx + width as usize])
                            / 4;
                        b = cine_file.pixel_buffer[idx];
                    }
                } else if x % 2 == 0 {
                    // R pixel
                    r = cine_file.pixel_buffer[idx];
                    g = (cine_file.pixel_buffer[idx - 1]
                        + cine_file.pixel_buffer[idx + 1]
                        + cine_file.pixel_buffer[idx - width as usize]
                        + cine_file.pixel_buffer[idx + width as usize])
                        / 4;
                    b = (cine_file.pixel_buffer[idx - width as usize - 1]
                        + cine_file.pixel_buffer[idx - width as usize + 1]
                        + cine_file.pixel_buffer[idx + width as usize - 1]
                        + cine_file.pixel_buffer[idx + width as usize + 1])
                        / 4;
                } else {
                    // G pixel on red-green row
                    r = (cine_file.pixel_buffer[idx - width as usize]
                        + cine_file.pixel_buffer[idx + width as usize])
                        / 2;
                    g = cine_file.pixel_buffer[idx];
                    b = (cine_file.pixel_buffer[idx - 1] + cine_file.pixel_buffer[idx + 1]) / 2;
                }

                let i: usize = idx * 3;
                cine_file.pixels[i] = r << 6;
                cine_file.pixels[i + 1] = g << 6;
                cine_file.pixels[i + 2] = b << 2;
            }
        }
        Ok(())
    }

    // rg/gb sensor
    fn bayer_flip(cine_file: &mut CineFile) -> CineResult<()> {
        todo!()
    }
    // gr/gb sensor
    fn bayer_flip_pb(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }
    // bg/gr sensor

    fn bayer_flip_ph(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }

    // TODO: This is for multi-head cameras. Since I don't have any to test,
    // this will go unimplimented.
    fn top_right_grey(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }

    fn top_left_grey(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }
    fn bottom_right_grey(cine_file: &mut CineFile) -> CineResult<()> {
        unimplemented!()
    }
    fn bottom_left_grey(cine_file: &mut CineFile) -> CineResult<()> {
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

// pub fn apply_gamma<'a>(cine_file: &CineFile, linear_pixels: &'a mut Vec<u16>) -> &'a Vec<u16> {
//     for pixel in linear_pixels.iter_mut() {
//         *pixel = (*pixel as f32).powf(1.0 / cine_file.setup.fGamma).round() as u16
//     }
//     linear_pixels
// }

#[cfg(test)]
mod tests {

    #[test]

    fn test_tests() {
        let a = 1;
        let b = 5;
        assert_eq!(a * b, 5);
    }
}
