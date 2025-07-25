use alvr_common::{
    ALVR_VERSION, DebugGroupsConfig, DebugGroupsConfigDefault, LogSeverity, LogSeverityDefault,
    LogSeverityDefaultVariant,
};
use alvr_system_info::{ClientFlavor, ClientFlavorDefault, ClientFlavorDefaultVariant};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use settings_schema::{
    ArrayDefault, DictionaryDefault, OptionalDefault, SettingsSchema, Switch, SwitchDefault,
    VectorDefault,
};

include!(concat!(env!("OUT_DIR"), "/openvr_property_keys.rs"));

pub enum OpenvrPropType {
    Bool,
    Float,
    Int32,
    Uint64,
    Vector3,
    Double,
    String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug)]
pub struct OpenvrProperty {
    pub key: OpenvrPropKey,
    pub value: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum FrameSize {
    Scale(#[schema(gui(slider(min = 0.25, max = 2.0, step = 0.01)))] f32),

    Absolute {
        #[schema(gui(slider(min = 32, max = 8192, step = 32)))]
        width: u32,
        #[schema(gui(slider(min = 32, max = 8192, step = 32)))]
        height: Option<u32>,
    },
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncoderQualityPreset {
    Quality = 0,
    Balanced = 1,
    Speed = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncoderQualityPresetNvidia {
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum NvencTuningPreset {
    HighQuality = 1,
    LowLatency = 2,
    UltraLowLatency = 3,
    Lossless = 4,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum NvencMultiPass {
    Disabled = 0,
    #[schema(strings(display_name = "1/4 resolution"))]
    QuarterResolution = 1,
    FullResolution = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum NvencAdaptiveQuantizationMode {
    Disabled = 0,
    Spatial = 1,
    Temporal = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum RateControlMode {
    #[schema(strings(display_name = "CBR"))]
    Cbr = 0,
    #[schema(strings(display_name = "VBR"))]
    Vbr = 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum EntropyCoding {
    #[schema(strings(display_name = "CAVLC"))]
    Cavlc = 1,
    #[schema(strings(display_name = "CABAC"))]
    Cabac = 0,
}

/// Except for preset, the value of these fields is not applied if == -1 (flag)
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct NvencConfig {
    #[schema(strings(
        help = "P1 is the fastest preset and P7 is the preset that produces better quality. P6 and P7 are too slow to be usable."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub quality_preset: EncoderQualityPresetNvidia,
    #[schema(flag = "steamvr-restart")]
    pub tuning_preset: NvencTuningPreset,
    #[schema(strings(
        help = "Reduce compression artifacts at the cost of small performance penalty"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub multi_pass: NvencMultiPass,
    #[schema(strings(
        help = r#"Spatial: Helps reduce color banding, but high-complexity scenes might look worse.
Temporal: Helps improve overall encoding quality, very small trade-off in speed."#
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adaptive_quantization_mode: NvencAdaptiveQuantizationMode,
    #[schema(flag = "steamvr-restart")]
    pub low_delay_key_frame_scale: i64,
    #[schema(flag = "steamvr-restart")]
    pub refresh_rate: i64,
    #[schema(flag = "steamvr-restart")]
    pub enable_intra_refresh: bool,
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_period: i64,
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_count: i64,
    #[schema(flag = "steamvr-restart")]
    pub max_num_ref_frames: i64,
    #[schema(flag = "steamvr-restart")]
    pub gop_length: i64,
    #[schema(flag = "steamvr-restart")]
    pub p_frame_strategy: i64,
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_buffer_size: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_initial_delay: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_max_bitrate: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_average_bitrate: i64,
    #[schema(flag = "steamvr-restart")]
    pub enable_weighted_prediction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct AmfConfig {
    #[schema(
        strings(
            display_name = "Enable High-Motion Quality Boost",
            help = r#"Enables high motion quality boost mode.
Allows the encoder to perform pre-analysis the motion of the video and use the information for better encoding"#
        ),
        flag = "steamvr-restart"
    )]
    pub enable_hmqb: bool,
    #[schema(flag = "steamvr-restart")]
    pub use_preproc: bool,
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_sigma: u32,
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_tor: u32,
    #[schema(
        strings(
            display_name = "Enable Pre-analysis",
            help = r#"Enables pre-analysis during encoding. This will likely result in reduced performance, but may increase quality.
Does not work with the "Reduce color banding" option, requires enabling "Use preproc""#
        ),
        flag = "steamvr-restart"
    )]
    pub enable_pre_analysis: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct SoftwareEncodingConfig {
    #[schema(strings(
        display_name = "Force software encoding",
        help = "Forces the encoder to use CPU instead of GPU"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_software_encoding: bool,

    #[schema(strings(display_name = "Encoder thread count"))]
    #[schema(flag = "steamvr-restart")]
    pub thread_count: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct HDRConfig {
    #[schema(strings(
        display_name = "Enable HDR",
        help = "If the client has no preference, enables compositing VR layers to an RGBA float16 framebuffer, and doing sRGB/YUV conversions in shader code."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub enable: Option<bool>,

    #[schema(strings(
        display_name = "Force HDR sRGB Correction",
        help = "Forces sRGB correction on all composited SteamVR layers. Useful if an HDR-injected game is too dark."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_hdr_srgb_correction: bool,

    #[schema(strings(
        display_name = "Clamp HDR extended range",
        help = "Clamps HDR extended range to 0.0~1.0, useful if you only want HDR to reduce banding."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub clamp_hdr_extended_range: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct EncoderConfig {
    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "Quality preset",
        help = "Controls overall quality preset of the encoder. Works only on Windows AMD AMF, Linux VAAPI (AMD/Intel)."
    ))]
    pub quality_preset: EncoderQualityPreset,

    #[schema(
        strings(
            display_name = "Enable VBAQ/CAQ",
            help = "Enables Variance Based Adaptive Quantization on h264 and HEVC, and Content Adaptive Quantization on AV1"
        ),
        flag = "steamvr-restart"
    )]
    pub enable_vbaq: bool,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(help = r#"CBR: Constant BitRate mode. This is recommended.
VBR: Variable BitRate mode. Not commended because it may throw off the adaptive bitrate algorithm. This is only supported on Windows and only with AMD/Nvidia GPUs"#))]
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: RateControlMode,

    #[schema(strings(
        display_name = "h264: Profile",
        help = "Whenever possible, attempts to use this profile. May increase compatibility with varying mobile devices. Only has an effect for h264. Doesn't affect NVENC on Windows."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub h264_profile: H264Profile,

    #[schema(strings(help = r#"CAVLC algorithm is recommended.
CABAC produces better compression but it's significantly slower and may lead to runaway latency"#))]
    #[schema(flag = "steamvr-restart")]
    pub entropy_coding: EntropyCoding,

    #[schema(strings(
        help = r#"In CBR mode, this makes sure the bitrate does not fall below the assigned value. This is mostly useful for debugging."#
    ))]
    #[schema(flag = "steamvr-restart")]
    pub filler_data: bool,

    #[schema(strings(
        display_name = "10-bit encoding",
        help = "Sets the encoder to use 10 bits per channel instead of 8, if the client has no preference. Does not work on Linux with Nvidia"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub use_10bit: Option<bool>,

    #[schema(strings(
        display_name = "Encoding Gamma",
        help = "To prioritize darker pixels at the expense of potentially additional banding in midtones, set to 2.2. To allow the encoder to decide priority on its own, set to 1.0."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub encoding_gamma: Option<f32>,

    #[schema(strings(display_name = "HDR"))]
    #[schema(flag = "steamvr-restart")]
    pub hdr: HDRConfig,

    #[schema(strings(display_name = "NVENC"))]
    #[schema(flag = "steamvr-restart")]
    pub nvenc: NvencConfig,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(display_name = "AMF"))]
    #[schema(flag = "steamvr-restart")]
    pub amf: AmfConfig,

