use crate::errors::CineResult;
use base64::{Engine as _, engine::general_purpose};
use image::ImageEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ImageBuffer, ImageFormat, Luma};
use std::cell::RefCell;
use std::io::Cursor;

// This enum is the solution to your mixed-type return problem.
// It wraps all possible data formats that can be returned.
#[derive(Debug)]
pub enum FrameData {
    Base64(String),
    Bytes(Vec<u8>),
    Png(Vec<u8>),
    Raw(Vec<u16>),
}

#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Base64,
    Bytes,
    Png,
    Raw,
}

impl FrameType {
    // The function now returns our wrapper enum, `FrameData`.
    // Note the return type is now `CineResult<FrameData>`.
    pub fn get_frame_from_frametype(
        &self,
        pixels: &[u16],
        width: u32,
        height: u32,
    ) -> CineResult<FrameData> {
        match self {
            // Each variant now wraps its result in the `FrameData` enum
            Self::Base64 => Ok(FrameData::Base64(Self::return_base64(pixels)?)),
            Self::Bytes => Ok(FrameData::Bytes(Self::return_bytes(pixels)?)),
            Self::Png => Ok(FrameData::Png(Self::return_png(pixels, width, height)?)),
            Self::Raw => Ok(FrameData::Raw(Self::return_raw(pixels)?)),
        }
    }

    pub fn save_frame_from_frametype(
        &self,
        pixels: &[u16],
        width: u32,
        height: u32,
    ) -> CineResult<FrameData> {
        match self {
            // Each variant now wraps its result in the `FrameData` enum
            Self::Base64 => Ok(FrameData::Base64(Self::return_base64(pixels)?)),
            Self::Bytes => Ok(FrameData::Bytes(Self::return_bytes(pixels)?)),
            Self::Png => Ok(FrameData::Png(Self::return_png(pixels, width, height)?)),
            Self::Raw => Ok(FrameData::Raw(Self::return_raw(pixels)?)),
        }
    }

    // --- Private helper functions ---
    // Note: I've added placeholder implementations for these to make the code runnable.

    fn return_base64(pixels: &[u16]) -> CineResult<String> {
        todo!()
    }

    fn return_bytes(pixels: &[u16]) -> CineResult<Vec<u8>> {
        todo!()
    }

    fn return_png(pixels: &[u16], width: u32, height: u32) -> CineResult<Vec<u8>> {
        thread_local! {
            static PNG_BUF: std::cell::RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        }

        PNG_BUF.with(|buf_cell| {
            let mut buf = buf_cell.borrow_mut();
            buf.clear();

            let encoder = PngEncoder::new_with_quality(
                &mut *buf,
                CompressionType::Fast,
                FilterType::NoFilter,
            );

            let _ = encoder.write_image(
                bytemuck::cast_slice(pixels),
                width,
                height,
                image::ExtendedColorType::L16,
            );

            // clone once here → safe for PyO3, Python gets its own copy
            Ok(buf.clone())
        })
    }

    // fn return_png(pixels: &[u16], width: u32, height: u32) -> CineResult<Vec<u8>> {
    //     let img =
    //         ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(width, height, pixels.to_vec()).unwrap();

    //     let mut img_png: Vec<u8> = Vec::new();
    //     img.write_to(&mut Cursor::new(&mut img_png), ImageFormat::Png)
    //         .expect("Failed to convert to png");
    //     Ok(img_png)
    // }

    fn return_raw(pixels: &[u16]) -> CineResult<Vec<u16>> {
        // To return an owned Vec, we must clone the data from the slice.
        Ok(pixels.to_vec())
    }
}

pub enum EncoderType {
    Base64,
}

impl EncoderType {
    // This function signature was incomplete. It needs to return a Result.
    pub fn encode(&self, frame: FrameData) -> CineResult<()> {
        // Placeholder for encoding logic
        Ok(())
    }
}
