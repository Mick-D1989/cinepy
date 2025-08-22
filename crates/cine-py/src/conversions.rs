use crate::lut::LUT_10_TO_12;

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
    pub fn from_u32(value: u32) -> Vec<ColorFilterArray> {
        let mut color_filter_array = Vec::new();

        // Extract CFA type from least significat byte (0xFF or 0x0000_00FF to be more explicit for u32)
        match value & 0x0000_00FF {
            0x0000_0000 => color_filter_array.push(ColorFilterArray::Gray),
            0x0000_0001 => color_filter_array.push(ColorFilterArray::Vri),
            0x0000_0010 => color_filter_array.push(ColorFilterArray::VriV6),
            0x0000_0011 => color_filter_array.push(ColorFilterArray::Bayer),
            0x0000_0100 => color_filter_array.push(ColorFilterArray::BayerFlip),
            0x0000_0101 => color_filter_array.push(ColorFilterArray::BayerFlipPb),
            0x0000_0110 => color_filter_array.push(ColorFilterArray::BayerFlipPh),
            _ => panic!("CFA Pattern not recognised"),
        }

        // Check high byte for color/gray heads
        match value & 0xF000_0000 {
            0x8000_0000 => color_filter_array.push(ColorFilterArray::TopLeftGray),
            0x4000_0000 => color_filter_array.push(ColorFilterArray::TopRightGray),
            0x2000_0000 => color_filter_array.push(ColorFilterArray::BottomLeftGray),
            0x1000_0000 => color_filter_array.push(ColorFilterArray::BottomRightGray),
            _ => (),
        }
        color_filter_array
    }
}

pub fn grayscale_10_to_16bit(pixels_10bit: &mut Vec<u16>) -> &Vec<u16> {
    for pixel in pixels_10bit.iter_mut() {
        // Convert from 10-bits to 16-bits.
        // Since the most significant 6 bits will always be empty in the 10-bit file,
        // this remains a linear scaling transformation, ie. pix << n == pix * (2^n).
        *pixel <<= 6;
    }
    pixels_10bit
}

pub fn apply_lut_10_to_12(pixels: &mut [u16]) {
    for pixel in pixels.iter_mut() {
        // Clamp the value to ensure it's a valid index for the LUT
        let clamped = (*pixel).min(1023);
        // Overwrite the original value with the new value from the LUT
        *pixel = LUT_10_TO_12[clamped as usize];
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