    #[schema(strings(display_name = "Software (CPU) encoding"))]
    pub software: SoftwareEncodingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MediacodecPropType {
    Float,
    Int32,
    Int64,
    String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediacodecProperty {
    #[schema(strings(display_name = "Type"))]
    pub ty: MediacodecPropType,
    pub value: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct EncoderLatencyLimiter {
    #[schema(strings(
        help = "Allowed percentage of frame interval to allocate for video encoding"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.3, max = 1.0, step = 0.01)))]
    pub max_saturation_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct DecoderLatencyLimiter {
    #[schema(strings(
        display_name = "Maximum decoder latency",
        help = "When the decoder latency goes above this threshold, the bitrate will be reduced"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
    pub max_decoder_latency_ms: u64,

    #[schema(strings(
        display_name = "latency overstep",
        help = "Number of consecutive frames above the threshold to trigger a bitrate reduction"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 100)), suffix = " frames")]
    pub latency_overstep_frames: usize,

    #[schema(strings(
        help = "Controls how much the bitrate is reduced when the decoder latency goes above the threshold"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.5, max = 1.0)))]
    pub latency_overstep_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum BitrateMode {
    #[schema(strings(display_name = "Constant"))]
    ConstantMbps(#[schema(gui(slider(min = 5, max = 1000, logarithmic)), suffix = "Mbps")] u64),

    #[schema(collapsible)]
    Adaptive {
        #[schema(strings(
            help = "Percentage of network bandwidth to allocate for video transmission"
        ))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 0.5, max = 5.0, step = 0.01)))]
        saturation_multiplier: f32,

        #[schema(strings(display_name = "Maximum bitrate"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 1000, logarithmic)), suffix = "Mbps")]
        max_throughput_mbps: Switch<u64>,

        #[schema(strings(display_name = "Minimum bitrate"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 100, logarithmic)), suffix = "Mbps")]
        min_throughput_mbps: Switch<u64>,

        #[schema(strings(display_name = "Maximum network latency"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
        max_network_latency_ms: Switch<u64>,

        #[schema(flag = "real-time")]
        encoder_latency_limiter: Switch<EncoderLatencyLimiter>,

        #[schema(strings(
            help = "Currently there is a bug where the decoder latency keeps rising when above a certain bitrate"
        ))]
        #[schema(flag = "real-time")]
        decoder_latency_limiter: Switch<DecoderLatencyLimiter>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BitrateAdaptiveFramerateConfig {
    #[schema(strings(
        display_name = "FPS reset threshold multiplier",
        help = "If the framerate changes more than this factor, trigger a parameters update",
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1.0, max = 3.0, step = 0.1)))]
    pub framerate_reset_threshold_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct BitrateConfig {
    #[schema(flag = "real-time")]
    pub mode: BitrateMode,

    #[schema(strings(
        help = "Ensure that the specified bitrate value is respected regardless of the framerate"
    ))]
    #[schema(flag = "real-time")]
    pub adapt_to_framerate: Switch<BitrateAdaptiveFramerateConfig>,

    #[schema(strings(help = "Controls the smoothness during calculations"))]
    pub history_size: usize,

    #[schema(strings(
        help = "When this is enabled, an IDR frame is requested after the bitrate is changed.
This has an effect only on AMD GPUs."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub image_corruption_fix: bool,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum ClientsideFoveationLevel {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClientsideFoveationMode {
    Static { level: ClientsideFoveationLevel },
    Dynamic { max_level: ClientsideFoveationLevel },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientsideFoveationConfig {
    pub mode: ClientsideFoveationMode,

    #[schema(strings(display_name = "Foveation offset"))]
    #[schema(gui(slider(min = -45.0, max = 45.0, step = 0.1)), suffix = "°")]
    pub vertical_offset_deg: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct FoveatedEncodingConfig {
    #[schema(strings(help = "Force enable on smartphone clients"))]
    pub force_enable: bool,

    #[schema(strings(display_name = "Center region width"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size_x: f32,

    #[schema(strings(display_name = "Center region height"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size_y: f32,

    #[schema(strings(display_name = "Center shift X"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift_x: f32,

    #[schema(strings(display_name = "Center shift Y"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift_y: f32,

    #[schema(strings(display_name = "Horizontal edge ratio"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio_x: f32,

    #[schema(strings(display_name = "Vertical edge ratio"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio_y: f32,
}

#[repr(C)]
#[derive(SettingsSchema, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct ColorCorrectionConfig {
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.001)))]
    #[schema(flag = "steamvr-restart")]
    pub brightness: f32,

    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.001)))]
    #[schema(flag = "steamvr-restart")]
    pub contrast: f32,

    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub saturation: f32,

    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub gamma: f32,

    #[schema(gui(slider(min = -1.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub sharpening: f32,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[schema(gui = "button_group")]
