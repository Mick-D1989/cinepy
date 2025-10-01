use cine_core::cine::{
    BitmapInfoHeader, CineFileHeader, IMFilter, Rect, Setup, TC, Time64, WBGain,
};
use cine_core::utils::c_char_array_to_string;
use pyo3::prelude::*;

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone, Copy)]
pub struct PyWBGain {
    pub r: f32,
    pub b: f32,
}
impl From<WBGain> for PyWBGain {
    fn from(val: WBGain) -> Self {
        Self { r: val.R, b: val.B }
    }
}

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
pub struct PyIMFilter {
    pub dim: i32,
    pub shifts: i32,
    pub bias: i32,
    pub coef: Vec<i32>,
}
impl From<IMFilter> for PyIMFilter {
    fn from(val: IMFilter) -> Self {
        Self {
            dim: val.dim,
            shifts: val.shifts,
            bias: val.bias,
            coef: val.Coef.to_vec(),
        }
    }
}

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone, Copy)]
pub struct PyRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl From<Rect> for PyRect {
    fn from(val: Rect) -> Self {
        Self {
            left: val.left,
            top: val.top,
            right: val.right,
            bottom: val.bottom,
        }
    }
}

#[pyclass(get_all)]
#[derive(Debug, Clone, Copy)]
pub struct PyTC {
    pub time_code: u32,
    pub user_bits: u32,
}
impl From<TC> for PyTC {
    fn from(val: TC) -> Self {
        Self {
            time_code: val.time_code,
            user_bits: val.user_bits,
        }
    }
}

#[pyclass(get_all)]
#[derive(Debug, Clone, Copy)]
pub struct PyTime64 {
    pub fractions: u32,
    pub seconds: u32,
}

impl From<Time64> for PyTime64 {
    fn from(core: Time64) -> Self {
        Self {
            fractions: core.fractions,
            seconds: core.seconds,
        }
    }
}

#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct PySetup {
    pub frame_rate_16: u16,
    pub shutter_16: u16,
    pub post_trigger_16: u16,
    pub frame_delay_16: u16,
    pub aspect_ratio: u16,
    pub description_old: String,
    pub samples_per_image: u8,
    pub bin_name: Vec<String>,
    pub ana_channels: i16,
    pub ch_option: Vec<i16>,
    pub ana_gain: Vec<f32>,
    pub ana_unit: Vec<String>,
    pub ana_name: Vec<String>,
    pub im_width: u16,
    pub im_height: u16,
    pub edr_shutter_16: u16,
    pub serial: u32,
    pub saturation: i32,
    pub auto_exposure: u32,
    pub b_flip_h: bool,
    pub b_flip_v: bool,
    pub grid: u32,
    pub frame_rate: u32,
    pub shutter: u32,
    pub edr_shutter: u32,
    pub post_trigger: u32,
    pub frame_delay: u32,
    pub b_enable_color: bool,
    pub camera_version: u32,
    pub firmware_version: u32,
    pub software_version: u32,
    pub recording_time_zone: i32,
    pub cfa: u32,
    pub bright: i32,
    pub contrast: i32,
    pub gamma: i32,
    pub auto_exp_level: u32,
    pub auto_exp_speed: u32,
    pub auto_exp_rect: PyRect,
    pub wb_gain: Vec<PyWBGain>,
    pub rotate: i32,
    pub wb_view: PyWBGain,
    pub real_bpp: u32,
    pub conv_8_min: u32,
    pub conv_8_max: u32,
    pub uf: PyIMFilter,
    pub b_stamp_time: bool,
    pub frp_steps: u32,
    pub frp_img_nr: Vec<i32>,
    pub frp_rate: Vec<u32>,
    pub frp_exp: Vec<u32>,
    pub mc_cnt: i32,
    pub mc_percent: Vec<f32>,
    pub ci_calib: u32,
    pub head_serial: Vec<u32>,
    pub sensor: u32,
    pub shutter_ns: u32,
    pub edr_shutter_ns: u32,
    pub frame_delay_ns: u32,
    pub im_pos_x_acq: u32,
    pub im_pos_y_acq: u32,
    pub im_width_acq: u32,
    pub im_height_acq: u32,
    pub description: String,
    pub rising_edge: bool,
    pub b_meta_wb: bool,
    pub hue: i32,
    pub black_level: i32,
    pub white_level: i32,
    pub lens_description: String,
    pub lens_aperture: f32,
    pub lens_focus_distance: f32,
    pub lens_focal_length: f32,
    pub f_offset: f32,
    pub f_gain: f32,
    pub f_saturation: f32,
    pub f_hue: f32,
    pub f_gamma: f32,
    pub f_gamma_r: f32,
    pub f_gamma_b: f32,
    pub f_flare: f32,
    pub f_pedestal_r: f32,
    pub f_pedestal_g: f32,
    pub f_pedestal_b: f32,
    pub f_chroma: f32,
    pub tone_label: String,
    pub tone_points: i32,
    pub f_tone: Vec<f32>,
    pub user_matrix_label: String,
    pub enable_matrices: bool,
    pub cm_user: Vec<f32>,
    pub enable_crop: bool,
    pub crop_rect: PyRect,
    pub enable_resample: bool,
    pub resample_width: u32,
    pub resample_height: u32,
    pub f_gain_16_8: f32,
    pub frp_shape: Vec<u32>,
    pub trig_tc: PyTC,
    pub f_pb_rate: f32,
    pub f_tc_rate: f32,
    pub cine_name: String,
    pub f_gain_r: f32,
    pub f_gain_g: f32,
    pub f_gain_b: f32,
    pub cm_calib: Vec<f32>,
    pub f_wb_temp: f32,
    pub f_wb_cc: f32,
    pub calibration_info: String,
    pub optical_filter: String,
    pub gps_info: String,
    pub uuid: String,
    pub created_by: String,
    pub rec_bpp: u32,
    pub lowest_format_bpp: u16,
    pub f_toe: f32,
    pub log_mode: u32,
    pub camera_model: String,
    pub wb_type: u32,
    pub d_frame_rate: f64,
    pub sensor_mode: u32,
    pub supports_binning: bool,
    pub ana_daq_description: String,
    pub bin_daq_description: String,
    pub daq_options: bool,
    pub sensor_options: u32,
}

