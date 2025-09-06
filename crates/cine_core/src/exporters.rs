use crate::errors::CineResult;
use base64::{Engine as _, engine::general_purpose};
use bytemuck;
use image::ImageEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use std::cell::RefCell;
use std::convert::AsRef;

#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Base64,
    Bytes,
    Png,
    Raw,
}

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

impl FrameType {
    pub fn format(&self, pixels: &[u16], width: u32, height: u32) -> CineResult<FrameData> {
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

#[derive(Debug, Clone, Copy)]
pub enum SaveType {
    Jpeg,
    Mp4,
    Png,
}

#[derive(Debug)]
pub enum SaveData {
    Jpeg(Vec<u8>),
    Mp4(Vec<u8>),
    Png(Vec<u8>),
}

impl AsRef<[u8]> for SaveData {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Jpeg(s) => s,
            Self::Mp4(s) => s,
            Self::Png(s) => s,
        }
    }
}

impl SaveType {
    pub fn format(&self, pixels: &[u16], width: u32, height: u32) -> CineResult<SaveData> {
        match self {
            Self::Jpeg => Ok(SaveData::Jpeg(Self::return_jpeg(pixels, width, height)?)),
            Self::Mp4 => Ok(SaveData::Mp4(Self::return_mp4(pixels, width, height)?)),
            Self::Png => Ok(SaveData::Png(Self::return_png(pixels, width, height)?)),
        }
    }

    fn return_jpeg(pixels: &[u16], width: u32, height: u32) -> CineResult<Vec<u8>> {
        todo!()
    }

    fn return_mp4(pixels: &[u16], width: u32, height: u32) -> CineResult<Vec<u8>> {
        todo!()
    }

    fn return_png(pixels: &[u16], width: u32, height: u32) -> CineResult<Vec<u8>> {
        FrameType::return_png(pixels, width, height)
    }
}