pub enum CodecType {
    #[default]
    #[schema(strings(display_name = "h264"))]
    H264 = 0,
    #[schema(strings(display_name = "HEVC"))]
    Hevc = 1,
    #[schema(strings(display_name = "AV1"))]
    AV1 = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum H264Profile {
    #[schema(strings(display_name = "High"))]
    High = 0,
    #[schema(strings(display_name = "Main"))]
    Main = 1,
    #[schema(strings(display_name = "Baseline"))]
    Baseline = 2,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RgbChromaKeyConfig {
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub red: u8,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub green: u8,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub blue: u8,

    #[schema(strings(help = "The threshold is applied per-channel"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 255)))]
    pub distance_threshold: u8,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.01, max = 1.0, step = 0.01)))]
    pub feathering: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HsvChromaKeyConfig {
    #[schema(strings(display_name = "Hue start max"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_start_max_deg: f32,

    #[schema(strings(display_name = "Hue start min"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_start_min_deg: f32,

    #[schema(strings(display_name = "Hue end min"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_end_min_deg: f32,

    #[schema(strings(display_name = "Hue end max"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_end_max_deg: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_start_max: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_start_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_end_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5,step = 0.01)))]
    pub saturation_end_max: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_start_max: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_start_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_end_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_end_max: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[schema(gui = "button_group")]
pub enum PassthroughMode {
    Blend {
        #[schema(strings(
            help = "Enabling this will adapt transparency based on the brightness of each pixel.
This is a similar effect to AR glasses."
        ))]
        #[schema(flag = "real-time")]
        premultiplied_alpha: bool,

        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
        threshold: f32,
    },

    #[schema(strings(display_name = "RGB Chroma Key"))]
    RgbChromaKey(#[schema(flag = "real-time")] RgbChromaKeyConfig),

    #[schema(strings(display_name = "HSV Chroma Key"))]
    HsvChromaKey(#[schema(flag = "real-time")] HsvChromaKeyConfig),
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum ClientsidePostProcessingSuperSamplingMode {
    Disabled = 0,
    Normal = 1 << 0,
    Quality = 1 << 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum ClientsidePostProcessingSharpeningMode {
    Disabled = 0,
    Normal = 1 << 2,
    Quality = 1 << 3,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ClientsidePostProcessingConfig {
    #[schema(strings(
        help = "Reduce flicker for high contrast edges.\nUseful when the input resolution is high compared to the headset display"
    ))]
    pub super_sampling: ClientsidePostProcessingSuperSamplingMode,
    #[schema(strings(
        help = "Improve clarity of high contrast edges and counteract blur.\nUseful when the input resolution is low compared to the headset display"
    ))]
    pub sharpening: ClientsidePostProcessingSharpeningMode,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UpscalingConfig {
    #[schema(strings(
        help = "Improves visual quality by using the edge direction to upscale at a slight performance loss"
    ))]
    pub edge_direction: bool,
    #[schema(gui(slider(min = 1.0, max = 16.0, step = 1.0)))]
    pub edge_threshold: f32,
    #[schema(gui(slider(min = 1.0, max = 2.0, step = 0.01)))]
    pub edge_sharpness: f32,
    #[schema(gui(slider(min = 1.0, max = 3.0, step = 0.01)))]
    #[schema(strings(
        help = "Dimensional resolution multiplier, high values will cause performance issues with weaker headset hardware or higher resolutions"
    ))]
    pub upscale_factor: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct VideoConfig {
    #[schema(flag = "real-time")]
    pub passthrough: Switch<PassthroughMode>,

    pub bitrate: BitrateConfig,

    #[schema(strings(
        help = "HEVC may provide better visual fidelity at the cost of increased encoder latency"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub preferred_codec: CodecType,

    #[schema(strings(
        notice = r"Disabling foveated encoding may result in significantly higher encode/decode times and stuttering, or even crashing.
If you want to reduce the amount of pixelation on the edges, increase the center region width and height"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub foveated_encoding: Switch<FoveatedEncodingConfig>,

    #[schema(flag = "steamvr-restart")]
    pub color_correction: Switch<ColorCorrectionConfig>,

    #[schema(
        strings(
            display_name = "Maximum buffering",
            help = "Increasing this value will help reduce stutter but it will increase latency"
        ),
        gui(slider(min = 1.0, max = 10.0, step = 0.1, logarithmic)),
        suffix = " frames"
    )]
    pub max_buffering_frames: f32,

    #[schema(gui(slider(min = 0.50, max = 0.99, step = 0.01)))]
    pub buffering_history_weight: f32,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(
        help = r"This works only on Windows. It shouldn't be disabled except in certain circumstances when you know the VR game will not meet the target framerate."
    ))]
    #[schema(flag = "real-time")]
    pub enforce_server_frame_pacing: bool,

    #[schema(flag = "steamvr-restart")]
    pub encoder_config: EncoderConfig,

    #[schema(strings(
        help = "Attempts to use a software decoder on the device. Slow, but may work around broken codecs."
    ))]
    pub force_software_decoder: bool,

    pub mediacodec_extra_options: Vec<(String, MediacodecProperty)>,

    #[schema(strings(
        help = "Resolution used for encoding and decoding. Relative to a single eye view."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub transcoding_view_resolution: FrameSize,

    #[schema(strings(
        help = "This is the resolution that SteamVR will use as default for the game rendering. Relative to a single eye view."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub emulated_headset_view_resolution: FrameSize,

    #[schema(strings(display_name = "Preferred FPS"))]
    #[schema(gui(slider(min = 60.0, max = 120.0)), suffix = "Hz")]
    #[schema(flag = "steamvr-restart")]
    pub preferred_fps: f32,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(
        help = "You probably don't want to change this. Allows for changing adapter for ALVR compositor."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adapter_index: u32,

    pub clientside_foveation: Switch<ClientsideFoveationConfig>,

    #[schema(strings(
        display_name = "Client-side post-processing",
        help = "Hardware optimized algorithms, available on Quest and Pico headsets"
    ))]
    #[schema(flag = "real-time")]
    pub clientside_post_processing: Switch<ClientsidePostProcessingConfig>,

    #[schema(strings(help = "Snapdragon Game Super Resolution client-side upscaling"))]
    pub upscaling: Switch<UpscalingConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum CustomAudioDeviceConfig {
    #[schema(strings(display_name = "By name (substring)"))]
    NameSubstring(String),
    #[schema(strings(display_name = "By index"))]
    Index(usize),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct AudioBufferingConfig {
    #[schema(strings(display_name = "Average buffering"))]
    #[schema(gui(slider(min = 0, max = 200)), suffix = "ms")]
    pub average_buffering_ms: u64,

    #[schema(strings(display_name = "Batch size"))]
    #[schema(gui(slider(min = 1, max = 20)), suffix = "ms")]
    pub batch_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct GameAudioConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    pub device: Option<CustomAudioDeviceConfig>,

    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    #[schema(strings(display_name = "Mute desktop audio when streaming"))]
    pub mute_when_streaming: bool,

    pub buffering: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum MicrophoneDevicesConfig {
    Automatic,
    #[schema(strings(display_name = "Virtual Audio Cable"))]
    VAC,
    #[schema(strings(display_name = "VB Cable"))]
    VBCable,
    #[schema(strings(display_name = "VoiceMeeter"))]
    VoiceMeeter,
    #[schema(strings(display_name = "VoiceMeeter Aux"))]
    VoiceMeeterAux,
    #[schema(strings(display_name = "VoiceMeeter VAIO3"))]
    VoiceMeeterVaio3,
    Custom {
        #[schema(strings(help = "This device is used by ALVR to output microphone audio"))]
        sink: CustomAudioDeviceConfig,
        #[schema(strings(help = "This device is set in SteamVR as the default microphone"))]
        source: CustomAudioDeviceConfig,
    },
}

// Note: sample rate is a free parameter for microphone, because both server and client supports
// resampling. In contrary, for game audio, the server does not support resampling.
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct MicrophoneConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    pub devices: MicrophoneDevicesConfig,

    pub buffering: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    #[schema(strings(display_name = "Headset speaker"))]
    pub game_audio: Switch<GameAudioConfig>,

    #[cfg_attr(
        windows,
        schema(strings(
            display_name = "Headset microphone",
            notice = r"To be able to use the microphone on Windows, you need to install Virtual Audio Cable"
        ))
    )]
    #[cfg_attr(not(windows), schema(strings(display_name = "Headset microphone")))]
    pub microphone: Switch<MicrophoneConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum HeadsetEmulationMode {
    #[schema(strings(display_name = "Rift S"))]
    RiftS,
    #[schema(strings(display_name = "Quest 2"))]
    Quest2,
    #[schema(strings(display_name = "Quest Pro"))]
    QuestPro,
    Vive,
    Custom {
        serial_number: String,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum FaceTrackingSourcesConfig {
    PreferEyeTrackingOnly,
    PreferFullFaceTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum FaceTrackingSinkConfig {
    #[schema(strings(display_name = "VRChat Eye OSC"))]
    VrchatEyeOsc { port: u16 },
    #[schema(strings(display_name = "VRCFaceTracking"))]
    VrcFaceTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct FaceTrackingConfig {
    pub sources: FaceTrackingSourcesConfig,
    pub sink: FaceTrackingSinkConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct BodyTrackingMetaConfig {
    pub prefer_full_body: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum BodyTrackingBDConfig {
    #[schema(strings(display_name = "Body Tracking"))]
    BodyTracking {
        #[schema(strings(
            help = "Improves accuracy of the tracking at the cost of higher latency."
        ))]
        high_accuracy: bool,
        #[schema(strings(
            help = "If trackers have not been calibrated before, the calibration process will start after you connect to the streamer."
        ))]
        prompt_calibration_on_start: bool,
    },

    #[schema(strings(display_name = "Object Tracking"))]
    ObjectTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BodyTrackingSourcesConfig {
    pub meta: BodyTrackingMetaConfig,
    pub bd: BodyTrackingBDConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum BodyTrackingSinkConfig {
    #[schema(strings(display_name = "Fake Vive Trackers"))]
    FakeViveTracker,
    #[schema(strings(display_name = "VRChat Body OSC"))]
    VrchatBodyOsc { port: u16 },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct BodyTrackingConfig {
    pub sources: BodyTrackingSourcesConfig,
    pub sink: BodyTrackingSinkConfig,
    #[schema(strings(help = "Turn this off to temporarily pause tracking."))]
    #[schema(flag = "real-time")]
    pub tracked: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct VMCConfig {
    pub host: String,
    pub port: u16,
    #[schema(strings(help = "Turn this off to temporarily pause sending data."))]
    #[schema(flag = "real-time")]
    pub publish: bool,
    #[schema(flag = "real-time")]
    pub orientation_correction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ControllersEmulationMode {
    #[schema(strings(display_name = "Rift S Touch"))]
    RiftSTouch,
    #[schema(strings(display_name = "Quest 2 Touch"))]
    Quest2Touch,
    #[schema(strings(display_name = "Quest 3 Touch Plus"))]
    Quest3Plus,
    #[schema(strings(display_name = "Quest Pro"))]
    QuestPro,
    #[schema(strings(display_name = "Pico 4"))]
    Pico4,
    #[schema(strings(display_name = "Valve Index"))]
    ValveIndex,
    #[schema(strings(display_name = "Vive Wand"))]
    ViveWand,
    #[schema(strings(display_name = "Vive Tracker"))]
    ViveTracker,
    Custom {
        serial_number: String,
        button_set: Vec<String>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct HysteresisThreshold {
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub value: f32,
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub deviation: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct BinaryToScalarStates {
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub off: f32,
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub on: f32,
}

// Remaps 0..1 to custom range
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct Range {
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub min: f32,
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub max: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum ButtonMappingType {
    Passthrough,
    HysteresisThreshold(HysteresisThreshold),
    BinaryToScalar(BinaryToScalarStates),
    Remap(Range),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ButtonBindingTarget {
    pub destination: String,
    pub mapping_type: ButtonMappingType,
    pub binary_conditions: Vec<String>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct AutomaticButtonMappingConfig {
    pub click_threshold: HysteresisThreshold,
    pub touch_threshold: HysteresisThreshold,
    pub force_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HandTrackingInteractionConfig {
    #[schema(flag = "real-time")]
    pub only_touch: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How close the tips of your fingers need to be to register a pinch click."
    ))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "cm")]
    pub pinch_touch_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How close together the tips of your fingers need to be to start registering a pinch trigger pull."
    ))]
    #[schema(gui(slider(min = 0.0, max = 2.5, step = 0.025)), suffix = "cm")]
    pub pinch_trigger_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How close to your palm the tips of your fingers need to be to register a curl click."
    ))]
    #[schema(gui(slider(min = 0.0, max = 5.0)), suffix = "cm")]
    pub curl_touch_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How close to your palm the tips of your fingers need to be to start registering a trigger pull."
    ))]
    #[schema(gui(slider(min = 0.0, max = 10.0)), suffix = "cm")]
    pub curl_trigger_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 100.0)), suffix = "%")]
    pub joystick_deadzone: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -5.0, max = 5.0)), suffix = "cm")]
    pub joystick_offset_horizontal: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -5.0, max = 5.0)), suffix = "cm")]
    pub joystick_offset_vertical: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "The radius of motion of the joystick. The joystick can be controlled if the thumb is within 2x this range."
    ))]
    #[schema(gui(slider(min = 0.0, max = 5.0)), suffix = "cm")]
    pub joystick_range: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How long the gesture must be continuously held before it is activated."
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub activation_delay: u32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How long the gesture must be continuously released before it is deactivated."
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub deactivation_delay: u32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "How long the after the gesture has been deactivated before it can be activated again."
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub repeat_delay: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct HapticsConfig {
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.1)))]
    pub intensity_multiplier: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub amplitude_curve: f32,

    #[schema(strings(display_name = "Minimum duration"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 0.1, step = 0.001)), suffix = "s")]
    pub min_duration_s: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HandSkeletonConfig {
    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        help = r"Enabling this will use separate tracker objects with the full skeletal tracking level when hand tracking is detected. This is required for VRChat hand tracking."
    ))]
    pub steamvr_input_2_0: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = r"Predict hand skeleton to make it less floaty. It may make hands too jittery."
    ))]
    pub predict: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct ControllersConfig {
    #[schema(strings(help = "Turning this off will make the controllers appear powered off."))]
    #[schema(flag = "real-time")]
    pub tracked: bool,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        help = "Enabling this passes skeletal hand data (finger tracking) to SteamVR."
    ))]
    pub hand_skeleton: Switch<HandSkeletonConfig>,

    #[schema(strings(
        help = r"Track hand skeleton while holding controllers. This will reduce hand tracking frequency to 30Hz.