impl From<Box<Setup>> for PySetup {
    fn from(core: Box<Setup>) -> Self {
        Self {
            frame_rate_16: core.FrameRate16,
            shutter_16: core.Shutter16,
            post_trigger_16: core.PostTrigger16,
            frame_delay_16: core.FrameDelay16,
            aspect_ratio: core.AspectRatio,
            description_old: c_char_array_to_string(&core.DescriptionOld),
            samples_per_image: core.SamplesPerImage,
            bin_name: core
                .BinName
                .iter()
                .map(|n| c_char_array_to_string(n))
                .collect(),
            ana_channels: core.AnaChannels,
            ch_option: core.ChOption.to_vec(),
            ana_gain: core.AnaGain.to_vec(),
            ana_unit: core
                .AnaUnit
                .iter()
                .map(|u| c_char_array_to_string(u))
                .collect(),
            ana_name: core
                .AnaName
                .iter()
                .map(|n| c_char_array_to_string(n))
                .collect(),
            im_width: core.ImWidth,
            im_height: core.ImHeight,
            edr_shutter_16: core.EDRShutter16,
            serial: core.Serial,
            saturation: core.Saturation,
            auto_exposure: core.AutoExposure,
            b_flip_h: core.bFlipH != 0, // Note: !=0 does a truthy conversion
            b_flip_v: core.bFlipV != 0,
            grid: core.Grid,
            frame_rate: core.FrameRate,
            shutter: core.Shutter,
            edr_shutter: core.EDRShutter,
            post_trigger: core.PostTrigger,
            frame_delay: core.FrameDelay,
            b_enable_color: core.bEnableColor != 0,
            camera_version: core.CameraVersion,
            firmware_version: core.FirmwareVersion,
            software_version: core.SoftwareVersion,
            recording_time_zone: core.RecordingTimeZone,
            cfa: core.CFA,
            bright: core.Bright,
            contrast: core.Contrast,
            gamma: core.Gamma,
            auto_exp_level: core.AutoExpLevel,
            auto_exp_speed: core.AutoExpSpeed,
            auto_exp_rect: core.AutoExpRect.into(),
            wb_gain: core.WBGain.iter().map(|&g| g.into()).collect(),
            rotate: core.Rotate,
            wb_view: core.WBView.into(),
            real_bpp: core.RealBPP,
            conv_8_min: core.Conv8Min,
            conv_8_max: core.Conv8Max,
            uf: core.UF.into(),
            b_stamp_time: core.bStampTime != 0,
            frp_steps: core.FRPSteps,
            frp_img_nr: core.FRPImgNr.to_vec(),
            frp_rate: core.FRPRate.to_vec(),
            frp_exp: core.FRPExp.to_vec(),
            mc_cnt: core.MCCnt,
            mc_percent: core.MCPercent.to_vec(),
            ci_calib: core.CICalib,
            head_serial: core.HeadSerial.to_vec(),
            sensor: core.Sensor,
            shutter_ns: core.ShutterNs,
            edr_shutter_ns: core.EDRShutterNs,
            frame_delay_ns: core.FrameDelayNs,
            im_pos_x_acq: core.ImPosXAcq,
            im_pos_y_acq: core.ImPosYAcq,
            im_width_acq: core.ImWidthAcq,
            im_height_acq: core.ImHeightAcq,
            description: c_char_array_to_string(&core.Description),
            rising_edge: core.RisingEdge != 0,
            b_meta_wb: core.bMetaWB != 0,
            hue: core.Hue,
            black_level: core.BlackLevel,
            white_level: core.WhiteLevel,
            lens_description: c_char_array_to_string(&core.LensDescription),
            lens_aperture: core.LensAperture,
            lens_focus_distance: core.LensFocusDistance,
            lens_focal_length: core.LensFocalLength,
            f_offset: core.fOffset,
            f_gain: core.fGain,
            f_saturation: core.fSaturation,
            f_hue: core.fHue,
            f_gamma: core.fGamma,
            f_gamma_r: core.fGammaR,
            f_gamma_b: core.fGammaB,
            f_flare: core.fFlare,
            f_pedestal_r: core.fPedestalR,
            f_pedestal_g: core.fPedestalG,
            f_pedestal_b: core.fPedestalB,
            f_chroma: core.fChroma,
            tone_label: c_char_array_to_string(&core.ToneLabel),
            tone_points: core.TonePoints,
            f_tone: core.fTone.to_vec(),
            user_matrix_label: c_char_array_to_string(&core.UserMatrixLabel),
            enable_matrices: core.EnableMatrices != 0,
            cm_user: core.cmUser.to_vec(),
            enable_crop: core.EnableCrop != 0,
            crop_rect: core.CropRect.into(),
            enable_resample: core.EnableResample != 0,
            resample_width: core.ResampleWidth,
            resample_height: core.ResampleHeight,
            f_gain_16_8: core.fGain16_8,
            frp_shape: core.FRPShape.to_vec(),
            trig_tc: core.TrigTC.into(),
            f_pb_rate: core.fPbRate,
            f_tc_rate: core.fTcRate,
            cine_name: c_char_array_to_string(&core.CineName),
            f_gain_r: core.fGainR,
            f_gain_g: core.fGainG,
            f_gain_b: core.fGainB,
            cm_calib: core.cmCalib.to_vec(),
            f_wb_temp: core.fWBTemp,
            f_wb_cc: core.fWBCc,
            calibration_info: c_char_array_to_string(&core.CalibrationInfo),
            optical_filter: c_char_array_to_string(&core.OpticalFilter),
            gps_info: c_char_array_to_string(&core.GpsInfo),
            uuid: c_char_array_to_string(&core.Uuid),
            created_by: c_char_array_to_string(&core.CreatedBy),
            rec_bpp: core.RecBPP,
            lowest_format_bpp: core.LowestFormatBPP,
            f_toe: core.fToe,
            log_mode: core.LogMode,
            camera_model: c_char_array_to_string(&core.CameraModel),
            wb_type: core.WBType,
            d_frame_rate: core.dFrameRate,
            sensor_mode: core.SensorMode,
            supports_binning: core.SupportsBinning != 0,
            ana_daq_description: c_char_array_to_string(&core.AnaDaqDescription),
            bin_daq_description: c_char_array_to_string(&core.BinDaqDescription),
            daq_options: core.DaqOptions != 0,
            sensor_options: core.SensorOptions,
        }
    }
}

