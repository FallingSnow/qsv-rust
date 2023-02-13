use crate::utils::MFX_MAKEFOURCC;

pub type mfxU8 = u8;
pub type mfxU16 = u16;
pub type mfxU32 = u32;
pub type mfxI32 = i32;
pub type mfxU64 = u64;
pub type mfxI64 = i64;
pub type mfxIMPL = mfxI32;
pub type mfxStatus = mfxI32;
pub type mfxSession = libc::c_void;
pub type mfxHDL = *const libc::c_void;
pub type mfxMemId = mfxHDL;
pub type mfxSyncPoint = *const libc::c_void;

pub const MFX_IMPL_AUTO: mfxIMPL = 0x0000;
pub const MFX_IMPL_SOFTWARE: mfxIMPL = 0x0001;
pub const MFX_IMPL_HARDWARE: mfxIMPL = 0x0002;
pub const MFX_IMPL_AUTO_ANY: mfxIMPL = 0x0003;
pub const MFX_IMPL_HARDWARE_ANY: mfxIMPL = 0x0004;

/// The function completed successfully.
pub const MFX_ERR_NONE: mfxStatus = 0;
/// An unknown error occurred in the library function operation. This is a reserved status code.
pub const MFX_ERR_UNKNOWN: mfxStatus = -1;
/// NULL pointer in the input or output arguments
pub const MFX_ERR_NULL_PTR: mfxStatus = -2;
/// Unsupported configurations, parameters, or features
pub const MFX_ERR_UNSUPPORTED: mfxStatus = -3;
/// Failed to allocate memory.
pub const MFX_ERR_MEMORY_ALLOC: mfxStatus = -4;
/// Insufficient buffer for input or output.
pub const MFX_ERR_NOT_ENOUGH_BUFFER: mfxStatus = -5;
/// Invalid Handle
pub const MFX_ERR_INVALID_HANDLE: mfxStatus = -6;
/// Specified object/item/sync point not found.
pub const MFX_ERR_NOT_FOUND: mfxStatus = -9;
/// eed more bitstream at decoding input, encoding input, or video processing input frames.
pub const MFX_ERR_MORE_DATA: mfxStatus = -10;
/// Incompatible video parameters detected. If a Reset function returns this status code, a component—decoder, encoder or video preprocessor—cannot process the specified configuration with existing structures and frame buffers. If the function MFXVideoDECODE_DecodeFrameAsync returns this status code, the bitstream contains an incompatible video parameter configuration that the decoder cannot follow.
pub const MFX_ERR_INVALID_VIDEO_PARAM: mfxStatus = -15;
/// MFX_ERR_UNDEFINED_BEHAVIOR
pub const MFX_ERR_UNDEFINED_BEHAVIOR: mfxStatus = -16;

/// SW is used
pub const MFX_WRN_PARTIAL_ACCELERATION: mfxStatus = 4;
/// The function detected some video parameters were incompatible with others; incompatibility resolved.
pub const MFX_WRN_INCOMPATIBLE_VIDEO_PARAM: mfxStatus = 5;

pub const MFX_TARGETUSAGE_1: u16 = 1;
pub const MFX_TARGETUSAGE_2: u16 = 2;
pub const MFX_TARGETUSAGE_3: u16 = 3;
pub const MFX_TARGETUSAGE_4: u16 = 4;
pub const MFX_TARGETUSAGE_5: u16 = 5;
pub const MFX_TARGETUSAGE_6: u16 = 6;
pub const MFX_TARGETUSAGE_7: u16 = 7;

pub const MFX_TARGETUSAGE_UNKNOWN: u16 = 0;
pub const MFX_TARGETUSAGE_BEST_QUALITY: u16 = MFX_TARGETUSAGE_1;
pub const MFX_TARGETUSAGE_BALANCED: u16 = MFX_TARGETUSAGE_4;
pub const MFX_TARGETUSAGE_BEST_SPEED: u16 = MFX_TARGETUSAGE_7;

pub const MFX_CODEC_AVC: mfxU32 = 0x20435641;
pub const MFX_FOURCC_NV12: mfxU32 = MFX_MAKEFOURCC!('N', 'V', '1', '2');
pub const MFX_FOURCC_YV12: mfxU32 = MFX_MAKEFOURCC!('Y', 'V', '1', '2');

pub const MFX_RATECONTROL_CBR: u16 = 1;
pub const MFX_RATECONTROL_VBR: u16 = 2;

pub const MFX_CHROMAFORMAT_MONOCHROME: u16 = 0;
pub const MFX_CHROMAFORMAT_YUV420: u16 = 1;

pub const MFX_PICSTRUCT_UNKNOWN: u16 = 0;
pub const MFX_PICSTRUCT_PROGRESSIVE: u16 = 1;

pub const MFX_IOPATTERN_IN_VIDEO_MEMORY: u16 = 0x01;
pub const MFX_IOPATTERN_IN_SYSTEM_MEMORY: u16 = 0x02;
pub const MFX_IOPATTERN_OUT_VIDEO_MEMORY: u16 = 0x10;
pub const MFX_IOPATTERN_OUT_SYSTEM_MEMORY: u16 = 0x20;