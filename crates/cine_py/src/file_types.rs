use crate::cine;
use crate::conversions::ColorFilterArray;
use crate::decompress::Decompression;
use base64::{Engine as _, engine::general_purpose};
use image::{ImageBuffer, ImageFormat, Luma, Rgb};
use pyo3::PyErr;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, Cursor, Read, Seek, SeekFrom};
use std::mem;

#[pyclass(module = "cinepy", name = "CineFile")]
pub struct CineFile {
    pub file: File,
    #[pyo3(get)]
    pub cine_file_header: cine::CineFileHeader,
    #[pyo3(get)]
    pub bitmap_info_header: cine::BitmapInfoHeader,
    #[pyo3(get)]
    pub setup: cine::Setup,
    p_images: Vec<i64>,
    compression_type: Decompression,
    cfa: ColorFilterArray,
}

// Implimentation for reading the file and setting the header info
#[pymethods]
impl CineFile {
    #[new]
    pub fn new(path: &str) -> Result<Self, PyErr> {
        let mut file = File::open(path).unwrap();
        // Read CINEFILEHEADER
        let cine_file_header: cine::CineFileHeader = read_structs(&mut file).unwrap();

        // Read BITMAPINFOHEADER
        file.seek(SeekFrom::Start(cine_file_header.offset_image_header as u64))
            .unwrap();
        let bitmap_info_header: cine::BitmapInfoHeader = read_structs(&mut file).unwrap();

        // Read SETUP
        file.seek(SeekFrom::Start(cine_file_header.offset_setup as u64))
            .unwrap();
        let packed_setup: cine::PackedSetup = read_structs(&mut file).unwrap();
        let setup: cine::Setup = cine::Setup::from(packed_setup);

        // Read frame offsets
        let image_count = cine_file_header.image_count as usize;
        file.seek(SeekFrom::Start(
            cine_file_header.offset_image_offsets as u64,
        ))
        .unwrap();

        let total_bytes = image_count
            .checked_mul(std::mem::size_of::<i64>())
            .expect("Image count is too large");

        let mut buffer = vec![0u8; total_bytes];
        file.read_exact(&mut buffer).unwrap();

        let p_images: Vec<i64> = buffer
            .chunks_exact(8)
            .map(|chunk| i64::from_le_bytes(chunk.try_into().unwrap()))
            .collect();

        Ok(Self {
            file,
            cine_file_header,
            bitmap_info_header,
            setup,
            p_images,
            compression_type: Decompression::get_decompression_type(
                &bitmap_info_header.bi_compression,
            )
            .unwrap(),
            cfa: ColorFilterArray::get_cfa(&setup.CFA).unwrap(),
        })
    }

    pub fn get_frame(&mut self, frame_no: i32) -> Result<Vec<u16>, PyErr> {
        // TODO: Split this into two functions 1) gets the raw bytes, 2) applies corrections.

        // TODO: Turn this into proper error handling
        // Check request frame actually exists, rq if it doesnt.
        // if frame_no >= self.cine_file_header.image_count as i32 {
        //     return PyErr;
        // }

        let pixel_buffer_size: u32 = self.bitmap_info_header.bi_size_image;
        // get the start byte of the image requesteds annotations
        let annotations_loc: i64 = self.p_images[frame_no as usize];
        // Get the size of the annotations so we can skip it and get to the start of the pixel location
        self.file
            .seek(SeekFrom::Start(annotations_loc as u64))
            .unwrap();
        let mut anno_offset_buf = [0u8; 4];
        self.file.read_exact(&mut anno_offset_buf).unwrap();
        let offset_to_pixels = u32::from_le_bytes(anno_offset_buf);

        // Get the raw pixels
        self.file
            .seek(SeekFrom::Start(
                (annotations_loc + offset_to_pixels as i64) as u64,
            ))
            .unwrap();
        let mut pixel_buffer = vec![0u8; pixel_buffer_size as usize];
        self.file.read_exact(&mut pixel_buffer).unwrap();

        // uncompress them into a new vector large enough to hold the decompressed pixels
        let mut decompressed_pixels =
            Decompression::decompress(&self.compression_type, &pixel_buffer).unwrap();
        // apply corrections to the decompressed pixels

        if self.setup.CFA != 0 {
            let decompressed_pixels =
                ColorFilterArray::apply_color_array(&self.cfa, &mut decompressed_pixels).unwrap();
            return Ok(decompressed_pixels.unwrap());
        } else {
            ColorFilterArray::apply_color_array(&self.cfa, &mut decompressed_pixels).unwrap();
        }
        Ok(decompressed_pixels)
    }

