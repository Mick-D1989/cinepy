use std::fmt::Error;

pub enum Decompression {
    Packed10Bit,
    Packed12Bit,
}

impl Decompression {
    pub fn get_decompression_type(compression: &u32) -> Result<Self, Error> {
        match compression {
            256 => Ok(Self::Packed10Bit),
            1024 => Ok(Self::Packed12Bit),
            _ => Err(Error),
        }
    }
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u16>, Error> {
        match self {
            Self::Packed10Bit => Ok(Self::decompress_10bit_packed(data)),
            Self::Packed12Bit => Ok(Self::decompress_12bit_packed(data)),
        }
    }
    /// Unpack 10-bit packed Bayer/greyscale into Vec<u16>
    /// bi_compression=256 means that there is 4 pixles of 10-bit data stored in 5 bytes(40-bits).
    fn decompress_10bit_packed(data: &[u8]) -> Vec<u16> {
        let mut out: Vec<u16> = Vec::with_capacity(data.len() * 4 / 5);

        let mut i: usize = 0;
        while i + 4 < data.len() {
            // set the values for each 4 pixels. assume they're ordered as;
            // 00000000 00|000000 0000|0000 000000|00 00000000
            // ----p0-- --|----p1 ----|---- p2----|-- p3------
            // turns into;
            // xxxxxx00 00000000 xxxxxx00 00000000 xxxxxx00 00000000 xxxxxx00 00000000
            // --------p0------- --------p1------- --------p2------- --------p3-------
            // and;
            // p0 starts in the top left corner of the frame.
            out.push(((data[i] as u16) << 2) | ((data[i + 1] as u16) >> 6));
            out.push((((data[i + 1] & 0b0011_1111) as u16) << 4) | ((data[i + 2] as u16) >> 4));
            out.push((((data[i + 2] & 0b0000_1111) as u16) << 6) | ((data[i + 3] as u16) >> 2));
            out.push((((data[i + 3] & 0b0000_0011) as u16) << 8) | (data[i + 4] as u16));
            i += 5;
        }
        out
    }

    /// Unpack 12-bit packed Bayer/greyscale into Vec<u16>
    /// bi_compression=1024 means that there is 2 pixles of 12-bit data stored in 3 bytes(24-bits).
    fn decompress_12bit_packed(data: &[u8]) -> Vec<u16> {
        let mut out = Vec::with_capacity(data.len() * 2 / 3);
        let mut i: usize = 0;
        while i + 3 < data.len() {
            // set the values for each 2 pixels. assume they're ordered as;
            // 00000000 0000|0000 00000000
            // ------p0 ----|---- p1-----|
            // turns into;
            // xxxx0000 00000000 xxxx0000 00000000
            // --------p0------- --------p1-------
            // and;
            // p0 starts in the top left corner of the frame.
            out.push(((data[i] as u16) << 4) | ((data[i + 1] as u16) >> 4));
            out.push((((data[i + 1] & 0b0000_1111) as u16) << 8) | (data[i + 2] as u16));
            i += 3
        }
        out
    }
}
