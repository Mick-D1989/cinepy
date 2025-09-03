use std::io::Error;

use crate::errors::CineResult;
use crate::file::CineFile;

#[derive(Clone, PartialEq)]
pub enum Decompression {
    Uncompressed,
    Packed10Bit,
    Packed12Bit,
}

impl Decompression {
    pub fn get_decompression_type(compression: &u32) -> Result<Self, Error> {
        match compression {
            0 => Ok(Self::Uncompressed),
            256 => Ok(Self::Packed10Bit),
            1024 => Ok(Self::Packed12Bit),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported compression type",
            )),
        }
    }
    pub fn decompress(cine_file: &mut CineFile) -> CineResult<()> {
        match cine_file.compression_type {
            Self::Uncompressed => Ok(Self::order_uncompressed(cine_file)?),
            Self::Packed10Bit => Ok(Self::decompress_10bit_packed(cine_file)?),
            Self::Packed12Bit => Ok(Self::decompress_12bit_packed(cine_file)?),
        }
    }
    /// Unpack 10-bit packed Bayer/greyscale into Vec<u16>
    /// bi_compression=256 means that there is 4 pixles of 10-bit data stored in 5 bytes(40-bits).
    fn decompress_10bit_packed(cine_file: &mut CineFile) -> CineResult<()> {
        // let mut out: Vec<u16> = Vec::with_capacity(data.len() * 4 / 5);

        let mut i: usize = 0;
        let mut j: usize = 0;

        while i + 4 < cine_file.img_byte_buffer.len() {
            // set the values for each 4 pixels. assume they're ordered as;
            // 00000000 00|000000 0000|0000 000000|00 00000000
            // ----p0-- --|----p1 ----|---- p2----|-- p3------
            // turns into;
            // xxxxxx00 00000000 xxxxxx00 00000000 xxxxxx00 00000000 xxxxxx00 00000000
            // --------p0------- --------p1------- --------p2------- --------p3-------
            // and;
            // p0 starts in the top left corner of the frame.
            cine_file.pixel_buffer[j] = ((cine_file.img_byte_buffer[i] as u16) << 2)
                | ((cine_file.img_byte_buffer[i + 1] as u16) >> 6);
            cine_file.pixel_buffer[j + 1] =
                (((cine_file.img_byte_buffer[i + 1] & 0b0011_1111) as u16) << 4)
                    | ((cine_file.img_byte_buffer[i + 2] as u16) >> 4);
            cine_file.pixel_buffer[j + 2] =
                (((cine_file.img_byte_buffer[i + 2] & 0b0000_1111) as u16) << 6)
                    | ((cine_file.img_byte_buffer[i + 3] as u16) >> 2);
            cine_file.pixel_buffer[j + 3] =
                (((cine_file.img_byte_buffer[i + 3] & 0b0000_0011) as u16) << 8)
                    | (cine_file.img_byte_buffer[i + 4] as u16);
            i += 5;
            j += 4;
        }
        Ok(())
    }

    /// Unpack 12-bit packed Bayer/greyscale into Vec<u16>
    /// bi_compression=1024 means that there is 2 pixles of 12-bit data stored in 3 bytes(24-bits).
    fn decompress_12bit_packed(cine_file: &mut CineFile) -> CineResult<()> {
        // let mut out = Vec::with_capacity(data.len() * 2 / 3);
        // let mut i: usize = 0;
        // while i + 3 < data.len() {
        //     // set the values for each 2 pixels. assume they're ordered as;
        //     // 00000000 0000|0000 00000000
        //     // ------p0 ----|---- p1-----|
        //     // turns into;
        //     // xxxx0000 00000000 xxxx0000 00000000
        //     // --------p0------- --------p1-------
        //     // and;
        //     // p0 starts in the top left corner of the frame.
        //     out.push(((data[i] as u16) << 4) | ((data[i + 1] as u16) >> 4));
        //     out.push((((data[i + 1] & 0b0000_1111) as u16) << 8) | (data[i + 2] as u16));
        //     i += 3
        // }
        // out
        todo!()
    }

    fn order_uncompressed(cine_file: &mut CineFile) -> CineResult<()> {
        // Uncompressed images are bottom up, which is the opposite of 10 & 12 bit compressed images
        // This flip the order of the pixels so they are top down so the corrections we apply later
        // on are consistent.
        todo!()
    }
}
