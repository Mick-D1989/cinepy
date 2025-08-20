use crate::{cine, conversions};
use image::{ImageBuffer, Luma};
use memmap2::Mmap;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;

#[pyclass(module = "cinepy", name = "CineFile")]
pub struct CineFile {
    pub file: Mmap,
    #[pyo3(get)]
    pub cine_file_header: cine::CineFileHeader,
    #[pyo3(get)]
    pub bitmap_info_header: cine::BitmapInfoHeader,
    #[pyo3(get)]
    pub setup: cine::Setup,
    pub p_images: Vec<i64>,
}

// Implimentation for reading the file and setting the header info
#[pymethods]
impl CineFile {
    #[new]
    pub fn new(path: &str) -> Self {
        let mut tmp_file = File::open(path).expect("Could not open file");
        let file = unsafe { Mmap::map(&tmp_file).expect("Could not map file") };
        // Read CINEFILEHEADER
        let header: cine::CineFileHeader = read_structs(&mut tmp_file).unwrap();

        // Read BITMAPINFOHEADER
        tmp_file
            .seek(SeekFrom::Start(header.offset_image_header as u64))
            .unwrap();
        let bitmap: cine::BitmapInfoHeader = read_structs(&mut tmp_file).unwrap();

        // Read SETUP
        tmp_file
            .seek(SeekFrom::Start(header.offset_setup as u64))
            .unwrap();
        let packed_setup: cine::PackedSetup = read_structs(&mut tmp_file).unwrap();
        let setup: cine::Setup = cine::Setup::from(packed_setup);

        // Read frame offsets
        let image_count = header.image_count as usize;
        let mut p_images = vec![0i64; image_count];
        tmp_file
            .seek(SeekFrom::Start(header.offset_image_offsets as u64))
            .unwrap();
        for image in p_images.iter_mut().take(image_count) {
            let mut buf = [0u8; 8];
            tmp_file.read_exact(&mut buf).unwrap();
            *image = i64::from_le_bytes(buf);
        }

        Self {
            file,
            cine_file_header: header,
            bitmap_info_header: bitmap,
            setup,
            p_images,
        }
    }

    fn get_frame(&mut self, frame_no: i32) -> Vec<u16> {
        if frame_no >= self.cine_file_header.image_count as i32 {
            panic!("Frame requested {} does not exist...", frame_no);
        }

        let pixel_buffer_size = self.bitmap_info_header.bi_size_image as usize;
        // Get the start byte of the image's annotations
        let annotations_loc = self.p_images[frame_no as usize] as usize;

        // --- I/O OPTIMIZATION ---
        // No more seeks or reads! Just slice the mmap.

        // 1. Get the offset from the annotation block
        let anno_offset_bytes = &self.file[annotations_loc..annotations_loc + 4];
        let offset_to_pixels = u32::from_le_bytes(anno_offset_bytes.try_into().unwrap());

        // 2. Get the pixels
        let pixel_start = annotations_loc + offset_to_pixels as usize;
        let pixel_end = pixel_start + pixel_buffer_size;
        let pixel_buffer: &[u8] = &self.file[pixel_start..pixel_end];
        // --- End of I/O ---

        // The rest of your CPU-bound logic remains the same
        let mut unpacked_pixels: Vec<u16> = conversions::decompress_10bit_packed(pixel_buffer);
        let width: u32 = self.bitmap_info_header.bi_width as u32;
        let height: u32 = self.bitmap_info_header.bi_height as u32;

        if self.setup.bFlipV == 1 {
            conversions::flip_vertical_16bit(&mut unpacked_pixels, width, height);
        }

        conversions::apply_lut_10_to_12(&mut unpacked_pixels);
        conversions::grayscale_10_to_16bit(&mut unpacked_pixels);
        unpacked_pixels
    }
    // fn get_frame(&mut self, frame_no: i32) -> Vec<u16> {
    //     // Check request frame actually exists, rq if it doesnt
    //     if frame_no > (self.cine_file_header.image_count as i32) {
    //         panic!("Frame requested {} does not exist in the file provided with total length of frames {}", frame_no, (self.cine_file_header.image_count-1))
    //     }

    //     let pixel_buffer_size: u32 = self.bitmap_info_header.bi_size_image;
    //     // get the start byte of the image requesteds annotations
    //     let annotations_loc: i64 = self.p_images[frame_no as usize];
    //     // Get the size of the annotations so we can skip it and get to the start of the pixel location
    //     self.file
    //         .seek(SeekFrom::Start(annotations_loc as u64))
    //         .unwrap();
    //     let mut anno_offset_buf = [0u8; 4];
    //     self.file.read_exact(&mut anno_offset_buf).unwrap();
    //     let offset_to_pixels = u32::from_le_bytes(anno_offset_buf);

    //     // Get the pixels
    //     self.file
    //         .seek(SeekFrom::Start(
    //             (annotations_loc + offset_to_pixels as i64) as u64,
    //         ))
    //         .unwrap();
    //     let mut pixel_buffer = vec![0u8; pixel_buffer_size as usize];
    //     self.file.read_exact(&mut pixel_buffer).unwrap();

    //     let mut unpacked_pixels: Vec<u16> = conversions::decompress_10bit_packed(&pixel_buffer);
    //     let width: u32 = self.bitmap_info_header.bi_width as u32;
    //     let height: u32 = self.bitmap_info_header.bi_height as u32;

    //     if self.setup.bFlipV == 1 {
    //         conversions::flip_vertical_16bit(&mut unpacked_pixels, width, height);
    //     }

    //     conversions::apply_lut_10_to_12(&mut unpacked_pixels);
    //     conversions::grayscale_10_to_16bit(&mut unpacked_pixels);
    //     unpacked_pixels
    // }

    fn save_single_frame(&mut self, frame_no: i32, out_path: String) {
        let width: u32 = self.bitmap_info_header.bi_width as u32;
        let height: u32 = self.bitmap_info_header.bi_height as u32;
        let pixels = CineFile::get_frame(self, frame_no);

        let img: ImageBuffer<Luma<u16>, Vec<u16>> =
            ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(width, height, pixels).expect("pls work?");

        img.save(out_path).expect("ohes nose");
    }

    // fn save_single_frame(&mut self, frame_no: i32, out_path: String) {
    //     let width: u32 = self.bitmap_info_header.bi_width as u32;
    //     let height: u32 = self.bitmap_info_header.bi_height as u32;
    //     let rgb_pixels = CineFile::get_frame(self, frame_no);

    //     let mut img = RgbImage::new(width, height);

    //     for y in 0..height {
    //         for x in 0..width {
    //             let idx: usize = ((y * width + x) * 3) as usize;
    //             let r: u8 = rgb_pixels[idx];
    //             let g: u8 = rgb_pixels[idx + 1];
    //             let b: u8 = rgb_pixels[idx + 2];
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
