use std::fmt::Error;

use crate::cine::BitmapInfoHeader;

pub trait Decompress {
    fn decompress(header: &BitmapInfoHeader, data: &[u8]) -> Result<Vec<u16>, Error>;
}

pub enum Decompression {
    Packed10Bit,
    Packed12Bit,
}

impl Decompress for Decompression {
    fn decompress(header: &BitmapInfoHeader, data: &[u8]) -> Result<Vec<u16>, Error> {
        match header.bi_compression {
            256 => Ok(Decompression::decompress_10bit_packed(data)),
            1024 => Ok(Decompression::decompress_12bit_packed(data)),
            _ => Err(Error),
        }
    }
}

impl Decompression {
    /// Unpack 10-bit packed Bayer/greyscale into Vec<u16>
    /// bi_compression=256 means that there is 4 pixles of 10-bit data stored in 5 bytes(40-bits).
    fn decompress_10bit_packed(data: &[u8]) -> Vec<u16> {
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

    /// Unpack 12-bit packed Bayer/greyscale into Vec<u16>
    /// bi_compression=1024 means that there is 2 pixles of 12-bit data stored in 3 bytes(24-bits).
    fn decompress_12bit_packed(data: &[u8]) -> Vec<u16> {
        let mut out = Vec::with_capacity(data.len() * 2 / 3);
        let mut i: usize = 0;
        while i + 3 < data.len() {
            let b0: u8 = data[i];
            let b1: u8 = data[i + 1];
            let b2: u8 = data[i + 2];

            // set the values for each 2 pixels. assume they're ordered as;
            // 00000000 0000|0000 00000000
            // ------p0 ----|---- p1-----|
            // turns into;
            // xxxx0000 00000000 xxxx0000 00000000
            // --------p0------- --------p1-------
            // and;
            // p0 starts in the top left corner of the frame.

            let p0: u16 = ((b0 as u16) << 4) | ((b1 as u16) >> 4);
            let p1: u16 = (((b1 & 0b0000_1111) as u16) << 8) | (b2 as u16);

            out.push(p0);
            out.push(p1);

            i += 3
        }
        out
    }
}