Because of runtime limitations, this option is ignored when body tracking is active."
    ))]
    pub multimodal_tracking: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "Enabling this allows using hand gestures to emulate controller inputs."
    ))]
    pub hand_tracking_interaction: Switch<HandTrackingInteractionConfig>,

    #[schema(strings(
        display_name = "Prediction",
        help = r"Higher values make the controllers track smoother.
Technically, this is the time (counted in frames) between pose submitted to SteamVR and the corresponding virtual vsync happens.
Currently this cannot be reliably estimated automatically. The correct value should be 2 but 3 is default for smoother tracking at the cost of slight lag."
    ))]
    #[schema(gui(slider(min = 1.0, max = 10.0, logarithmic)), suffix = "frames")]
    pub steamvr_pipeline_frames: f32,

    #[schema(flag = "real-time")]
    pub haptics: Switch<HapticsConfig>,

    #[schema(flag = "steamvr-restart")]
    pub emulation_mode: ControllersEmulationMode,

    #[schema(flag = "steamvr-restart")]
    pub extra_openvr_props: Vec<OpenvrProperty>,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "m/s")]
    pub linear_velocity_cutoff: f32,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = 0.0, max = 100.0, step = 1.0)), suffix = "°/s")]
    pub angular_velocity_cutoff: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(help = "Right controller offset is mirrored horizontally"))]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_controller_position_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(help = "Right controller offset is mirrored horizontally"))]
    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_controller_rotation_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(help = "Right controller offset is mirrored horizontally"))]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_hand_tracking_position_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(help = "Right controller offset is mirrored horizontally"))]
    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_hand_tracking_rotation_offset: [f32; 3],

    #[schema(strings(help = "List of OpenXR-syle paths"))]
    pub button_mappings: Option<Vec<(String, Vec<ButtonBindingTarget>)>>,

    pub button_mapping_config: AutomaticButtonMappingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum PositionRecenteringMode {
    Disabled,
    LocalFloor,
    Local {
        #[schema(gui(slider(min = 0.0, max = 3.0)), suffix = "m")]
        view_height: f32,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum RotationRecenteringMode {
    Disabled,
    Yaw,
    Tilted,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HeadsetConfig {
    #[schema(strings(
        help = r#"Disabled: the playspace origin is determined by the room-scale guardian setup.
Local floor: the origin is on the floor and resets when long pressing the oculus button.
Local: the origin resets when long pressing the oculus button, and is calculated as an offset from the current head position."#
    ))]
    #[schema(flag = "real-time")]
    pub position_recentering_mode: PositionRecenteringMode,

    #[schema(strings(
        help = r#"Disabled: the playspace orientation is determined by the room-scale guardian setup.
Yaw: the forward direction is reset when long pressing the oculus button.
Tilted: the world gets tilted when long pressing the oculus button. This is useful for using VR while laying down."#
    ))]
    #[schema(flag = "real-time")]
    pub rotation_recentering_mode: RotationRecenteringMode,

    #[schema(flag = "steamvr-restart")]
    pub controllers: Switch<ControllersConfig>,

    #[schema(flag = "steamvr-restart")]
    pub emulation_mode: HeadsetEmulationMode,

    #[schema(flag = "steamvr-restart")]
    pub extra_openvr_props: Vec<OpenvrProperty>,

    #[schema(flag = "steamvr-restart")]
    pub tracking_ref_only: bool,

    #[schema(flag = "steamvr-restart")]
    pub enable_vive_tracker_proxy: bool,

    pub face_tracking: Switch<FaceTrackingConfig>,

    #[schema(flag = "steamvr-restart")]
    pub body_tracking: Switch<BodyTrackingConfig>,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(display_name = "VMC"))]
    pub vmc: Switch<VMCConfig>,

    #[schema(strings(
        help = "Maximum prediction for head and controllers. Used to avoid too much jitter during loading."
    ))]
    #[schema(gui(slider(min = 0, max = 200, step = 5)), suffix = "ms")]
    pub max_prediction_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[schema(gui = "button_group")]
