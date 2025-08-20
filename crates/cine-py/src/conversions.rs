use crate::lut::LUT_10_TO_12;

pub enum ColorFilterArray {
    None,        // 0 - gray sensor
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
            0x0000_0000 => color_filter_array.push(ColorFilterArray::None),
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

/// Unpack 10-bit packed Bayer/greyscale into Vec<u16>
/// bi_compression=256 means that there is 4 pixles of 10-bit data stored in 5 bytes(40-bits).
pub fn decompress_10bit_packed(data: &[u8]) -> Vec<u16> {
    let mut out: Vec<u16> = Vec::with_capacity(data.len() * 4 / 5);

    let mut i: usize = 0;
    while i + 4 < data.len() {
        // Get the first 5 bytes.
        let b0: u8 = data[i];
        let b1: u8 = data[i + 1];
        let b2: u8 = data[i + 2];
        let b3: u8 = data[i + 3];
        let b4: u8 = data[i + 4];

        // set the values for each 4 pixels. assume they're ordered as;
        // 00000000 00|000000 0000|0000 000000|00 00000000
        // ----p0-- --|----p1 ----|---- p2----|-- p3------
        // turns into;
        // xxxxxx00 00000000 xxxxxx00 00000000 xxxxxx00 00000000 xxxxxx00 00000000
        // --------p0------- --------p1------- --------p2------- --------p3-------
        // and;
        // p0 starts in the top left corner of the frame.

        let p0: u16 = ((b0 as u16) << 2) | ((b1 as u16) >> 6);
        let p1: u16 = (((b1 & 0b0011_1111) as u16) << 4) | ((b2 as u16) >> 4);
        let p2: u16 = (((b2 & 0b0000_1111) as u16) << 6) | ((b3 as u16) >> 2);
        let p3: u16 = (((b3 & 0b0000_0011) as u16) << 8) | (b4 as u16);

        out.push(p0);
        out.push(p1);
        out.push(p2);
        out.push(p3);

        i += 5;
    }

    // Output is a unformatted, but unpacked vector (n,1) shape,
    // needs to be converted to (width, height) shape later on.
    out
}

pub fn grayscale_10_to_16bit(pixels_10bit: &mut Vec<u16>) -> &Vec<u16> {
    for pixel in pixels_10bit.iter_mut() {
        // Convert from 10-bits to fill the u16.
        *pixel <<= 6;
    }
    pixels_10bit
}

// pub fn apply_lut_10_to_12(pixels_10bit: &mut [u16]) -> Vec<u16> {
//     pixels_10bit
//         .iter()
//         .map(|&val| {
//             let clamped = val.min(1023); // ensure valid index
//             LUT_10_TO_12[clamped as usize]
//         })
//         .collect()
// }

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
