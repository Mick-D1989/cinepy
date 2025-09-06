use crate::errors::CineResult;
use base64::{Engine as _, engine::general_purpose};
use bytemuck;
use image::ImageEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use std::cell::RefCell;
use std::convert::AsRef;
use std::io::Cursor;

#[derive(Debug)]
pub enum FrameData {
    Base64(String),
    Bytes(Vec<u8>),
    Png(Vec<u8>),
    Raw(Vec<u16>),
}

impl AsRef<[u8]> for FrameData {
    fn as_ref(&self) -> &[u8] {
        match self {
            FrameData::Base64(s) => s.as_bytes(),
            FrameData::Bytes(s) => s,
            FrameData::Png(s) => s,
            FrameData::Raw(s) => bytemuck::cast_slice(s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Base64,
    Bytes,
    Png,
    Raw,
}

impl FrameType {
    pub fn get_frame_from_frametype(
        &self,
        pixels: &[u16],
        width: u32,
        height: u32,
    ) -> CineResult<FrameData> {
        match self {
            Self::Base64 => Ok(FrameData::Base64(Self::return_base64(
                pixels, width, height,
            )?)),
            Self::Bytes => Ok(FrameData::Bytes(Self::return_bytes(pixels)?)),
            Self::Png => Ok(FrameData::Png(Self::return_png(pixels, width, height)?)),
            Self::Raw => Ok(FrameData::Raw(Self::return_raw(pixels, width, height)?)),
        }
    }

    pub fn save_frame_from_frametype(
        &self,
        pixels: &[u16],
        width: u32,
        height: u32,
    ) -> CineResult<FrameData> {
        match self {
            Self::Base64 => Ok(FrameData::Base64(Self::return_base64(
                pixels, width, height,
            )?)),
            Self::Bytes => Ok(FrameData::Bytes(Self::return_bytes(pixels)?)),
            Self::Png => Ok(FrameData::Png(Self::return_png(pixels, width, height)?)),
            Self::Raw => Ok(FrameData::Raw(Self::return_raw(pixels, width, height)?)),
        }
    }

    fn return_base64(pixels: &[u16], width: u32, height: u32) -> CineResult<String> {
        // Returns a PNG encoded as base64
        let img_png = Self::return_png(pixels, width, height)?;
        Ok(general_purpose::STANDARD.encode(img_png))
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

            encoder.write_image(
                bytemuck::cast_slice(pixels),
                width,
                height,
                image::ExtendedColorType::L16,
            )?;

            let out_vec = std::mem::take(&mut *buf);

            Ok(out_vec)
        })
    }

    fn return_raw(pixels: &[u16], width: u32, height: u32) -> CineResult<Vec<u16>> {
        Ok(pixels.to_vec())
    }
}