    pub fn save_single_frame(&mut self, frame_no: i32, out_path: String) {
        let width: u32 = self.bitmap_info_header.bi_width as u32;
        let height: u32 = self.bitmap_info_header.bi_height as u32;
        let pixels = CineFile::get_frame(self, frame_no);
        // apply_gamma(self, &mut pixels);
        let img = ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(width, height, pixels.unwrap())
            .expect("pls work?");

        img.save(out_path).expect("ohes nose");
    }

    pub fn save_single_colour_frame(
        &mut self,
        frame_no: i32,
        out_path: String,
    ) -> Result<(), PyErr> {
        let width: u32 = self.bitmap_info_header.bi_width as u32;
        let height: u32 = self.bitmap_info_header.bi_height as u32;
        let pixels = CineFile::get_frame(self, frame_no)?;
        // apply_gamma(self, &mut pixels);
        let img = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_raw(width, height, pixels).unwrap();

        img.save(out_path).expect("ohes nose");
        Ok(())
    }

    pub fn base64_png(&mut self, frame_no: i32) -> Result<String, PyErr> {
        let width: u32 = self.bitmap_info_header.bi_width as u32;
        let height: u32 = self.bitmap_info_header.bi_height as u32;
        let pixels = CineFile::get_frame(self, frame_no)?;
        let img = ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(width, height, pixels).unwrap();

        let mut img_png: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut img_png), ImageFormat::Png)
            .expect("Failed to convert to png");
        Ok(general_purpose::STANDARD.encode(img_png))
    }

    pub fn get_frame_as_png(&mut self, frame_no: i32) -> Result<Vec<u8>, PyErr> {
        let width: u32 = self.bitmap_info_header.bi_width as u32;
        let height: u32 = self.bitmap_info_header.bi_height as u32;
        let pixels = CineFile::get_frame(self, frame_no)?;
        let img = ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(width, height, pixels).unwrap();

        let mut img_png: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut img_png), ImageFormat::Png)
            .expect("Failed to convert to png");
        Ok(img_png)
    }
    // fn save_single_colour_frame(&mut self, frame_no: i32, out_path: String) {
    //     let width: u32 = self.bitmap_info_header.bi_width as u32;
    //     let height: u32 = self.bitmap_info_header.bi_height as u32;
    //     let rgb_pixels = CineFile::get_frame(self, frame_no);

    //     let mut img: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageBuffer::new(width, height);

    //     for y in 0..height {
    //         for x in 0..width {
    //             let idx: usize = ((y * width + x) * 3) as usize;
    //             let r: u16 = rgb_pixels[idx];
    //             let g: u16 = rgb_pixels[idx + 1];
    //             let b: u16 = rgb_pixels[idx + 2];
    //             img.put_pixel(x, y, Rgb([r, g, b]));
    //         }
    //     }

    //     img.save(out_path).unwrap();
    // }
}

fn read_structs<T: Copy, R: Read>(mut reader: R) -> io::Result<T> {
    let buf_size = mem::size_of::<T>();
    let mut buffer = vec![0u8; buf_size];
    reader.read_exact(&mut buffer)?;
    let buffer_ptr = buffer.as_ptr() as *const T;
    let result = unsafe { buffer_ptr.read_unaligned() };
    Ok(result)
}

#[cfg(test)]
mod tests {

    #[test]

    fn test_tests() {
        let a = 2;
        let b = 5;
        assert_eq!(a * b, 10);
    }
}
