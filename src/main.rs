#![allow(non_camel_case_types, non_snake_case)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::mem;
use std::ptr;
use std::slice;
use constants::*;

use crate::utils::{align16, align32, assert_error_msg, check_error};

pub mod utils;
pub mod constants;

#[derive(Debug)]
struct Params {
    input: String,
    output: String,
    width: usize,
    height: usize,
    bitrate: u16,
}

#[repr(C)]
pub struct mfxVersion {
    pub Minor: mfxU16,
    pub Major: mfxU16,
}

impl mfxVersion {
    pub const fn new(Major: mfxU16, Minor: mfxU16) -> Self {
        mfxVersion { Major, Minor }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mfxFrameIdStruct1 {
    pub DependencyId: mfxU16,
    pub QualityId: mfxU16,
}

impl mfxFrameIdStruct1 {
    pub fn new() -> Self {
        mfxFrameIdStruct1 {
            DependencyId: 0,
            QualityId: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mfxFrameIdStruct2 {
    pub ViewId: mfxU16,
}

impl mfxFrameIdStruct2 {
    pub fn new() -> Self {
        mfxFrameIdStruct2 { ViewId: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union mfxFrameIdUnion {
    pub s1: mfxFrameIdStruct1,
    pub s2: mfxFrameIdStruct2,
}

impl mfxFrameIdUnion {
    pub fn new() -> Self {
        mfxFrameIdUnion {
            s1: mfxFrameIdStruct1::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mfxFrameId {
    pub TemporalId: mfxU16,
    pub PriorityId: mfxU16,
    pub u: mfxFrameIdUnion,
}

impl mfxFrameId {
    pub fn new() -> Self {
        mfxFrameId {
            TemporalId: 0,
            PriorityId: 0,
            u: mfxFrameIdUnion::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mfxFrameInfo {
    pub reserved: [mfxU32; 4],
    pub reserved4: mfxU16,
    /// Number of bits used to represent luma samples.
    /// Not all codecs and SDK implementations support this value. Use Query function to check if this feature is supported.
    pub BitDepthLuma: mfxU16,
    ///     Number of bits used to represent chroma samples.
    /// Not all codecs and SDK implementations support this value. Use Query function to check if this feature is supported.
    pub BitDepthChroma: mfxU16,
    /// When not zero indicates that values of luma and chroma samples are shifted. Use BitDepthLuma and BitDepthChroma to calculate shift size. Use zero value to indicate absence of shift.
    /// Not all codecs and SDK implementations support this value. Use Query function to check if this feature is supported.
    pub Shift: mfxU16,

    pub FrameId: mfxFrameId,
    /// FourCC code of the color format; see the ColorFourCC enumerator for details.
    pub FourCC: mfxU32,

    // TODO: union, frame paramaters, omit buffer parameters, both 96 bits
    /// Width of the video frame in pixels; Width must be a multiple of 16.
    pub Width: mfxU16,
    /// Height of the video frame in pixels. Height must be a multiple of 16 for progressive frame sequence and a multiple of 32 otherwise.
    pub Height: mfxU16,
    /// Display the region of interest of the frame; specify the display width and height in mfxVideoParam.
    pub CropX: mfxU16,
    /// Display the region of interest of the frame; specify the display width and height in mfxVideoParam.
    pub CropY: mfxU16,
    /// Display the region of interest of the frame; specify the display width and height in mfxVideoParam.
    pub CropW: mfxU16,
    /// Display the region of interest of the frame; specify the display width and height in mfxVideoParam.
    pub CropH: mfxU16,

    /// Specify the frame rate by the formula: FrameRateExtN / FrameRateExtD.
    /// For encoding, frame rate must be specified. For decoding, frame rate may be unspecified (FrameRateExtN and FrameRateExtD are all zeros.) In this case, the frame rate is default to 30 frames per second.
    pub FrameRateExtN: mfxU32,
    pub FrameRateExtD: mfxU32,
    pub reserved3: mfxU16,

    ///     These parameters specify the sample aspect ratio. If sample aspect ratio is explicitly defined by the standards (see Table 6-3 in the MPEG-2 specification or Table E-1 in the H.264 specification), AspectRatioW and AspectRatioH should be the defined values. Otherwise, the sample aspect ratio can be derived as follows:
    /// AspectRatioW=display_aspect_ratio_width*display_height;
    /// For MPEG-2, the above display aspect ratio must be one of the defined values in Table 6-3. For H.264, there is no restriction on display aspect ratio values.
    /// If both parameters are zero, the encoder uses default value of sample aspect ratio.
    pub AspectRatioW: mfxU16,
    /// AspectRatioH=display_aspect_ratio_height*display_width;
    pub AspectRatioH: mfxU16,

    ///	Picture type as specified in the PicStruct enumerator
    pub PicStruct: mfxU16,
    /// Color sampling method; the value of ChromaFormat is the same as that of ChromaFormatIdc. ChromaFormat is not defined if FourCC is zero.
    pub ChromaFormat: mfxU16,
    pub reserved2: mfxU16,
}

impl mfxFrameInfo {
    pub fn new() -> Self {
        mfxFrameInfo {
            reserved: [0; 4],
            reserved4: 0,
            BitDepthLuma: 0,
            BitDepthChroma: 0,
            Shift: 0,
            FrameId: mfxFrameId::new(),
            FourCC: 0,
            Width: 0,
            Height: 0,
            CropX: 0,
            CropY: 0,
            CropW: 0,
            CropH: 0,
            FrameRateExtN: 0,
            FrameRateExtD: 0,
            reserved3: 0,
            AspectRatioW: 0,
            AspectRatioH: 0,
            PicStruct: 0,
            ChromaFormat: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union mfxInfoMFXUnion1 {
    /// InitialDelayInKB, TargetKbps, MaxKbps parameters are for the constant bitrate (CBR), variable bitrate control (VBR) and CQP HRD algorithms.
    /// The SDK encoders follow the Hypothetical Reference Decoding (HRD) model. The HRD model assumes that data flows into a buffer of the fixed size BufferSizeInKB with a constant bitrate TargetKbps. (Estimate the targeted frame size by dividing the framerate by the bitrate.)
    /// The decoder starts decoding after the buffer reaches the initial size InitialDelayInKB, which is equivalent to reaching an initial delay of InitialDelayInKB*8000/TargetKbpsms. Note: In this context, KB is 1000 bytes and Kbps is 1000 bps.
    /// If InitialDelayInKB or BufferSizeInKB is equal to zero, the value is calculated using bitrate, frame rate, profile, level, and so on.
    pub InitialDelayInKB: mfxU16,
    /// Quantization Parameters (QP) for I, P and B frames for constant QP mode (CQP). Zero QP is not valid and means that default value is assigned by MediaSDK. Non-zero QPs might be clipped to supported QP range.
    /// Note: Default QPI/QPP/QPB values are implementation dependent and subject to change without additional notice in this document.
    pub QPI: mfxU16,
    /// TargetKbps, Accuracy, Convergence parameters are for the average variable bitrate control (AVBR) algorithm. The algorithm focuses on overall encoding quality while meeting the specified bitrate, TargetKbps, within the accuracy range Accuracy, after a Convergence period. This method does not follow HRD and the instant bitrate is not capped or padded.
    /// The Accuracy value is specified in the unit of tenth of percent.
    pub Accuracy: mfxU16,
}

impl mfxInfoMFXUnion1 {
    pub fn new() -> Self {
        mfxInfoMFXUnion1 {
            InitialDelayInKB: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union mfxInfoMFXUnion2 {
    /// The TargetKbps value is specified in the unit of 1000 bits per second.
    pub TargetKbps: mfxU16,
    /// Note: Default QPI/QPP/QPB values are implementation dependent and subject to change without additional notice in this document.
    pub QPP: mfxU16,
    /// This parameter is for Intelligent Constant Quality (ICQ) bitrate control algorithm. It is value in the 1…51 range, where 1 corresponds the best quality.
    pub ICQQuality: mfxU16,
}

impl mfxInfoMFXUnion2 {
    pub fn new() -> Self {
        mfxInfoMFXUnion2 { TargetKbps: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union mfxInfoMFXUnion3 {
    pub MaxKbps: mfxU16,
    /// Note: Default QPI/QPP/QPB values are implementation dependent and subject to change without additional notice in this document.
    pub QPB: mfxU16,
    /// The Convergence value is specified in the unit of 100 frames.
    pub Convergence: mfxU16,
}

impl mfxInfoMFXUnion3 {
    pub fn new() -> Self {
        mfxInfoMFXUnion3 { MaxKbps: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxinfomfx>
pub struct mfxInfoMFX {
    reserved: [mfxU32; 7],
    /// For encoders set this flag to ON to reduce power consumption and GPU usage. See the CodingOptionValue enumerator for values of this option. Use Query function to check if this feature is supported.
    pub LowPower: mfxU16,
    /// Specifies a multiplier for bitrate control parameters. Affects next four variables InitialDelayInKB, BufferSizeInKB, TargetKbps, MaxKbps. If this value is not equal to zero encoder calculates BRC parameters as value * BRCParamMultiplier.
    pub BRCParamMultiplier: mfxU16,
    /// mfxFrameInfo structure that specifies frame parameters.
    pub FrameInfo: mfxFrameInfo,
    /// Specifies the codec format identifier in the FOURCC code; see the CodecFormatFourCC enumerator for details. This is a mandated input parameter for QueryIOSurf and Init functions.
    pub CodecId: mfxU32,
    /// Specifies the codec profile; see the CodecProfile enumerator for details. Specify the codec profile explicitly or the SDK functions will determine the correct profile from other sources, such as resolution and bitrate.
    pub CodecProfile: mfxU16,
    /// Codec level; see the CodecLevel enumerator for details. Specify the codec level explicitly or the SDK functions will determine the correct level from other sources, such as resolution and bitrate.
    pub CodecLevel: mfxU16,
    #[deprecated]
    /// Deprecated; Used to represent the number of threads the underlying implementation can use on the host processor. **Always set this parameter to zero.**
    pub NumThread: mfxU16,

    // FIXME: only includes encoding options, need to change to union for other options
    /// Target usage model that guides the encoding process; see the TargetUsage enumerator for details.
    pub TargetUsage: mfxU16,
    /// Number of pictures within the current GOP (Group of Pictures); if GopPicSize = 0, then the GOP size is unspecified. If GopPicSize = 1, only I-frames are used. See Example 17 for pseudo-code that demonstrates how SDK uses this parameter.
    pub GopPicSize: mfxU16,
    /// Distance between I- or P (or GPB) - key frames; if it is zero, the GOP structure is unspecified. Note: If GopRefDist = 1, there are no regular B-frames used (only P or GPB); if mfxExtCodingOption3::GPB is ON, GPB frames (B without backward references) are used instead of P. See Example 17 for pseudo-code that demonstrates how SDK uses this parameter.
    pub GopRefDist: mfxU16,
    /// ORs of the GopOptFlag enumerator indicate the additional flags for the GOP specification; see Example 17 for an example of pseudo-code that demonstrates how to use this parameter.
    pub GopOptFlag: mfxU16,
    /// For H.264, IdrInterval specifies IDR-frame interval in terms of I-frames; if IdrInterval = 0, then every I-frame is an IDR-frame. If IdrInterval = 1, then every other I-frame is an IDR-frame, etc.
    /// For HEVC, if IdrInterval = 0, then only first I-frame is an IDR-frame. If IdrInterval = 1, then every I-frame is an IDR-frame. If IdrInterval = 2, then every other I-frame is an IDR-frame, etc.
    /// For MPEG2, IdrInterval defines sequence header interval in terms of I-frames. If IdrInterval = N, SDK inserts the sequence header before every Nth I-frame. If IdrInterval = 0 (default), SDK inserts the sequence header once at the beginning of the stream.
    /// If GopPicSize or GopRefDist is zero, IdrInterval is undefined.
    pub IdrInterval: mfxU16,
    /// Rate control method; see the RateControlMethod enumerator for details.
    pub RateControlMethod: mfxU16,
    pub u1: mfxInfoMFXUnion1,
    /// BufferSizeInKB represents the maximum possible size of any compressed frames.
    pub BufferSizeInKB: mfxU16,
    pub u2: mfxInfoMFXUnion2,
    pub u3: mfxInfoMFXUnion3,
    /// Number of slices in each video frame; each slice contains one or more macro-block rows. If NumSlice equals zero, the encoder may choose any slice partitioning allowed by the codec standard. See also mfxExtCodingOption2::NumMbPerSlice.
    pub NumSlice: mfxU16,
    /// Max number of all available reference frames (for AVC/HEVC NumRefFrame defines DPB size); if NumRefFrame = 0, this parameter is not specified.
    /// See also mfxExtCodingOption3::NumRefActiveP, NumRefActiveBL0 and NumRefActiveBL1 which set a number of active references.
    pub NumRefFrame: mfxU16,
    /// If not zero, EncodedOrder specifies that ENCODE takes the input surfaces in the encoded order and uses explicit frame type control. Application still must provide GopRefDist and mfxExtCodingOption2::BRefType so SDK can pack headers and build reference lists correctly.
    pub EncodedOrder: mfxU16,
}

impl mfxInfoMFX {
    pub fn new() -> Self {
        mfxInfoMFX {
            reserved: [0; 7],
            LowPower: 0,
            BRCParamMultiplier: 0,
            FrameInfo: mfxFrameInfo::new(),
            CodecId: 0,
            CodecProfile: 0,
            CodecLevel: 0,
            NumThread: 0,
            TargetUsage: 0,
            GopPicSize: 0,
            GopRefDist: 0,
            GopOptFlag: 0,
            IdrInterval: 0,
            RateControlMethod: 0,
            u1: mfxInfoMFXUnion1::new(),
            BufferSizeInKB: 0,
            u2: mfxInfoMFXUnion2::new(),
            u3: mfxInfoMFXUnion3::new(),
            NumSlice: 0,
            NumRefFrame: 0,
            EncodedOrder: 0,
        }
    }
}

/// The mfxExtBuffer structure is the common header definition for external buffers and video processing hints.
#[repr(C)]
pub struct mfxExtBuffer {
    /// Identifier of the buffer content. See the ExtendedBufferID enumerator for a complete list of extended buffers.
    pub BufferId: mfxU32,
    /// Size of the buffer
    pub BufferSz: mfxU32,
}

impl mfxExtBuffer {
    pub fn new() -> Self {
        mfxExtBuffer {
            BufferId: 0,
            BufferSz: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
/// The mfxInfoVPP structure specifies configurations for video processing. A zero value in any of the fields indicates that the corresponding field is not explicitly specified.
pub struct mfxInfoVPP {
    pub reserved: [mfxU32; 8],
    /// Input format for video processing
    pub In: mfxFrameInfo,
    /// Output format for video processing
    pub Out: mfxFrameInfo,
}

impl mfxInfoVPP {
    pub fn new() -> Self {
        mfxInfoVPP {
            reserved: [0; 8],
            In: mfxFrameInfo::new(),
            Out: mfxFrameInfo::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union mfxVideoParamUnion {
    /// Configurations related to encoding, decoding and transcoding; see the definition of the mfxInfoMFX structure for details.
    pub mfx: mfxInfoMFX,
    /// Configurations related to video processing; see the definition of the mfxInfoVPP structure for details.
    pub vpp: mfxInfoVPP,
}

impl mfxVideoParamUnion {
    pub fn new() -> Self {
        mfxVideoParamUnion {
            mfx: mfxInfoMFX::new(),
        }
    }
}

#[repr(C)]
/// The mfxVideoParam structure contains configuration parameters for encoding, decoding, transcoding and video processing.
///
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxvideoparam>
pub struct mfxVideoParam {
    ///	Unique component ID that will be passed by SDK to mfxFrameAllocRequest. Useful in pipelines where several components of the same type share the same allocator.
    pub AllocId: mfxU32,
    reserved: [mfxU32; 2],
    reserved3: mfxU16,
    /// Specifies how many asynchronous operations an application performs before the application explicitly synchronizes the result. If zero, the value is not specified.
    pub AsyncDepth: mfxU16,
    pub u: mfxVideoParamUnion,
    /// Specifies the content protection mechanism; see the Protected enumerator for a list of supported protection schemes.
    pub Protected: mfxU16,
    /// Input and output memory access types for SDK functions; see the enumerator IOPattern for details. The Query functions return the natively supported IOPattern if the Query input argument is NULL. This parameter is a mandated input for QueryIOSurf and Init functions. For DECODE, the output pattern must be specified; for ENCODE, the input pattern must be specified; and for VPP, both input and output pattern must be specified.
    pub IOPattern: mfxU16,
    /// Points to an array of pointers to the extra configuration structures; see the ExtendedBufferID enumerator for a list of extended configurations.
    ///
    /// The list of extended buffers should not contain duplicated entries, i.e. entries of the same type. If mfxVideoParam structure is used to query the SDK capability, then list of extended buffers attached to input and output mfxVideoParam structure should be equal, i.e. should contain the same number of extended buffers of the same type.
    pub ExtParam: *const *const mfxExtBuffer,
    /// The number of extra configuration structures attached to this structure.
    pub NumExtParam: mfxU16,
    reserved2: mfxU16,
}

impl mfxVideoParam {
    pub fn new() -> Self {
        mfxVideoParam {
            AllocId: 0,
            reserved: [0; 2],
            reserved3: 0,
            AsyncDepth: 0,
            u: mfxVideoParamUnion::new(),
            Protected: 0,
            IOPattern: 0,
            ExtParam: ptr::null(),
            NumExtParam: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
/// The mfxFrameAllocRequest structure describes multiple frame allocations when initializing encoders, decoders and video preprocessors. A range specifies the number of video frames. Applications are free to allocate additional frames. In any case, the minimum number of frames must be at least NumFrameMin or the called function will return an error.
pub struct mfxFrameAllocRequest {
    // TODO: AllocId should be stored in a union
    /// Unique (within the session) ID of component requested the allocation.
    pub AllocId: mfxU32,
    reserved3: [mfxU32; 3],
    /// Describes the properties of allocated frames
    pub Info: mfxFrameInfo,
    /// Allocated memory type; see the ExtMemFrameType enumerator for details.
    pub Type: mfxU16,
    /// Minimum number of allocated frames
    pub NumFrameMin: mfxU16,
    /// Suggested number of allocated frames
    pub NumFrameSuggested: mfxU16,
    reserved2: mfxU16,
}

impl mfxFrameAllocRequest {
    pub fn new() -> Self {
        mfxFrameAllocRequest {
            AllocId: 0,
            reserved3: [0; 3],
            Info: mfxFrameInfo::new(),
            Type: 0,
            NumFrameMin: 0,
            NumFrameSuggested: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
/// The mfxFrameData structure describes frame buffer pointers.
///
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxframedata>
pub struct mfxFrameData {
    // TODO: union ExtParam: const* const* mfxExtBuffer
    pub reserved2: mfxU64,
    /// The number of extra configuration structures attached to this structure.
    pub NumExtParam: mfxU16,
    pub reserved: [mfxU16; 9],
    ///	Allocated memory type; see the ExtMemFrameType enumerator for details. Used for better integration of 3rd party plugins into SDK pipeline.
    pub MemType: mfxU16,
    /// PitchHigh/PitchLow params. Distance in bytes between the start of two consecutive rows in a frame.
    pub PitchHigh: mfxU16,
    /// Time stamp of the video frame in units of 90KHz (divide TimeStamp by 90,000 (90 KHz) to obtain the time in seconds). A value of MFX_TIMESTAMP_UNKNOWN indicates that there is no time stamp.
    pub TimeStamp: mfxU64,
    /// Current frame counter for the top field of the current frame; an invalid value of MFX_FRAMEORDER_UNKNOWN indicates that SDK functions that generate the frame output do not use this frame.
    pub FrameOrder: mfxU32,
    /// Counter flag for the application; if Locked is greater than zero then the application locks the frame or field pair. Do not move, alter or delete the frame.
    pub Locked: mfxU16,
    // TODO: union Pitch, pitch is depricated
    /// PitchHigh/PitchLow params. Distance in bytes between the start of two consecutive rows in a frame.
    pub PitchLow: mfxU16,

    /// Data pointers to corresponding color channels. The frame buffer pointers must be 16-byte aligned. The application has to specify pointers to all color channels even for packed formats. For example, for YUY2 format the application has to specify Y, U and V pointers. For RGB32 – R, G, B and A pointers.
    pub Y: *mut mfxU8,
    // union
    pub UV: *mut mfxU8,
    // union
    pub V: *mut mfxU8,
    pub A: *mut mfxU8,
    /// Memory ID of the data buffers; if any of the preceding data pointers is non-zero then the SDK ignores MemId.
    pub MemId: mfxMemId,
    /// Some part of the frame or field pair is corrupted. See the Corruption enumerator for details.
    pub Corrupted: mfxU16,
    /// Additional flags to indicate frame data properties. See the FrameDataFlag enumerator for details.
    pub DataFlag: mfxU16,
}

impl mfxFrameData {
    pub fn new() -> Self {
        mfxFrameData {
            reserved2: 0,
            NumExtParam: 0,
            reserved: [0; 9],
            MemType: 0,
            PitchHigh: 0,
            TimeStamp: 0,
            FrameOrder: 0,
            Locked: 0,
            PitchLow: 0,

            Y: ptr::null_mut(),
            UV: ptr::null_mut(),
            V: ptr::null_mut(),
            A: ptr::null_mut(),
            MemId: ptr::null(),
            Corrupted: 0,
            DataFlag: 0,
        }
    }
}

#[repr(C)]
/// The mfxFrameSurface1 structure defines the uncompressed frames surface information and data buffers. The frame surface is in the frame or complementary field pairs of pixels up to four color-channels, in two parts: mfxFrameInfo and mfxFrameData.
///
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxframesurface1>
pub struct mfxFrameSurface1 {
    reserved: [mfxU32; 4],
    /// mfxFrameInfo structure specifies surface properties
    pub Info: mfxFrameInfo,
    /// mfxFrameData structure describes the actual frame buffer.
    pub Data: mfxFrameData,
}

impl mfxFrameSurface1 {
    pub fn new() -> Self {
        mfxFrameSurface1 {
            reserved: [0; 4],
            Info: mfxFrameInfo::new(),
            Data: mfxFrameData::new(),
        }
    }
}

#[repr(C)]
/// The mfxBitstream structure defines the buffer that holds compressed video data.
///
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxbitstream>
pub struct mfxBitstream {
    // TODO: union encrypted data
    reserved: [mfxU32; 6],
    /// Decode time stamp of the compressed bitstream in units of 90KHz. A value of MFX_TIMESTAMP_UNKNOWN indicates that there is no time stamp.
    /// This value is calculated by the SDK encoder from presentation time stamp provided by the application in mfxFrameSurface1 structure and from frame rate provided by the application during the SDK encoder initialization.
    pub DecodeTimeStamp: mfxI64,
    /// Time stamp of the compressed bitstream in units of 90KHz. A value of MFX_TIMESTAMP_UNKNOWN indicates that there is no time stamp.
    pub TimeStamp: mfxU64,
    /// Bitstream buffer pointer—32-bytes aligned
    pub Data: *const mfxU8,
    /// Next reading or writing position in the bitstream buffer
    pub DataOffset: mfxU32,
    /// Size of the actual bitstream data in bytes
    pub DataLength: mfxU32,
    /// Allocated bitstream buffer size in bytes
    pub MaxLength: mfxU32,
    /// Type of the picture in the bitstream; this is an output parameter.
    pub PicStruct: mfxU16,
    /// Frame type of the picture in the bitstream; this is an output parameter.
    pub FrameType: mfxU16,
    /// Indicates additional bitstream properties; see the BitstreamDataFlag enumerator for details.
    pub DataFlag: mfxU16,
    reserved2: mfxU16,
}

impl mfxBitstream {
    pub fn new() -> Self {
        mfxBitstream {
            reserved: [0; 6],
            DecodeTimeStamp: 0,
            TimeStamp: 0,
            Data: ptr::null(),
            DataOffset: 0,
            DataLength: 0,
            MaxLength: 0,
            PicStruct: 0,
            FrameType: 0,
            DataFlag: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
/// The mfxEncodeCtrl structure contains parameters for per-frame based encoding control.
///
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxencodectrl>
pub struct mfxEncodeCtrl {
    pub Header: mfxExtBuffer,
    pub reserved: [mfxU32; 5],
    /// Type of NAL unit that contains encoding frame. All supported values are defined by MfxNalUnitType enumerator. Other values defined in ITU-T H.265 specification are not supported.
    /// The SDK encoder uses this field only if application sets mfxExtCodingOption3::EnableNalUnitType option to ON during encoder initialization.
    /// Only encoded order is supported. If application specifies this value in display order or uses value inappropriate for current frame or invalid value, then SDK encoder silently ignores it.
    pub MfxNalUnitType: mfxU16,
    /// Indicates that current frame should be skipped or number of missed frames before the current frame. See the mfxExtCodingOption2::SkipFrame for details.
    pub SkipFrame: mfxU16,

    /// If nonzero, this value overwrites the global QP value for the current frame in the constant QP mode.
    pub QP: mfxU16,
    /// Encoding frame type; see the FrameType enumerator for details. If the encoder works in the encoded order, the application must specify the frame type. If the encoder works in the display order, only key frames are enforceable.
    pub FrameType: mfxU16,
    /// Number of extra control buffers.
    pub NumExtParam: mfxU16,
    /// Number of payload records to insert into the bitstream.
    pub NumPayload: mfxU16,
    pub reserved2: mfxU16,

    /// Pointer to an array of pointers to external buffers that provide additional information or control to the encoder for this frame or field pair; a typical usage is to pass the VPP auxiliary data generated by the video processing pipeline to the encoder. See the ExtendedBufferID for the list of extended buffers.
    pub ExtParam: *const *const mfxExtBuffer,
    /// Pointer to an array of pointers to user data (MPEG-2) or SEI messages (H.264) for insertion into the bitstream; for field pictures, odd payloads are associated with the first field and even payloads are associated with the second field. See the mfxPayload structure for payload definitions.
    pub Payload: *const *const mfxPayload,
}

#[repr(C)]
/// The mfxPayload structure describes user data payload in MPEG-2 or SEI message payload in H.264. For encoding, these payloads can be inserted into the bitstream. The payload buffer must contain a valid formatted payload. For H.264, this is the sei_message() as specified in the section 7.3.2.3.1 “Supplemental enhancement information message syntax” of the ISO*/IEC* 14496-10 specification. For MPEG-2, this is the section 6.2.2.2.2 “User data” of the ISO*/IEC* 13818-2 specification, excluding the user data start_code. For decoding, these payloads can be retrieved as the decoder parses the bitstream and caches them in an internal buffer.
///
/// See <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxpayload> for list of Payloads insertion support in encoders.
pub struct mfxPayload {
    ///Additional payload properties. See the PayloadCtrlFlags enumerator for details.
    pub CtrlFlags: mfxU32,
    reserved: [mfxU32; 3],
    /// Pointer to the actual payload data buffer
    pub Data: *const mfxU8,
    /// Number of bits in the payload data
    pub NumBit: mfxU32,
    /// MPEG-2 user data start code or H.264 SEI message type
    pub Type: mfxU16,
    /// Payload buffer size in bytes
    pub BufSize: mfxU16,
}

#[repr(C)]
/// The mfxExtVppAuxData structure returns auxiliary data generated by the video processing pipeline. The encoding process may use the auxiliary data by attaching this structure to the mfxEncodeCtrl structure.
///
/// <https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxextvppauxdata>
pub struct mfxExtVppAuxData {
    /// Header.BufferId must be set to MFX_EXTBUFF_VPP_AUXDATA
    Header: mfxExtBuffer,

    // TODO: union
    #[deprecated]
    SpatialComplexity: mfxU32,
    #[deprecated]
    TemporalComplexity: mfxU32,

    #[deprecated]
    SceneChangeRate: mfxU16,
    #[deprecated]
    RepeatedFrame: mfxU16,
}

// https://github.com/Intel-Media-SDK/MediaSDK#media-sdk-support-matrix
#[link(name = "mfxhw64")]
extern "C" {
    /// This function creates and initializes an SDK session. Call this function before calling any other SDK functions. If the desired implementation specified by impl is MFX_IMPL_AUTO, the function will search for the platform-specific SDK implementation. If the function cannot find it, it will use the software implementation.
    ///
    /// The argument ver indicates the desired version of the library implementation. The loaded SDK will have an API version compatible to the specified version (equal in the major version number, and no less in the minor version number.) If the desired version is not specified, the default is to use the API version from the SDK release, with which an application is built.
    ///
    /// We recommend that production applications always specify the minimum API version that meets their functional requirements. For example, if an application uses only H.264 decoding as described in API v1.0, have the application initialize the library with API v1.0. This ensures backward compatibility.
    ///
    /// # Arguments
    /// * `implementation` - mfxIMPL enumerator that indicates the desired SDK implementation
    /// * `ver` - Pointer to the minimum library version or zero, if not specified
    /// * `session` - Pointer to the SDK session handle
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully. The output parameter contains the handle of the session.
    /// * `MFX_ERR_UNSUPPORTED` - The function cannot find the desired SDK implementation or version.
    pub fn MFXInit(
        implementation: mfxIMPL,
        ver: *const mfxVersion,
        session: *mut *mut mfxSession,
    ) -> mfxStatus;

    /// This function returns the implementation type of a given session.
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `implementation` - Pointer to the implementation type
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully. The output parameter contains the handle of the session.
    ///
    /// # Since
    /// SDK API 1.0
    pub fn MFXQueryIMPL(session: *const mfxSession, implementation: *mut mfxIMPL) -> mfxStatus;

    /// This function works in either of four modes:

    /// If the in pointer is zero, the function returns the class configurability in the output structure. A non-zero value in each field of the output structure indicates that the SDK implementation can configure the field with Init.
    ///
    /// If the in parameter is non-zero, the function checks the validity of the fields in the input structure. Then the function returns the corrected values in the output structure. If there is insufficient information to determine the validity or correction is impossible, the function zeroes the fields. This feature can verify whether the SDK implementation supports certain profiles, levels or bitrates.
    ///
    /// If the in parameter is non-zero and mfxExtEncoderResetOption structure is attached to it, then the function queries for the outcome of the MFXVideoENCODE_Reset function and returns it in the mfxExtEncoderResetOption structure attached to out. The query function succeeds if such reset is possible and returns error otherwise. Unlike other modes that are independent of the SDK encoder state, this one checks if reset is possible in the present SDK encoder state. This mode also requires completely defined mfxVideoParam structure, unlike other modes that support partially defined configurations. See mfxExtEncoderResetOption description for more details.
    ///
    /// If the in parameter is non-zero and mfxExtEncoderCapability structure is attached to it, then the function returns encoder capability in mfxExtEncoderCapability structure attached to out. It is recommended to fill in mfxVideoParam structure and set hardware acceleration device handle before calling the function in this mode.
    ///
    /// The application can call this function before or after it initializes the encoder. The CodecId field of the output structure is a mandated field (to be filled by the application) to identify the coding standard.
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `in` - Pointer to the mfxVideoParam structure as input
    /// * `out` - Pointer to the mfxVideoParam structure as output
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_ERR_UNSUPPORTED` - The function failed to identify a specific implementation for the required features.
    /// * `MFX_WRN_PARTIAL_ACCELERATION` - The underlying hardware does not fully support the specified video parameters; The encoding may be partially accelerated. Only SDK HW implementations may return this status code.
    /// * `MFX_WRN_INCOMPATIBLE_VIDEO_PARAM` - The function detected some video parameters were incompatible with others; incompatibility resolved.
    ///
    /// # Since
    /// SDK API 1.0
    pub fn MFXVideoENCODE_Query(
        session: *const mfxSession,
        input: *const mfxVideoParam,
        output: *mut mfxVideoParam,
    ) -> mfxStatus;

    /// This function returns minimum and suggested numbers of the input frame surfaces required for encoding initialization and their type. Init will call the external allocator for the required frames with the same set of numbers.
    ///
    /// The use of this function is recommended. For more information, see the section Working with hardware acceleration.
    ///
    /// This function does not validate I/O parameters except those used in calculating the number of input surfaces.
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `par` - Pointer to the mfxVideoParam structure as input
    /// * `request` - Pointer to the mfxFrameAllocRequest structure as output
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_WRN_PARTIAL_ACCELERATION` - The underlying hardware does not fully support the specified video parameters; The encoding may be partially accelerated. Only SDK HW implementations may return this status code.
    /// * `MFX_ERR_INVALID_VIDEO_PARAM` - The function detected invalid video parameters. These parameters may be out of the valid range, or the combination of them resulted in incompatibility. Incompatibility not resolved.
    /// * `MFX_WRN_INCOMPATIBLE_VIDEO_PARAM` - The function detected some video parameters were incompatible with others; incompatibility resolved.
    pub fn MFXVideoENCODE_QueryIOSurf(
        session: *const mfxSession,
        par: *const mfxVideoParam,
        request: *mut mfxFrameAllocRequest,
    ) -> mfxStatus;

    /// This function allocates memory and prepares tables and necessary structures for encoding. This function also does extensive validation to ensure if the configuration, as specified in the input parameters, is supported.
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `par` - Pointer to the mfxVideoParam structure
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_WRN_PARTIAL_ACCELERATION` - The underlying hardware does not fully support the specified video parameters; The encoding may be partially accelerated. Only SDK HW implementations may return this status code.
    /// * `MFX_ERR_INVALID_VIDEO_PARAM` - The function detected invalid video parameters. These parameters may be out of the valid range, or the combination of them resulted in incompatibility. Incompatibility not resolved.
    /// * `MFX_WRN_INCOMPATIBLE_VIDEO_PARAM` - The function detected some video parameters were incompatible with others; incompatibility resolved.
    /// * `MFX_ERR_UNDEFINED_BEHAVIOR` - The function is called twice without a close;
    pub fn MFXVideoENCODE_Init(session: *const mfxSession, par: *const mfxVideoParam) -> mfxStatus;

    /// This function retrieves current working parameters to the specified output structure. If extended buffers are to be returned, the application must allocate those extended buffers and attach them as part of the output structure.
    ///
    /// The application can retrieve a copy of the bitstream header, by attaching the mfxExtCodingOptionSPSPPS structure to the mfxVideoParam structure.
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `par` - Pointer to the corresponding parameter structure
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    pub fn MFXVideoENCODE_GetVideoParam(
        session: *const mfxSession,
        par: *mut mfxVideoParam,
    ) -> mfxStatus;

    /// This function takes a single input frame in either encoded or display order and generates its output bitstream. In the case of encoded ordering the mfxEncodeCtrl structure must specify the explicit frame type. In the case of display ordering, this function handles frame order shuffling according to the GOP structure parameters specified during initialization.
    ///
    /// Since encoding may process frames differently from the input order, not every call of the function generates output and the function returns MFX_ERR_MORE_DATA. If the encoder needs to cache the frame, the function locks the frame. The application should not alter the frame until the encoder unlocks the frame. If there is output (with return status MFX_ERR_NONE), the return is a frame worth of bitstream.
    ///
    /// It is the calling application’s responsibility to ensure that there is sufficient space in the output buffer. The value BufferSizeInKB in the mfxVideoParam structure at encoding initialization specifies the maximum possible size for any compressed frames. This value can also be obtained from MFXVideoENCODE_GetVideoParam after encoding initialization.
    ///
    /// To mark the end of the encoding sequence, call this function with a NULL surface pointer. Repeat the call to drain any remaining internally cached bitstreams(one frame at a time) until MFX_ERR_MORE_DATA is returned.
    ///
    /// This function is asynchronous.
    /// # Remarks
    ///
    /// If the EncodedOrder field in the mfxInfoMFX structure is true, input frames enter the encoder in the order of their encoding. However, the FrameOrder field in the mfxFrameData structure of each frame must be set to the display order. If EncodedOrder is false, the function ignores the FrameOrder field.
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `ctrl` - Pointer to the mfxEncodeCtrl structure for per-frame encoding control; this parameter is optional(it can be NULL) if the encoder works in the display order mode.
    /// * `surface` - Pointer to the frame surface structure
    /// * `bs` - Pointer to the output bitstream
    /// * `syncp` - Pointer to the returned sync point associated with this operation
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_ERR_NOT_ENOUGH_BUFFER` - The bitstream buffer size is insufficient.
    /// * `MFX_ERR_MORE_DATA` - The function requires more data to generate any output.
    /// * `MFX_ERR_DEVICE_LOST` - Hardware device was lost; See Working with Microsoft* DirectX* Applications section for further information.
    /// * `MFX_WRN_DEVICE_BUSY` - Hardware device is currently busy. Call this function again in a few milliseconds.
    /// * `MFX_ERR_INCOMPATIBLE_VIDEO_PARAM` - Inconsistent parameters detected not conforming to Appendix A.
    pub fn MFXVideoENCODE_EncodeFrameAsync(
        session: *const mfxSession,
        ctrl: *const mfxEncodeCtrl,
        surface: *const mfxFrameSurface1,
        bs: *mut mfxBitstream,
        syncp: *mut mfxSyncPoint,
    ) -> mfxStatus;

    /// This function initiates execution of an asynchronous function not already started and returns the status code after the specified asynchronous operation completes. If wait is zero, the function returns immediately.
    ///
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `syncp` - Sync point
    /// * `wait` - Wait time in milliseconds
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_ERR_NONE_PARTIAL_OUTPUT` - The function completed successfully, bitstream contains a portion of the encoded frame according to required granularity.
    /// * `MFX_WRN_IN_EXECUTION` - The specified asynchronous function is in execution.
    /// * `MFX_ERR_ABORTED` - he specified asynchronous function aborted due to data dependency on a previous asynchronous function that did not complete.
    pub fn MFXVideoCORE_SyncOperation(
        session: *const mfxSession,
        syncp: mfxSyncPoint,
        wait: mfxU32,
    ) -> mfxStatus;

    pub fn MFXVideoENCODE_Close(session: *const mfxSession) -> mfxStatus;

    /// This function works in either of two modes:
    ///
    /// If in is zero, the function returns the class configurability in the output structure. A non-zero value in a field indicates that the SDK implementation can configure it with Init.
    ///
    /// If in is non-zero, the function checks the validity of the fields in the input structure. Then the function returns the corrected values in the output structure. If there is insufficient information to determine the validity or correction is impossible, the function zeroes the fields.
    ///
    /// The application can call this function before or after it initializes the preprocessor.
    ///
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `in` - Pointer to the mfxVideoParam structure as input
    /// * `out` - Pointer to the mfxVideoParam structure as output
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_WRN_PARTIAL_ACCELERATION` - The underlying hardware does not fully support the specified video parameters; The video processing may be partially accelerated. Only SDK HW implementation may return this status code.
    /// * `MFX_ERR_INVALID_VIDEO_PARAM` - The function detected invalid video parameters. These parameters may be out of the valid range, or the combination of them resulted in incompatibility. Incompatibility not resolved.
    /// * `MFX_WRN_INCOMPATIBLE_VIDEO_PARAM` - The function detected some video parameters were incompatible with others; incompatibility resolved.
    pub fn MFXVideoVPP_Query(
        session: *const mfxSession,
        r#in: *const mfxVideoParam,
        out: *const mfxVideoParam,
    ) -> mfxStatus;

    /// This function returns minimum and suggested numbers of input and output frame surfaces required for video processing initialization and their type. The parameter request[0] refers to the input requirements; request[1] refers to output requirements. Init will call the external allocator for the required frames with the same set of numbers.
    ///
    /// The function is recommended. For more information, see the Working with hardware acceleration.
    ///
    /// This function does not validate I/O parameters except those used in calculating the number of input and output surfaces.
    ///
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `par` - Pointer to the mfxVideoParam structure as input
    /// * `request` - Pointer to the output mfxFrameAllocRequest structure; use request[0] for input requirements and request[1] for output requirements for video processing.
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The function completed successfully.
    /// * `MFX_WRN_PARTIAL_ACCELERATION` - The underlying hardware does not fully support the specified video parameters; The video processing may be partially accelerated. Only SDK HW implementation may return this status code.
    /// * `MFX_ERR_INVALID_VIDEO_PARAM` - The function detected invalid video parameters. These parameters may be out of the valid range, or the combination of them resulted in incompatibility. Incompatibility not resolved.
    /// * `MFX_WRN_INCOMPATIBLE_VIDEO_PARAM` - The function detected some video parameters were incompatible with others; incompatibility resolved.
    pub fn MFXVideoVPP_QueryIOSurf(
        session: *const mfxSession,
        par: *const mfxVideoParam,
        request: &mut [mfxFrameAllocRequest; 2],
    ) -> mfxStatus;

    pub fn MFXVideoVPP_Init(session: *const mfxSession, par: *mut mfxVideoParam) -> mfxStatus;

    /// This function processes a single input frame to a single output frame. Retrieval of the auxiliary data is optional; the encoding process may use it.
    ///
    /// The video processing process may not generate an instant output given an input. See section Video Processing Procedures for details on how to correctly send input and retrieve output.
    ///
    /// At the end of the stream, call this function with the input argument in=NULL to retrieve any remaining frames, until the function returns MFX_ERR_MORE_DATA.
    ///
    /// This function is asynchronous.
    ///
    /// # Arguments
    /// * `session` - SDK session handle
    /// * `in` - Pointer to the input video surface structure
    /// * `out` - Pointer to the output video surface structure
    /// * `aux` - Optional pointer to the auxiliary data structure
    /// * `syncp` - Pointer to the output sync point
    ///
    /// # Returns
    /// * `MFX_ERR_NONE` - The output frame is ready after synchronization.
    /// * `MFX_ERR_MORE_DATA` - Need more input frames before VPP can produce an output
    /// * `MFX_ERR_MORE_SURFACE` - The output frame is ready after synchronization. Need more surfaces at output for additional output frames available.
    /// * `MFX_ERR_DEVICE_LOST` - Hardware device was lost; See the Working with Microsoft* DirectX* Applications section for further information.
    /// * `MFX_WRN_DEVICE_BUSY` - Hardware device is currently busy. Call this function again in a few milliseconds.
    pub fn MFXVideoVPP_RunFrameVPPAsync(
        session: *const mfxSession,
        input: *const mfxFrameSurface1,
        output: *mut mfxFrameSurface1,
        aux: *const mfxExtVppAuxData,
        syncp: *mut mfxSyncPoint,
    ) -> mfxStatus;

    pub fn MFXVideoVPP_Close(session: *const mfxSession) -> mfxStatus;
}

fn GetFreeSurfaceIndex(surfaces: &Vec<mfxFrameSurface1>) -> Result<usize, mfxStatus> {
    for i in 0..surfaces.len() {
        if surfaces[i].Data.Locked == 0 {
            return Ok(i);
        }
    }

    return Err(MFX_ERR_NOT_FOUND);
}

fn LoadRawFrame(surface: &mut mfxFrameSurface1, file: &mut File) -> Result<mfxStatus, mfxStatus> {
    let pInfo = &surface.Info;
    let pData = &surface.Data;
    let w = pInfo.CropW as usize;
    let h = pInfo.CropH as usize;

    let size = w * h;
    let ptr = unsafe { pData.Y.offset(0) };
    let slice = unsafe { slice::from_raw_parts_mut(ptr, size) };
    let result = file.read(slice);
    if result.is_err() {
        return Err(MFX_ERR_MORE_DATA);
    }
    if result.unwrap() == 0 {
        return Err(MFX_ERR_MORE_DATA);
    }

    let size_uv = size / 4;
    let ptr_u = unsafe { pData.UV.offset(0) };
    let slice_u = unsafe { slice::from_raw_parts_mut(ptr_u, size_uv) };
    let result_u = file.read(slice_u);
    if result_u.is_err() {
        return Err(MFX_ERR_MORE_DATA);
    }
    if result_u.unwrap() != size_uv {
        return Err(MFX_ERR_MORE_DATA);
    }

    let ptr_v = unsafe { pData.V.offset(0) };
    let slice_v = unsafe { slice::from_raw_parts_mut(ptr_v, size_uv) };
    let result_v = file.read(slice_v);
    if result_v.is_err() {
        return Err(MFX_ERR_MORE_DATA);
    }
    if result_v.unwrap() != size_uv {
        return Err(MFX_ERR_MORE_DATA);
    }

    return Ok(MFX_ERR_NONE);
}

fn VppToEncSurface(
    src: &mfxFrameSurface1,
    dst: &mut mfxFrameSurface1,
) -> Result<mfxStatus, mfxStatus> {
    let info_src = &src.Info;
    let data_src = &src.Data;

    let w_src = info_src.CropW as usize;
    let h_src = info_src.CropH as usize;

    let bits_per_pixel = 12;
    let size_src = w_src * h_src * bits_per_pixel / 8;

    let info_dst = &dst.Info;
    let data_dst = &dst.Data;

    let w_dst = info_dst.CropW as usize;
    let h_dst = info_dst.CropH as usize;

    let size_dst = w_dst * h_dst * bits_per_pixel / 8;

    if size_src != size_dst {
        return Err(MFX_ERR_UNKNOWN);
    }

    let ptr_src = unsafe { data_src.Y.offset(0) };
    let ptr_dst = unsafe { data_dst.Y.offset(0) };

    unsafe { ptr::copy(ptr_src, ptr_dst, size_src) };

    return Ok(MFX_ERR_NONE);
}

fn WriteBitStreamFrame(pMfxBitstream: &mut mfxBitstream, file: &mut File) -> io::Result<()> {
    let buffer = unsafe {
        slice::from_raw_parts(
            pMfxBitstream.Data.offset(pMfxBitstream.DataOffset as isize),
            pMfxBitstream.DataLength as usize,
        )
    };
    let nBytesWritten = file.write(buffer)?;
    if nBytesWritten != (pMfxBitstream.DataLength as usize) {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    pMfxBitstream.DataLength = 0;
    return Ok(());
}

fn main() -> io::Result<()> {
    println!("Size of mfxFrameInfo: {}", mem::size_of::<mfxFrameInfo>());
    println!("Size of mfxInfoMFX: {}", mem::size_of::<mfxInfoMFX>());
    println!("Size of mfxInfoVPP: {}", mem::size_of::<mfxInfoVPP>());
    println!("Size of mfxVideoParam: {}", mem::size_of::<mfxVideoParam>());

    let mut sts: mfxStatus;
    let implementation = MFX_IMPL_HARDWARE_ANY;
    let version = mfxVersion::new(1, 0);
    let mut session: *mut mfxSession = ptr::null_mut();
    sts = unsafe { MFXInit(implementation, &version, &mut session) };
    assert_error_msg(sts, "MFX initialized");

    let mut actual = MFX_IMPL_HARDWARE_ANY;
    unsafe { MFXQueryIMPL(session, &mut actual) };
    println!("H264 implementation: 0x{:x}", actual);

    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("Usage: {} input output width height bitrate", args[0]);
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let params = Params {
        input: args[1].clone(),
        output: args[2].clone(),
        width: args[3].parse::<usize>().unwrap(),
        height: args[4].parse::<usize>().unwrap(),
        bitrate: args[5].parse::<u16>().unwrap(),
    };
    println!("{:?}", params);

    let mut configurability = mfxVideoParam::new();
    sts = unsafe { MFXVideoVPP_Query(session, 0 as *const mfxVideoParam, &mut configurability) };
    assert_error_msg(sts, "VPP query");
    // dbg!(configurability.u.vpp.In.AspectRatioH);

    let mut VppParams = mfxVideoParam::new();
    VppParams.u.vpp.In.FourCC = MFX_FOURCC_YV12;
    VppParams.u.vpp.In.ChromaFormat = MFX_CHROMAFORMAT_YUV420;
    VppParams.u.vpp.In.CropX = 0;
    VppParams.u.vpp.In.CropY = 0;
    VppParams.u.vpp.In.CropW = params.width as u16;
    VppParams.u.vpp.In.CropH = params.height as u16;
    VppParams.u.vpp.In.PicStruct = MFX_PICSTRUCT_PROGRESSIVE;
    VppParams.u.vpp.In.FrameRateExtN = 30;
    VppParams.u.vpp.In.FrameRateExtD = 1;
    VppParams.u.vpp.In.Width = align16(params.width as u16);
    VppParams.u.vpp.In.Height = align16(params.height as u16);

    VppParams.u.vpp.Out.FourCC = MFX_FOURCC_NV12;
    VppParams.u.vpp.Out.ChromaFormat = MFX_CHROMAFORMAT_YUV420;
    VppParams.u.vpp.Out.CropX = 0;
    VppParams.u.vpp.Out.CropY = 0;
    VppParams.u.vpp.Out.CropW = params.width as u16;
    VppParams.u.vpp.Out.CropH = params.height as u16;
    VppParams.u.vpp.Out.PicStruct = MFX_PICSTRUCT_PROGRESSIVE;
    VppParams.u.vpp.Out.FrameRateExtN = 30;
    VppParams.u.vpp.Out.FrameRateExtD = 1;
    VppParams.u.vpp.Out.Width = align16(params.width as u16);
    VppParams.u.vpp.Out.Height = align16(params.height as u16);
    VppParams.IOPattern = MFX_IOPATTERN_IN_SYSTEM_MEMORY | MFX_IOPATTERN_OUT_SYSTEM_MEMORY;

    let mut VPPRequest = [mfxFrameAllocRequest::new(), mfxFrameAllocRequest::new()];
    sts = unsafe { MFXVideoVPP_QueryIOSurf(session, &VppParams, &mut VPPRequest) };
    println!("Checking VPP surfaces: {sts}");
    println!("{:?}", check_error(sts));

    let nVPPSurfNumIn: usize = VPPRequest[0].NumFrameSuggested as usize;
    let nVPPSurfNumOut: usize = VPPRequest[1].NumFrameSuggested as usize;

    println!("VPP Surfaces: {}->{}", nVPPSurfNumIn, nVPPSurfNumOut);

    // allocate surfaces for VPP in
    let width_vpp_in: usize = align32(unsafe { VppParams.u.vpp.In.Width as u32 }) as usize;
    let height_vpp_in: usize = align32(unsafe { VppParams.u.vpp.In.Height as u32 }) as usize;
    let bitsPerPixel = 12;
    let surfaceSizeIn = width_vpp_in * height_vpp_in * bitsPerPixel / 8;

    let mut surface_buffers_in: Vec<u8> = Vec::with_capacity(nVPPSurfNumIn * surfaceSizeIn);
    surface_buffers_in.resize(nVPPSurfNumIn * surfaceSizeIn, 0);

    let mut vpp_surfaces_in: Vec<mfxFrameSurface1> = Vec::new();
    for i in 0..nVPPSurfNumIn {
        let mut surface = mfxFrameSurface1::new();
        surface.Info = unsafe { VppParams.u.vpp.In.clone() };
        surface.Data.Y = unsafe {
            surface_buffers_in
                .as_mut_ptr()
                .offset((surfaceSizeIn * i) as isize)
        };
        surface.Data.UV = unsafe {
            surface
                .Data
                .Y
                .offset((width_vpp_in * height_vpp_in) as isize)
        };
        surface.Data.V = unsafe {
            surface
                .Data
                .UV
                .offset((width_vpp_in * height_vpp_in / 4) as isize)
        };
        surface.Data.PitchLow = width_vpp_in as u16;
        println!(
            "VPP input surface {}, size: {} x {}",
            i, surface.Info.Width, surface.Info.Height
        );
        vpp_surfaces_in.push(surface);
    }

    // allocate surfaces for VPP out
    let width_vpp_out: usize = align32(unsafe { VppParams.u.vpp.Out.Width as u32 }) as usize;
    let height_vpp_out: usize = align32(unsafe { VppParams.u.vpp.Out.Height as u32 }) as usize;
    let surfaceSizeOut = width_vpp_out * height_vpp_out * bitsPerPixel / 8;

    let mut surface_buffers_out: Vec<u8> = Vec::with_capacity(nVPPSurfNumOut * surfaceSizeOut);
    surface_buffers_out.resize(nVPPSurfNumOut * surfaceSizeOut, 0);

    let mut vpp_surfaces_out: Vec<mfxFrameSurface1> = Vec::new();
    for i in 0..nVPPSurfNumOut {
        let mut surface = mfxFrameSurface1::new();
        surface.Info = unsafe { VppParams.u.vpp.Out.clone() };
        surface.Data.Y = unsafe {
            surface_buffers_out
                .as_mut_ptr()
                .offset((surfaceSizeOut * i) as isize)
        };
        surface.Data.UV = unsafe {
            surface
                .Data
                .Y
                .offset((width_vpp_out * height_vpp_out) as isize)
        };
        surface.Data.V = unsafe { surface.Data.UV.offset(1) };
        surface.Data.PitchLow = width_vpp_in as u16;
        println!(
            "VPP output surface {}, size: {} x {}",
            i, surface.Info.Width, surface.Info.Height
        );
        vpp_surfaces_out.push(surface);
    }

    sts = unsafe { MFXVideoVPP_Init(session, &mut VppParams) };
    assert_error_msg(sts, "VPP init");

    let mut EncParams = mfxVideoParam::new();
    EncParams.u.mfx.CodecId = MFX_CODEC_AVC;
    EncParams.u.mfx.TargetUsage = MFX_TARGETUSAGE_BALANCED;
    EncParams.u.mfx.u2.TargetKbps = params.bitrate;
    EncParams.u.mfx.RateControlMethod = MFX_RATECONTROL_VBR;
    EncParams.u.mfx.FrameInfo.FrameRateExtN = 30;
    EncParams.u.mfx.FrameInfo.FrameRateExtD = 1;
    EncParams.u.mfx.FrameInfo.FourCC = MFX_FOURCC_NV12;
    EncParams.u.mfx.FrameInfo.ChromaFormat = MFX_CHROMAFORMAT_YUV420;
    EncParams.u.mfx.FrameInfo.PicStruct = MFX_PICSTRUCT_PROGRESSIVE;
    EncParams.u.mfx.FrameInfo.CropX = 0;
    EncParams.u.mfx.FrameInfo.CropY = 0;
    EncParams.u.mfx.FrameInfo.CropW = params.width as u16;
    EncParams.u.mfx.FrameInfo.CropH = params.height as u16;
    EncParams.u.mfx.FrameInfo.Width = align16(params.width as u16);
    EncParams.u.mfx.FrameInfo.Height = align16(params.height as u16);
    EncParams.IOPattern = MFX_IOPATTERN_IN_SYSTEM_MEMORY;

    sts = unsafe { MFXVideoENCODE_Query(session, &EncParams, &mut EncParams) };
    println!("Checking encoding parameters: {}", sts);

    let mut encRequest = mfxFrameAllocRequest::new();
    sts = unsafe { MFXVideoENCODE_QueryIOSurf(session, &EncParams, &mut encRequest) };
    println!("Checking surfaces: {}", sts);

    let encSurfNum: usize = encRequest.NumFrameSuggested as usize;
    let width: usize = align32(encRequest.Info.Width as u32) as usize;
    let height: usize = align32(encRequest.Info.Height as u32) as usize;
    let bitsPerPixel = 12;
    let surfaceSize = (width) * (height) * bitsPerPixel / 8;

    println!("Surfaces: {}, size: {}", encSurfNum, surfaceSize);

    let mut surface_buffers_enc: Vec<u8> = Vec::with_capacity(encSurfNum * surfaceSize);
    surface_buffers_enc.resize(encSurfNum * surfaceSize, 0);

    let mut enc_surfaces: Vec<mfxFrameSurface1> = Vec::new();
    for i in 0..encSurfNum {
        let mut surface = mfxFrameSurface1::new();
        surface.Info = unsafe { EncParams.u.mfx.FrameInfo.clone() };
        surface.Data.Y = unsafe {
            surface_buffers_enc
                .as_mut_ptr()
                .offset((surfaceSize * i) as isize)
        };
        surface.Data.UV = unsafe { surface.Data.Y.offset((width * height) as isize) };
        surface.Data.V = unsafe { surface.Data.UV.offset(1) };
        surface.Data.PitchLow = width as u16;
        println!(
            "Encoder surface {}, size: {} x {}",
            i, surface.Info.Width, surface.Info.Height
        );
        enc_surfaces.push(surface);
    }

    sts = unsafe { MFXVideoENCODE_Init(session, &EncParams) };
    println!("Initializing encoder: {}", sts);

    let mut par = mfxVideoParam::new();
    let getParam = unsafe { MFXVideoENCODE_GetVideoParam(session, &mut par) };
    println!("Getting encoder parameters: {}", getParam);
    let bufferSizeInKB = unsafe { par.u.mfx.BufferSizeInKB } as u32;
    println!("Buffer BufferSizeInKB: {}", bufferSizeInKB);

    let mut mfxBS = mfxBitstream::new();
    mfxBS.MaxLength = 1000 * bufferSizeInKB;
    let mut encoded: Vec<u8> = Vec::with_capacity(mfxBS.MaxLength as usize);
    encoded.resize(mfxBS.MaxLength as usize, 0);
    mfxBS.Data = encoded.as_ptr();

    let mut syncp_vpp: mfxSyncPoint = ptr::null_mut();
    let mut syncp_enc: mfxSyncPoint = ptr::null_mut();
    let mut nFrame: mfxU32 = 0;

    let mut file_in = File::open(params.input)?;
    let mut file_out = File::create(params.output)?;

    // Stage 1: Main encoding loop
    while MFX_ERR_NONE <= sts || MFX_ERR_MORE_DATA == sts {
        let mut get_surface_status = GetFreeSurfaceIndex(&vpp_surfaces_in);
        if get_surface_status.is_err() {
            println!("Error getting VPP in surface");
            return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
        }
        let nSurfIdxIn = get_surface_status.unwrap();

        let read_status = LoadRawFrame(&mut vpp_surfaces_in[nSurfIdxIn], &mut file_in);
        if read_status.is_err() {
            sts = read_status.unwrap_err();
            break;
        }

        get_surface_status = GetFreeSurfaceIndex(&vpp_surfaces_out);
        if get_surface_status.is_err() {
            println!("Error getting VPP out surface");
            return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
        }
        let nSurfIdxOut = get_surface_status.unwrap();

        sts = unsafe {
            MFXVideoVPP_RunFrameVPPAsync(
                session,
                &vpp_surfaces_in[nSurfIdxIn],
                &mut vpp_surfaces_out[nSurfIdxOut],
                ptr::null(),
                &mut syncp_vpp,
            )
        };

        println!(
            "VPP result for {} -> {}: {}, sync: {:#?}",
            nSurfIdxIn, nSurfIdxOut, sts, syncp_vpp
        );

        if sts == MFX_ERR_MORE_DATA {
            continue;
        }

        sts = unsafe { MFXVideoCORE_SyncOperation(session, syncp_vpp, 6000) };
        println!("VPP sync result: {}", sts);

        get_surface_status = GetFreeSurfaceIndex(&enc_surfaces);
        if get_surface_status.is_err() {
            println!("Error getting ENC surface");
            return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
        }
        let nEncSurfIdx = get_surface_status.unwrap();

        let copy_status = VppToEncSurface(
            &vpp_surfaces_out[nSurfIdxOut],
            &mut enc_surfaces[nEncSurfIdx],
        );

        if copy_status.is_err() {
            println!("Error copying VPP to ENC");
            return Err(Error::new(ErrorKind::Other, "Frame copy error"));
        }

        sts = unsafe {
            MFXVideoENCODE_EncodeFrameAsync(
                session,
                ptr::null(),
                &enc_surfaces[nEncSurfIdx],
                &mut mfxBS,
                &mut syncp_enc,
            )
        };

        println!("Encode result: {}, sync: {:#?}", sts, syncp_enc);

        if MFX_ERR_NONE < sts {
            println!("Encode warning: {}", sts);
        }
        if MFX_ERR_NOT_ENOUGH_BUFFER == sts {
            println!("Encode not enough buffers");
        }
        if MFX_ERR_NONE == sts {
            sts = unsafe { MFXVideoCORE_SyncOperation(session, syncp_enc, 6000) };
            println!("Encode sync resut: {}", sts);
            nFrame += 1;
            println!("Processed frame {}", nFrame);

            WriteBitStreamFrame(&mut mfxBS, &mut file_out)?;
        }
    }

    // MFX_ERR_MORE_DATA means that the input file has ended, we do not care flushing encode buffers
    if sts != MFX_ERR_MORE_DATA {
        return Err(Error::new(ErrorKind::Other, "Encode error"));
    }

    unsafe { MFXVideoENCODE_Close(session) };

    Ok(())
}

#[cfg(test)]
mod test {
    
}