pub enum SocketProtocol {
    #[schema(strings(display_name = "UDP"))]
    Udp,
    #[schema(strings(display_name = "TCP"))]
    Tcp,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct DiscoveryConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    #[schema(strings(
        help = "Allow untrusted clients to connect without confirmation. This is not recommended for security reasons."
    ))]
    pub auto_trust_clients: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum SocketBufferSize {
    Default,
    Maximum,
    Custom(#[schema(suffix = "B")] u32),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    #[schema(strings(
        help = r#"UDP: Faster, but less stable than TCP. Try this if your network is well optimized and free of interference.
TCP: Slower than UDP, but more stable. Pick this if you experience video or audio stutters with UDP."#
    ))]
    pub stream_protocol: SocketProtocol,

    pub client_discovery: Switch<DiscoveryConfig>,

    #[schema(strings(
        help = r#"Which release type of client should ALVR look for when establishing a wired connection."#
    ))]
    pub wired_client_type: ClientFlavor,

    #[schema(strings(
        help = r#"Wether ALVR should try to automatically launch the client when establishing a wired connection."#
    ))]
    pub wired_client_autolaunch: bool,

    #[cfg_attr(
        windows,
        schema(strings(
            help = "If on_connect.bat exists alongside session.json, it will be run on headset connect. Env var ACTION will be set to `connect`."
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            help = "If on_connect.sh exists alongside session.json, it will be run on headset connect. Env var ACTION will be set to `connect`."
        ))
    )]
    pub enable_on_connect_script: bool,

    #[cfg_attr(
        windows,
        schema(strings(
            help = "If on_disconnect.bat exists alongside session.json, it will be run on headset disconnect. Env var ACTION will be set to `disconnect`."
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            help = "If on_disconnect.sh exists alongside session.json, it will be run on headset disconnect. Env var ACTION will be set to `disconnect`."
        ))
    )]
    #[schema(flag = "real-time")]
    pub enable_on_disconnect_script: bool,

    #[schema(strings(
        help = "Allow cross-origin browser requests to control ALVR settings remotely."
    ))]
    #[schema(flag = "real-time")]
    pub allow_untrusted_http: bool,

    #[schema(strings(
        help = r#"If the client, server or the network discarded one packet, discard packets until a IDR packet is found."#
    ))]
    pub avoid_video_glitching: bool,

    #[schema(gui(slider(min = 1024, max = 65507, logarithmic)), suffix = "B")]
    pub packet_size: i32,

    pub stream_port: u16,
    pub web_server_port: u16,
    pub osc_local_port: u16,

    #[schema(strings(display_name = "Streamer send buffer size"))]
    pub server_send_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "Streamer receive buffer size"))]
    pub server_recv_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "Client send buffer size"))]
    pub client_send_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "Client receive buffer size"))]
    pub client_recv_buffer_bytes: SocketBufferSize,

    #[schema(strings(
        help = r#"The server discards video packets if it can't push them to the network.
This could happen on TCP. A IDR frame is requested in this case."#
    ))]
    pub max_queued_server_video_frames: usize,

    #[schema(suffix = " frames")]
    pub statistics_history_size: usize,

    #[schema(strings(display_name = "Minimum IDR interval"))]
    #[schema(flag = "steamvr-restart")]
    #[schema(gui(slider(min = 5, max = 1000, step = 5)), suffix = "ms")]
    pub minimum_idr_interval_ms: u64,

    pub dscp: Option<DscpTos>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
