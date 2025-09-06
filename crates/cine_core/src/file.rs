use crate::cine;
use crate::conversions::ColorFilterArray;
use crate::decompress::Decompression;
use crate::errors::CineResult;
use crate::exporters::{FrameData, FrameType, SaveData, SaveType};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

// The operations a caller can make on a generic video type
pub trait VideoOps {
    fn get_headers(&self) -> CineResult<VideoHeader>;
    fn get_frame_as(&mut self, frame_no: i32, frame_type: FrameType) -> CineResult<FrameData>; // Returns either a Vec<u8> or Vec<u16> in the format of bytes, PNG representation, etc
    fn save_frame_as(&mut self, frame_no: i32, save_type: SaveType, f_pth: &str) -> CineResult<()>;
}

pub struct VideoHeader {
    pub file_name: String,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

pub struct CineFile {
    pub file: File,
    pub cine_file_header: cine::CineFileHeader,
    pub bitmap_info_header: cine::BitmapInfoHeader,
    pub setup: cine::Setup,
    p_images: Vec<i64>,
    pub compression_type: Decompression,
    pub cfa: ColorFilterArray,
    pub img_byte_buffer: Vec<u8>,
    pub pixel_buffer: Vec<u16>,
    pub pixels: Vec<u16>,
}

impl CineFile {
    pub fn open(path: &str) -> CineResult<CineFile> {
        let mut file = File::open(path)?;
        // Read CINEFILEHEADER
        let cine_file_header: cine::CineFileHeader = Self::read_structs(&mut file)?;

        // Read BITMAPINFOHEADER
        file.seek(SeekFrom::Start(cine_file_header.offset_image_header as u64))
            .unwrap();
        let bitmap_info_header: cine::BitmapInfoHeader = Self::read_structs(&mut file)?;

        // Read SETUP
        file.seek(SeekFrom::Start(cine_file_header.offset_setup as u64))?;
        let packed_setup: cine::PackedSetup = Self::read_structs(&mut file)?;
        let setup: cine::Setup = cine::Setup::from(packed_setup);

        // Read frame offsets
        let image_count = cine_file_header.image_count as usize;
        file.seek(SeekFrom::Start(
            cine_file_header.offset_image_offsets as u64,
        ))?;

        let total_bytes = image_count
            .checked_mul(std::mem::size_of::<i64>())
            .expect("Image count is too large");

        let mut buffer = vec![0u8; total_bytes];
        file.read_exact(&mut buffer)?;

        let p_images: Vec<i64> = buffer
            .chunks_exact(8)
            .map(|chunk| i64::from_le_bytes(chunk.try_into().unwrap()))
            .collect();

        let img_byte_buffer = vec![0u8; bitmap_info_header.bi_size_image as usize];

        let compression_type =
            Decompression::get_decompression_type(&bitmap_info_header.bi_compression)?;

        let pixel_buffer = if compression_type == Decompression::Packed10Bit {
            vec![0u16; img_byte_buffer.capacity() * 4 / 5]
        } else if compression_type == Decompression::Packed12Bit {
            vec![0u16; img_byte_buffer.capacity() * 2 / 3]
        } else {
            vec![0u16; img_byte_buffer.capacity()]
        };

        let cfa = ColorFilterArray::get_cfa(&setup.CFA).unwrap();

        let pixels = if cfa == ColorFilterArray::Gray {
            vec![0u16; (bitmap_info_header.bi_width * bitmap_info_header.bi_height) as usize]
        } else {
            vec![0u16; (bitmap_info_header.bi_width * bitmap_info_header.bi_height * 3) as usize]
        };

        Ok(CineFile {
            file,
            cine_file_header,
            bitmap_info_header,
            setup,
            p_images,
            compression_type,
            cfa,
            img_byte_buffer,
            pixel_buffer,
            pixels,
        })
    }

    pub fn get_frame(&mut self, frame_no: i32) -> CineResult<()> {
        self.get_bytes(frame_no)?;
        self.apply_correction()
    }

    fn read_structs<T: Copy, R: Read>(mut reader: R) -> CineResult<T> {
        let buf_size = mem::size_of::<T>();
        let mut buffer = vec![0u8; buf_size];
        reader.read_exact(&mut buffer)?;
        let buffer_ptr = buffer.as_ptr() as *const T;
        let result = unsafe { buffer_ptr.read_unaligned() };
        Ok(result)
    }

    fn get_bytes(&mut self, frame_no: i32) -> CineResult<()> {
        // Get the size of the annotations so we can skip them
        // and get to the start of the pixel data location.

        let annotations_loc: i64 = self.p_images[frame_no as usize];
        self.file.seek(SeekFrom::Start(annotations_loc as u64))?;
        let mut anno_offset_buf = [0u8; 4];
        self.file.read_exact(&mut anno_offset_buf)?;
        let offset_to_pixels = u32::from_le_bytes(anno_offset_buf);

        // Get the raw pixels.
        self.file.seek(SeekFrom::Start(
            (annotations_loc + offset_to_pixels as i64) as u64,
        ))?;
        Ok(self.file.read_exact(&mut self.img_byte_buffer)?)
    }

    fn apply_correction(&mut self) -> CineResult<()> {
        Decompression::decompress(self)?;
        ColorFilterArray::apply_color_array(self)
    }
}

impl VideoOps for CineFile {
    fn get_headers(&self) -> CineResult<VideoHeader> {
        Ok(VideoHeader {
            file_name: "temp.cine".to_string(), // placeholder, need to drop all the null bytes from what the field actually takes
            width: self.bitmap_info_header.bi_width as u32,
            height: self.bitmap_info_header.bi_height as u32,
            frame_count: self.cine_file_header.image_count,
        })
    }

    fn get_frame_as(&mut self, frame_no: i32, frame_type: FrameType) -> CineResult<FrameData> {
        let width = self.bitmap_info_header.bi_width as u32;
        let height = self.bitmap_info_header.bi_height as u32;

        self.get_frame(frame_no)?;
        frame_type.format(&self.pixels, width, height)
    }

    fn save_frame_as(&mut self, frame_no: i32, save_type: SaveType, f_pth: &str) -> CineResult<()> {
        let width = self.bitmap_info_header.bi_width as u32;
        let height = self.bitmap_info_header.bi_height as u32;

        self.get_frame(frame_no)?;
        let img = save_type.format(&self.pixels, width, height)?;
        Ok(std::fs::write(f_pth, &img)?)
    }
}

#[derive(Clone, Copy)]
pub struct Mp4File {}

impl Mp4File {
    pub fn open(path: &str) -> CineResult<Mp4File> {
        todo!()
    }
}

impl VideoOps for Mp4File {
    fn get_headers(&self) -> CineResult<VideoHeader> {
        todo!()
    }
    fn get_frame_as(&mut self, frame_no: i32, frame_type: FrameType) -> CineResult<FrameData> {
        todo!()
    }
    fn save_frame_as(&mut self, frame_no: i32, save_type: SaveType, f_pth: &str) -> CineResult<()> {
        todo!()
    }
}