#[pyclass(get_all)]
#[derive(Debug, Clone, Copy)]
pub struct PyBitmapInfoHeader {
    pub bi_size: u32,
    pub bi_width: i32,
    pub bi_height: i32,
    pub bi_planes: u16,
    pub bi_bit_count: u16,
    pub bi_compression: u32,
    pub bi_size_image: u32,
    pub bi_x_pels_per_meter: i32,
    pub bi_y_pels_per_meter: i32,
    pub bi_clr_used: u32,
    pub bi_clr_important: u32,
}

impl From<BitmapInfoHeader> for PyBitmapInfoHeader {
    fn from(core: BitmapInfoHeader) -> Self {
        Self {
            bi_size: core.bi_size,
            bi_width: core.bi_width,
            bi_height: core.bi_height,
            bi_planes: core.bi_planes,
            bi_bit_count: core.bi_bit_count,
            bi_compression: core.bi_compression,
            bi_size_image: core.bi_size_image,
            bi_x_pels_per_meter: core.bi_x_pels_per_meter,
            bi_y_pels_per_meter: core.bi_y_pels_per_meter,
            bi_clr_used: core.bi_clr_used,
            bi_clr_important: core.bi_clr_important,
        }
    }
}

#[pyclass(get_all)]
#[derive(Debug, Clone, Copy)]
pub struct PyCineFileHeader {
    pub type_marker: u16,
    pub header_size: u16,
    pub compression: u16,
    pub version: u16,
    pub first_movie_image: i32,
    pub total_image_count: u32,
    pub first_image_no: i32,
    pub image_count: u32,
    pub offset_image_header: u32,
    pub offset_setup: u32,
    pub offset_image_offsets: u32,
    pub trigger_time: PyTime64,
}

impl From<CineFileHeader> for PyCineFileHeader {
    fn from(core: CineFileHeader) -> Self {
        Self {
            type_marker: core.type_marker,
            header_size: core.header_size,
            compression: core.compression,
            version: core.version,
            first_movie_image: core.first_movie_image,
            total_image_count: core.total_image_count,
            first_image_no: core.first_image_no,
            image_count: core.image_count,
            offset_image_header: core.offset_image_header,
            offset_setup: core.offset_setup,
            offset_image_offsets: core.offset_image_offsets,
            trigger_time: core.trigger_time.into(),
        }
    }
}