#[schema(gui = "button_group")]
pub enum DropProbability {
    Low = 0x01,
    Medium = 0x10,
    High = 0x11,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum DscpTos {
    BestEffort,

    ClassSelector(#[schema(gui(slider(min = 1, max = 7)))] u8),

    AssuredForwarding {
        #[schema(gui(slider(min = 1, max = 4)))]
        class: u8,
        drop_probability: DropProbability,
    },

    ExpeditedForwarding,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct RawEventsConfig {
    #[schema(flag = "real-time")]
    pub hide_spammy_events: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    #[schema(strings(help = "Notification tips teach you how to use ALVR"))]
    pub show_notification_tip: bool,

    #[schema(strings(help = "This applies only to certain error or warning messages."))]
    #[schema(flag = "steamvr-restart")]
    pub prefer_backtrace: bool,

    #[schema(flag = "real-time")]
    pub notification_level: LogSeverity,

    pub client_log_report_level: Switch<LogSeverity>,

    #[schema(flag = "real-time")]
    pub show_raw_events: Switch<RawEventsConfig>,

    #[schema(strings(help = "Write logs into the session_log.txt file."))]
    pub log_to_disk: bool,

    #[schema(flag = "real-time")]
    pub log_tracking: bool,

    #[schema(flag = "real-time")]
    pub log_button_presses: bool,

    #[schema(flag = "real-time")]
    pub log_haptics: bool,

    #[cfg_attr(not(debug_assertions), schema(flag = "hidden"))]
    #[schema(strings(help = "These settings enable extra spammy logs for debugging purposes."))]
    pub debug_groups: DebugGroupsConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct SteamvrLauncher {
    #[schema(strings(display_name = "Open and close SteamVR with dashboard"))]
    pub open_close_steamvr_with_dashboard: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct RollingVideoFilesConfig {
    #[schema(strings(display_name = "Duration"))]
    #[schema(suffix = "s")]
    pub duration_s: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct CaptureConfig {
    #[schema(strings(display_name = "Start video recording at client connection"))]
    pub startup_video_recording: bool,

    pub rolling_video_files: Switch<RollingVideoFilesConfig>,

    #[schema(flag = "steamvr-restart")]
    pub capture_frame_dir: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Patches {
    #[schema(strings(
        help = "Async Compute is currently broken in SteamVR, keep disabled. ONLY FOR TESTING."
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_compute: bool,
    #[schema(strings(
        help = "Async reprojection only works if you can always hit at least half of your refresh rate.",
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_reprojection: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct NewVersionPopupConfig {
    pub hide_while_version: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ExtraConfig {
    pub steamvr_launcher: SteamvrLauncher,
    pub capture: CaptureConfig,
    pub logging: LoggingConfig,
    #[cfg_attr(not(target_os = "linux"), schema(flag = "hidden"))]
    pub patches: Patches,

    #[schema(
        strings(help = "Linear and angular velocity multiplier for debug purposes.
It does not update in real time.")
    )]
    pub velocities_multiplier: f32,

    pub open_setup_wizard: bool,
    pub new_version_popup: Switch<NewVersionPopupConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub video: VideoConfig,
    pub audio: AudioConfig,
    pub headset: HeadsetConfig,
    pub connection: ConnectionConfig,
    pub extra: ExtraConfig,
}

pub fn session_settings_default() -> SettingsDefault {
    let view_resolution = FrameSizeDefault {
        variant: FrameSizeDefaultVariant::Absolute,
        Scale: 1.0,
        Absolute: FrameSizeAbsoluteDefault {
            width: 2144,
            height: OptionalDefault {
                set: false,
                content: 1072,
            },
        },
    };
    let default_custom_audio_device = CustomAudioDeviceConfigDefault {
        NameSubstring: "".into(),
        Index: 0,
        variant: CustomAudioDeviceConfigDefaultVariant::NameSubstring,
    };
    let default_custom_openvr_props = VectorDefault {
        gui_collapsed: true,
        element: OpenvrPropertyDefault {
            key: OpenvrPropKeyDefault {
                variant: OpenvrPropKeyDefaultVariant::TrackingSystemNameString,
            },
            value: "".into(),
        },
        content: vec![],
    };
    let socket_buffer = SocketBufferSizeDefault {
        Custom: 100000,
        variant: SocketBufferSizeDefaultVariant::Maximum,
    };

    SettingsDefault {
        video: VideoConfigDefault {
            passthrough: SwitchDefault {
                enabled: false,
                content: PassthroughModeDefault {
                    variant: PassthroughModeDefaultVariant::Blend,
                    Blend: PassthroughModeBlendDefault {
                        premultiplied_alpha: true,
                        threshold: 0.5,
                    },
                    RgbChromaKey: RgbChromaKeyConfigDefault {
                        red: 0,
                        green: 255,
                        blue: 0,
                        distance_threshold: 85,
                        feathering: 0.05,
                    },
                    HsvChromaKey: HsvChromaKeyConfigDefault {
                        hue_start_max_deg: 70.0,
                        hue_start_min_deg: 80.0,
                        hue_end_min_deg: 160.0,
                        hue_end_max_deg: 170.0,
                        saturation_start_max: 0.2,
                        saturation_start_min: 0.3,
                        saturation_end_min: 1.0,
                        saturation_end_max: 1.1,
                        value_start_max: 0.0,
                        value_start_min: 0.1,
                        value_end_min: 1.0,
                        value_end_max: 1.1,
                    },
                },
            },
            clientside_post_processing: SwitchDefault {
                enabled: false,
                content: ClientsidePostProcessingConfigDefault {
                    super_sampling: ClientsidePostProcessingSuperSamplingModeDefault {
                        variant: ClientsidePostProcessingSuperSamplingModeDefaultVariant::Quality,
                    },
                    sharpening: ClientsidePostProcessingSharpeningModeDefault {
                        variant: ClientsidePostProcessingSharpeningModeDefaultVariant::Quality,
                    },
                },
            },
            upscaling: SwitchDefault {
                enabled: false,
                content: UpscalingConfigDefault {
                    edge_direction: true,
                    edge_threshold: 4.0,
                    edge_sharpness: 2.0,
                    upscale_factor: 1.5,
                },
            },
            adapter_index: 0,
            transcoding_view_resolution: view_resolution.clone(),
            emulated_headset_view_resolution: view_resolution,
            preferred_fps: 72.,
            max_buffering_frames: 2.0,
            buffering_history_weight: 0.90,
            enforce_server_frame_pacing: true,
            bitrate: BitrateConfigDefault {
                gui_collapsed: false,
                mode: BitrateModeDefault {
                    ConstantMbps: 30,
                    Adaptive: BitrateModeAdaptiveDefault {
                        gui_collapsed: true,
                        saturation_multiplier: 0.95,
                        max_throughput_mbps: SwitchDefault {
                            enabled: false,
                            content: 100,
                        },
                        min_throughput_mbps: SwitchDefault {
                            enabled: false,
                            content: 5,
                        },
                        max_network_latency_ms: SwitchDefault {
                            enabled: false,
                            content: 8,
                        },
                        encoder_latency_limiter: SwitchDefault {
                            enabled: true,
                            content: EncoderLatencyLimiterDefault {
                                max_saturation_multiplier: 0.9,
                            },
                        },
                        decoder_latency_limiter: SwitchDefault {
                            enabled: true,
                            content: DecoderLatencyLimiterDefault {
                                gui_collapsed: false,
                                max_decoder_latency_ms: 30,
                                latency_overstep_frames: 90,
                                latency_overstep_multiplier: 0.99,
                            },
                        },
                    },
                    variant: BitrateModeDefaultVariant::ConstantMbps,
                },
                adapt_to_framerate: SwitchDefault {
                    enabled: false,
                    content: BitrateAdaptiveFramerateConfigDefault {
                        framerate_reset_threshold_multiplier: 2.0,
                    },
                },
                history_size: 256,
                image_corruption_fix: false,
            },
            preferred_codec: CodecTypeDefault {
                variant: CodecTypeDefaultVariant::H264,
            },
            encoder_config: EncoderConfigDefault {
                gui_collapsed: true,
                rate_control_mode: RateControlModeDefault {
                    variant: RateControlModeDefaultVariant::Cbr,
                },
                filler_data: false,
                h264_profile: H264ProfileDefault {
                    variant: H264ProfileDefaultVariant::High,
                },
                entropy_coding: EntropyCodingDefault {
                    variant: EntropyCodingDefaultVariant::Cavlc,
                },
                use_10bit: OptionalDefault {
                    set: false,
                    content: false,
                },
                encoding_gamma: OptionalDefault {
                    set: false,
                    content: 1.0,
                },
                hdr: HDRConfigDefault {
                    gui_collapsed: true,
                    enable: OptionalDefault {
                        set: false,
                        content: false,
                    },
                    force_hdr_srgb_correction: false,
                    clamp_hdr_extended_range: false,
                },
                nvenc: NvencConfigDefault {
                    gui_collapsed: true,
                    quality_preset: EncoderQualityPresetNvidiaDefault {
                        variant: EncoderQualityPresetNvidiaDefaultVariant::P1,
                    },
                    tuning_preset: NvencTuningPresetDefault {
                        variant: NvencTuningPresetDefaultVariant::LowLatency,
                    },
                    multi_pass: NvencMultiPassDefault {
                        variant: NvencMultiPassDefaultVariant::QuarterResolution,
                    },
                    adaptive_quantization_mode: NvencAdaptiveQuantizationModeDefault {
                        variant: NvencAdaptiveQuantizationModeDefaultVariant::Spatial,
                    },
                    low_delay_key_frame_scale: -1,
                    refresh_rate: -1,
                    enable_intra_refresh: false,
                    intra_refresh_period: -1,
                    intra_refresh_count: -1,
                    max_num_ref_frames: -1,
                    gop_length: -1,
                    p_frame_strategy: -1,
                    rate_control_mode: -1,
                    rc_buffer_size: -1,
                    rc_initial_delay: -1,
                    rc_max_bitrate: -1,
                    rc_average_bitrate: -1,
                    enable_weighted_prediction: false,
                },
                quality_preset: EncoderQualityPresetDefault {
                    variant: EncoderQualityPresetDefaultVariant::Speed,
                },
                enable_vbaq: false,
                amf: AmfConfigDefault {
                    gui_collapsed: true,
                    enable_pre_analysis: false,
                    enable_hmqb: false,
                    use_preproc: false,
                    preproc_sigma: 4,
                    preproc_tor: 7,
                },
                software: SoftwareEncodingConfigDefault {
                    gui_collapsed: true,
                    force_software_encoding: false,
                    thread_count: 0,
                },
            },
            mediacodec_extra_options: {
                fn int32_default(int32: i32) -> MediacodecPropertyDefault {
                    MediacodecPropertyDefault {
                        ty: MediacodecPropTypeDefault {
                            variant: MediacodecPropTypeDefaultVariant::Int32,
                        },
                        value: int32.to_string(),
                    }
                }
                DictionaryDefault {
                    gui_collapsed: true,
                    key: "".into(),
                    value: int32_default(0),
                    content: vec![
                        ("operating-rate".into(), int32_default(i32::MAX)),
                        ("priority".into(), int32_default(0)),
                        // low-latency: only applicable on API level 30. Quest 1 and 2 might not be
                        // cabable, since they are on level 29.
                        // ("low-latency".into(), int32_default(1)), // Android smartphones crashes enabling this feature (https://github.com/PhoneVR-Developers/alvr-cardboard/issues/5)
                        (
                            "vendor.qti-ext-dec-low-latency.enable".into(),
                            int32_default(1),
                        ),
                    ],
                }
            },
            foveated_encoding: SwitchDefault {
                enabled: true,
                content: FoveatedEncodingConfigDefault {
                    gui_collapsed: true,
                    force_enable: false,
                    center_size_x: 0.45,
                    center_size_y: 0.4,
                    center_shift_x: 0.4,
                    center_shift_y: 0.1,
                    edge_ratio_x: 4.,
                    edge_ratio_y: 5.,
                },
            },
            clientside_foveation: SwitchDefault {
                enabled: false,
                content: ClientsideFoveationConfigDefault {
                    mode: ClientsideFoveationModeDefault {
                        Static: ClientsideFoveationModeStaticDefault {
                            level: ClientsideFoveationLevelDefault {
                                variant: ClientsideFoveationLevelDefaultVariant::High,
                            },
                        },
                        Dynamic: ClientsideFoveationModeDynamicDefault {
                            max_level: ClientsideFoveationLevelDefault {
                                variant: ClientsideFoveationLevelDefaultVariant::High,
                            },
                        },
                        variant: ClientsideFoveationModeDefaultVariant::Dynamic,
                    },
                    vertical_offset_deg: 0.0,
                },
            },
            force_software_decoder: false,
            color_correction: SwitchDefault {
                enabled: false,
                content: ColorCorrectionConfigDefault {
                    brightness: 0.,
                    contrast: 0.,
                    saturation: 0.5,
                    gamma: 1.,
                    sharpening: 0.5,
                },
            },
        },
        audio: AudioConfigDefault {
            game_audio: SwitchDefault {
                enabled: true,
                content: GameAudioConfigDefault {
                    gui_collapsed: true,
                    device: OptionalDefault {
                        set: false,
                        content: default_custom_audio_device.clone(),
                    },
                    mute_when_streaming: true,
                    buffering: AudioBufferingConfigDefault {
                        gui_collapsed: true,
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
            microphone: SwitchDefault {
                enabled: cfg!(target_os = "linux"),
                content: MicrophoneConfigDefault {
                    gui_collapsed: true,
                    devices: MicrophoneDevicesConfigDefault {
                        Custom: MicrophoneDevicesConfigCustomDefault {
                            source: default_custom_audio_device.clone(),
                            sink: default_custom_audio_device,
                        },
                        variant: MicrophoneDevicesConfigDefaultVariant::Automatic,
                    },
                    buffering: AudioBufferingConfigDefault {
                        gui_collapsed: true,
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
        },
        headset: HeadsetConfigDefault {
            emulation_mode: HeadsetEmulationModeDefault {
                Custom: HeadsetEmulationModeCustomDefault {
                    serial_number: "Unknown".into(),
                },
                variant: HeadsetEmulationModeDefaultVariant::Quest2,
            },
            extra_openvr_props: default_custom_openvr_props.clone(),
            tracking_ref_only: false,
            enable_vive_tracker_proxy: false,
            face_tracking: SwitchDefault {
                enabled: false,
                content: FaceTrackingConfigDefault {
                    gui_collapsed: true,
                    sources: FaceTrackingSourcesConfigDefault {
                        variant: FaceTrackingSourcesConfigDefaultVariant::PreferFullFaceTracking,
                    },
                    sink: FaceTrackingSinkConfigDefault {
                        VrchatEyeOsc: FaceTrackingSinkConfigVrchatEyeOscDefault { port: 9000 },
                        variant: FaceTrackingSinkConfigDefaultVariant::VrchatEyeOsc,
                    },
                },
            },
            body_tracking: SwitchDefault {
                enabled: false,
                content: BodyTrackingConfigDefault {
                    gui_collapsed: true,
                    sources: BodyTrackingSourcesConfigDefault {
                        meta: BodyTrackingMetaConfigDefault {
                            prefer_full_body: true,
                        },
                        bd: BodyTrackingBDConfigDefault {
                            BodyTracking: BodyTrackingBDConfigBodyTrackingDefault {
                                high_accuracy: true,
                                prompt_calibration_on_start: true,
                            },
                            variant: BodyTrackingBDConfigDefaultVariant::BodyTracking,
                        },
                    },
                    sink: BodyTrackingSinkConfigDefault {
                        VrchatBodyOsc: BodyTrackingSinkConfigVrchatBodyOscDefault { port: 9000 },
                        variant: BodyTrackingSinkConfigDefaultVariant::FakeViveTracker,
                    },
                    tracked: true,
                },
            },
            vmc: SwitchDefault {
                enabled: false,
                content: VMCConfigDefault {
                    gui_collapsed: true,
                    host: "127.0.0.1".into(),
                    port: 39539,
                    publish: true,
                    orientation_correction: true,
                },
            },
            controllers: SwitchDefault {
                enabled: true,
                content: ControllersConfigDefault {
                    gui_collapsed: false,
                    tracked: true,
                    hand_skeleton: SwitchDefault {
                        enabled: true,
                        content: HandSkeletonConfigDefault {
                            steamvr_input_2_0: true,
                            predict: false,
                        },
                    },
                    multimodal_tracking: false,
                    emulation_mode: ControllersEmulationModeDefault {
                        Custom: ControllersEmulationModeCustomDefault {
                            serial_number: "ALVR Controller".into(),
                            button_set: VectorDefault {
                                gui_collapsed: false,
                                element: "/user/hand/left/input/a/click".into(),
                                content: vec![],
                            },
                        },
                        variant: ControllersEmulationModeDefaultVariant::Quest2Touch,
                    },
                    extra_openvr_props: default_custom_openvr_props,
                    button_mappings: OptionalDefault {
                        set: false,
                        content: DictionaryDefault {
                            gui_collapsed: false,
                            key: "/user/hand/left/input/a/click".into(),
                            value: VectorDefault {
                                gui_collapsed: false,
                                element: ButtonBindingTargetDefault {
                                    destination: "/user/hand/left/input/a/click".into(),
                                    mapping_type: ButtonMappingTypeDefault {
                                        HysteresisThreshold: HysteresisThresholdDefault {
                                            value: 0.5,
                                            deviation: 0.05,
                                        },
                                        BinaryToScalar: BinaryToScalarStatesDefault {
                                            off: 0.0,
                                            on: 1.0,
                                        },
                                        Remap: RangeDefault { min: 0.0, max: 1.0 },
                                        variant: ButtonMappingTypeDefaultVariant::Passthrough,
                                    },
                                    binary_conditions: VectorDefault {
                                        gui_collapsed: true,
                                        element: "/user/hand/left/input/trigger/touch".into(),
                                        content: vec![],
                                    },
                                },
                                content: vec![],
                            },
                            content: vec![],
                        },
                    },
                    button_mapping_config: AutomaticButtonMappingConfigDefault {
                        gui_collapsed: true,
                        click_threshold: HysteresisThresholdDefault {
                            value: 0.5,
                            deviation: 0.05,
                        },
                        touch_threshold: HysteresisThresholdDefault {
                            value: 0.1,
                            deviation: 0.05,
                        },
                        force_threshold: 0.8,
                    },
                    hand_tracking_interaction: SwitchDefault {
                        enabled: false,
                        content: HandTrackingInteractionConfigDefault {
                            only_touch: false,
                            pinch_touch_distance: 0.0,
                            pinch_trigger_distance: 0.25,
                            curl_touch_distance: 2.0,
                            curl_trigger_distance: 2.5,
                            joystick_deadzone: 40.0,
                            joystick_offset_horizontal: 0.0,
                            joystick_offset_vertical: 0.0,
                            joystick_range: 1.0,
                            repeat_delay: 100,
                            activation_delay: 50,
                            deactivation_delay: 100,
                        },
                    },
                    steamvr_pipeline_frames: 2.1,
                    linear_velocity_cutoff: 0.05,
                    angular_velocity_cutoff: 10.0,
                    left_controller_position_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0, 0.0, -0.11],
                    },
                    left_controller_rotation_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0; 3],
                    },
                    left_hand_tracking_position_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.04, -0.02, -0.13],
                    },
                    left_hand_tracking_rotation_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0, -45.0, -90.0],
                    },
                    haptics: SwitchDefault {
                        enabled: true,
                        content: HapticsConfigDefault {
                            gui_collapsed: true,
                            intensity_multiplier: 1.0,
                            amplitude_curve: 1.0,
                            min_duration_s: 0.01,
                        },
                    },
                },
            },
            position_recentering_mode: PositionRecenteringModeDefault {
                Local: PositionRecenteringModeLocalDefault { view_height: 1.5 },
                variant: PositionRecenteringModeDefaultVariant::LocalFloor,
            },
            rotation_recentering_mode: RotationRecenteringModeDefault {
                variant: RotationRecenteringModeDefaultVariant::Yaw,
            },
            max_prediction_ms: 100,
        },
        connection: ConnectionConfigDefault {
            stream_protocol: SocketProtocolDefault {
                variant: SocketProtocolDefaultVariant::Udp,
            },
            client_discovery: SwitchDefault {
                enabled: true,
                content: DiscoveryConfigDefault {
                    auto_trust_clients: cfg!(debug_assertions),
                },
            },
            wired_client_type: ClientFlavorDefault {
                Custom: "alvr.client".to_owned(),
                variant: if alvr_common::is_stable() {
                    ClientFlavorDefaultVariant::Store
                } else {
                    ClientFlavorDefaultVariant::Github
                },
            },
            wired_client_autolaunch: true,
            web_server_port: 8082,
            stream_port: 9944,
            osc_local_port: 9942,
            dscp: OptionalDefault {
                set: false,
                content: DscpTosDefault {
                    ClassSelector: 7,
                    AssuredForwarding: DscpTosAssuredForwardingDefault {
                        class: 4,
                        drop_probability: DropProbabilityDefault {
                            variant: DropProbabilityDefaultVariant::Low,
                        },
                    },
                    variant: DscpTosDefaultVariant::ExpeditedForwarding,
                },
            },
            server_send_buffer_bytes: socket_buffer.clone(),
            server_recv_buffer_bytes: socket_buffer.clone(),
            client_send_buffer_bytes: socket_buffer.clone(),
            client_recv_buffer_bytes: socket_buffer,
            max_queued_server_video_frames: 1024,
            avoid_video_glitching: false,
            minimum_idr_interval_ms: 100,
            enable_on_connect_script: false,
            enable_on_disconnect_script: false,
            allow_untrusted_http: false,
            packet_size: 1400,
            statistics_history_size: 256,
        },
        extra: ExtraConfigDefault {
            logging: LoggingConfigDefault {
                client_log_report_level: SwitchDefault {
                    enabled: true,
                    content: LogSeverityDefault {
                        variant: LogSeverityDefaultVariant::Error,
                    },
                },
                log_to_disk: cfg!(debug_assertions),
                log_button_presses: false,
                log_tracking: false,
                log_haptics: false,
                notification_level: LogSeverityDefault {
                    variant: if cfg!(debug_assertions) {
                        LogSeverityDefaultVariant::Info
                    } else {
                        LogSeverityDefaultVariant::Warning
                    },
                },
                show_raw_events: SwitchDefault {
                    enabled: false,
                    content: RawEventsConfigDefault {
                        hide_spammy_events: false,
                    },
                },
                prefer_backtrace: false,
                show_notification_tip: true,
                debug_groups: DebugGroupsConfigDefault {
                    server_impl: false,
                    client_impl: false,
                    server_core: false,
                    client_core: false,
                    connection: false,
                    sockets: false,
                    server_gfx: false,
                    client_gfx: false,
                    encoder: false,
                    decoder: false,
                },
            },
            steamvr_launcher: SteamvrLauncherDefault {
                open_close_steamvr_with_dashboard: false,
            },
            capture: CaptureConfigDefault {
                startup_video_recording: false,
                rolling_video_files: SwitchDefault {
                    enabled: false,
                    content: RollingVideoFilesConfigDefault { duration_s: 5 },
                },
                capture_frame_dir: if !cfg!(target_os = "linux") {
                    "/tmp".into()
                } else {
                    "".into()
                },
            },
            patches: PatchesDefault {
                linux_async_compute: false,
                linux_async_reprojection: false,
            },
            velocities_multiplier: 1.0,
            open_setup_wizard: alvr_common::is_stable() || alvr_common::is_nightly(),
            new_version_popup: SwitchDefault {
                enabled: alvr_common::is_stable(),
                content: NewVersionPopupConfigDefault {
                    hide_while_version: ALVR_VERSION.to_string(),
                },
            },
        },
    }
}
