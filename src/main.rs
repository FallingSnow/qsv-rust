#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::mem;
use std::os::fd::AsRawFd;
use std::ptr;
use std::slice;
// use constants::*;

use libc::c_void;

use crate::utils::{align16, align32, assert_error_msg, check_error};

pub mod utils;
// pub mod constants;

#[derive(Debug)]
struct Params {
    input: String,
    output: String,
    width: usize,
    height: usize,
    bitrate: u16,
}

impl mfxVersion {
    pub const fn new(Major: mfxU16, Minor: mfxU16) -> Self {
        mfxVersion {
            __bindgen_anon_1: mfxVersion__bindgen_ty_1 { Major, Minor },
        }
    }
}

impl mfxFrameAllocRequest {
    pub fn new() -> Self {
        mfxFrameAllocRequest {
            __bindgen_anon_1: mfxFrameAllocRequest__bindgen_ty_1 {
                reserved: [0; 1usize],
            },
            reserved3: [0; 3usize],
            Info: mfxFrameInfo::new(),
            Type: 0,
            NumFrameMin: 0,
            NumFrameSuggested: 0,
            reserved2: 0,
        }
    }
}

impl mfxFrameId {
    pub fn new() -> Self {
        mfxFrameId {
            TemporalId: 0,
            PriorityId: 0,
            __bindgen_anon_1: mfxFrameId__bindgen_ty_1 {
                __bindgen_anon_1: mfxFrameId__bindgen_ty_1__bindgen_ty_1 {
                    QualityId: 0,
                    DependencyId: 0,
                },
            },
        }
    }
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
            __bindgen_anon_1: mfxFrameInfo__bindgen_ty_1 {
                __bindgen_anon_1: mfxFrameInfo__bindgen_ty_1__bindgen_ty_1 {
                    Width: 0,
                    Height: 0,
                    CropX: 0,
                    CropY: 0,
                    CropW: 0,
                    CropH: 0,
                },
            },
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
            __bindgen_anon_1: mfxInfoMFX__bindgen_ty_1 {
                __bindgen_anon_1: mfxInfoMFX__bindgen_ty_1__bindgen_ty_1 {
                    TargetUsage: 0,
                    GopPicSize: 0,
                    GopRefDist: 0,
                    GopOptFlag: 0,
                    IdrInterval: 0,
                    RateControlMethod: 0,
                    __bindgen_anon_1: mfxInfoMFX__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
                        InitialDelayInKB: 0,
                    },
                    BufferSizeInKB: 0,
                    __bindgen_anon_2: mfxInfoMFX__bindgen_ty_1__bindgen_ty_1__bindgen_ty_2 {
                        TargetKbps: 0,
                    },
                    __bindgen_anon_3: mfxInfoMFX__bindgen_ty_1__bindgen_ty_1__bindgen_ty_3 {
                        MaxKbps: 0,
                    },
                    NumSlice: 0,
                    NumRefFrame: 0,
                    EncodedOrder: 0,
                },
            },
        }
    }
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

impl mfxVideoParam {
    pub fn new(info: mfxVideoParam__bindgen_ty_1) -> Self {
        mfxVideoParam {
            AllocId: 0,
            reserved: [0; 2],
            reserved3: 0,
            AsyncDepth: 0,
            __bindgen_anon_1: info,
            Protected: 0,
            IOPattern: 0,
            ExtParam: ptr::null_mut(),
            NumExtParam: 0,
            reserved2: 0,
        }
    }
}

impl mfxFrameSurface1 {
    pub fn new() -> Self {
        let expParam = mfxFrameData__bindgen_ty_1 { reserved2: 0 };
        let param1 = mfxFrameData__bindgen_ty_3 { Y: ptr::null_mut() };
        let param2 = mfxFrameData__bindgen_ty_4 {
            VU: ptr::null_mut(),
        };
        let param3 = mfxFrameData__bindgen_ty_5 { V: ptr::null_mut() };
        mfxFrameSurface1 {
            reserved: [0; 4usize],
            Info: mfxFrameInfo::new(),
            Data: mfxFrameData::new(expParam, param1, param2, param3),
        }
    }
}

impl mfxFrameData {
    pub fn new(
        extParam: mfxFrameData__bindgen_ty_1,
        param1: mfxFrameData__bindgen_ty_3,
        param2: mfxFrameData__bindgen_ty_4,
        param3: mfxFrameData__bindgen_ty_5,
    ) -> Self {
        mfxFrameData {
            __bindgen_anon_1: extParam,
            NumExtParam: 0,
            reserved: [0; 9],
            MemType: 0,
            PitchHigh: 0,
            TimeStamp: 0,
            FrameOrder: 0,
            Locked: 0,
            __bindgen_anon_2: mfxFrameData__bindgen_ty_2 { PitchLow: 0 },

            __bindgen_anon_3: param1,
            __bindgen_anon_4: param2,
            __bindgen_anon_5: param3,
            A: ptr::null_mut(),
            MemId: ptr::null_mut(),
            Corrupted: 0,
            DataFlag: 0,
        }
    }
}

impl mfxBitstream {
    pub fn new() -> Self {
        mfxBitstream {
            __bindgen_anon_1: mfxBitstream__bindgen_ty_1 {
                reserved: [0; 6usize],
            },
            DecodeTimeStamp: 0,
            TimeStamp: 0,
            Data: ptr::null_mut(),
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

fn GetFreeSurfaceIndex(surfaces: &Vec<mfxFrameSurface1>) -> Result<usize, mfxStatus> {
    for i in 0..surfaces.len() {
        if surfaces[i].Data.Locked == 0 {
            return Ok(i);
        }
    }

    return Err(mfxStatus_MFX_ERR_NOT_FOUND);
}

// fn LoadRawFrame(surface: &mut mfxFrameSurface1, file: &mut File) -> Result<mfxStatus, mfxStatus> {
//     let pInfo = &surface.Info;
//     let pData = &surface.Data;
//     let w = pInfo.__bindgen_anon_1.__bindgen_anon_1.CropW as usize;
//     let h = pInfo.__bindgen_anon_1.__bindgen_anon_1.CropH as usize;

//     let size = w * h;
//     let ptr = unsafe { pData.__bindgen_anon_3.Y.offset(0) };
//     let slice = unsafe { slice::from_raw_parts_mut(ptr, size) };
//     let result = file.read(slice);
//     if result.is_err() {
//         return Err(mfxStatus_MFX_ERR_MORE_DATA);
//     }
//     if result.unwrap() == 0 {
//         return Err(mfxStatus_MFX_ERR_MORE_DATA);
//     }

//     let size_uv = size / 4;
//     let ptr_u = unsafe { pData.__bindgen_anon_4.UV.offset(0) };
//     let slice_u = unsafe { slice::from_raw_parts_mut(ptr_u, size_uv) };
//     let result_u = file.read(slice_u);
//     if result_u.is_err() {
//         return Err(mfxStatus_MFX_ERR_MORE_DATA);
//     }
//     if result_u.unwrap() != size_uv {
//         return Err(mfxStatus_MFX_ERR_MORE_DATA);
//     }

//     let ptr_v = unsafe { pData.__bindgen_anon_5.V.offset(0) };
//     let slice_v = unsafe { slice::from_raw_parts_mut(ptr_v, size_uv) };
//     let result_v = file.read(slice_v);
//     if result_v.is_err() {
//         return Err(mfxStatus_MFX_ERR_MORE_DATA);
//     }
//     if result_v.unwrap() != size_uv {
//         return Err(mfxStatus_MFX_ERR_MORE_DATA);
//     }

//     return Err(mfxStatus_MFX_ERR_NONE);
// }

// fn VppToEncSurface(
//     src: &mfxFrameSurface1,
//     dst: &mut mfxFrameSurface1,
// ) -> Result<mfxStatus, mfxStatus> {
//     let info_src = &src.Info;
//     let data_src = &src.Data;

//     let w_src = info_src.__bindgen_anon_1.__bindgen_anon_1.CropW as usize;
//     let h_src = info_src.__bindgen_anon_1.__bindgen_anon_1.CropH as usize;

//     let bits_per_pixel = 12;
//     let size_src = w_src * h_src * bits_per_pixel / 8;

//     let info_dst = &dst.Info;
//     let data_dst = &dst.Data;

//     let w_dst = info_dst.__bindgen_anon_1.__bindgen_anon_1.CropW as usize;
//     let h_dst = info_dst.__bindgen_anon_1.__bindgen_anon_1.CropH as usize;

//     let size_dst = w_dst * h_dst * bits_per_pixel / 8;

//     if size_src != size_dst {
//         return Err(mfxStatus_MFX_ERR_UNKNOWN);
//     }

//     let ptr_src = unsafe { data_src.__bindgen_anon_3.Y.offset(0) };
//     let ptr_dst = unsafe { data_dst.__bindgen_anon_3.Y.offset(0) };

//     unsafe { ptr::copy(ptr_src, ptr_dst, size_src) };

//     return Err(mfxStatus_MFX_ERR_NONE);
// }

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

    // Close any encode sessions
    unsafe { MFXClose(ptr::null_mut()) };

    let implementation = MFX_IMPL_HARDWARE as mfxIMPL | MFX_IMPL_VIA_VAAPI as mfxIMPL;
    let mut version = mfxVersion::new(1, 10);
    let mut session: mfxSession = ptr::null_mut();
    let sts = unsafe { MFXInit(implementation, &mut version, &mut session) };
    assert_error_msg(sts, "MFX initialized");

    let mut actual = MFX_IMPL_SOFTWARE as mfxIMPL;
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

    // Hardware acceleration requires a handle be set to the hardware device you wish to use
    // https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#working-with-va-api-applications
    {
        // let display = cros_libva::Display::open().expect("Failed to find a drm display");
        // Open device via Direct Rendering Infrastructure
        let card = std::fs::File::options()
            .read(true)
            .write(true)
            .open("/dev/dri/card0")?;

        let handle = unsafe { vaGetDisplayDRM(card.as_raw_fd()) } as mfxHDL;
        assert!(
            handle >= ptr::null_mut(),
            "Failed to get drm display handle"
        );
        let sts =
            unsafe { MFXVideoCORE_SetHandle(session, mfxHandleType_MFX_HANDLE_VA_DISPLAY, handle) };
        println!("Set hardware device: {:?}", check_error(sts));
    }

    // Set to default frame allocator
    // https://github.com/Intel-Media-SDK/MediaSDK/blob/master/doc/mediasdk-man.md#mfxvideocore_setframeallocator
    {
        fn alloc(
            allocator: *mut c_void,
            req: *mut mfxFrameAllocRequest,
            res: *mut mfxFrameAllocResponse,
        ) -> mfxStatus {
            -1
        }
        let alloc_ptr = alloc
            as fn(
                allocator: *mut c_void,
                req: *mut mfxFrameAllocRequest,
                res: *mut mfxFrameAllocResponse,
            ) -> mfxStatus;
        let alloc_ptr: unsafe extern "C" fn(
            allocator: *mut c_void,
            req: *mut mfxFrameAllocRequest,
            res: *mut mfxFrameAllocResponse,
        ) -> mfxStatus = unsafe { std::mem::transmute(alloc_ptr) };

        fn lock(allocator: *mut c_void, mid: mfxMemId, ptr: *mut mfxFrameData) -> mfxStatus {
            -1
        }
        let lock_ptr =
            lock as fn(allocator: *mut c_void, mid: mfxMemId, ptr: *mut mfxFrameData) -> mfxStatus;
        let lock_ptr: unsafe extern "C" fn(
            allocator: *mut c_void,
            mid: mfxMemId,
            ptr: *mut mfxFrameData,
        ) -> mfxStatus = unsafe { std::mem::transmute(lock_ptr) };

        fn get_handle(allocator: *mut c_void, mid: mfxMemId, handle: *mut mfxHDL) -> mfxStatus {
            -1
        }
        let get_handle_ptr =
            get_handle as fn(allocator: *mut c_void, mid: mfxMemId, handle: *mut mfxHDL) -> mfxStatus;
        let get_handle_ptr: unsafe extern "C" fn(
            allocator: *mut c_void,
            mid: mfxMemId,
            handle: *mut mfxHDL,
        ) -> mfxStatus = unsafe { std::mem::transmute(get_handle_ptr) };

        fn free(allocator: *mut c_void, res: *mut mfxFrameAllocResponse) -> mfxStatus {
            -1
        }
        let free_ptr =
            free as fn(allocator: *mut c_void, res: *mut mfxFrameAllocResponse) -> mfxStatus;
        let free_ptr: unsafe extern "C" fn(
            allocator: *mut c_void,
            res: *mut mfxFrameAllocResponse,
        ) -> mfxStatus = unsafe { std::mem::transmute(free_ptr) };

        let mut allocator = mfxFrameAllocator {
            reserved: [0u32; 4],
            pthis: ptr::null_mut(),
            Alloc: Some(alloc_ptr),
            Lock: Some(lock_ptr),
            Unlock: Some(lock_ptr),
            GetHDL: Some(get_handle_ptr),
            Free: Some(free_ptr),
        };
        allocator.pthis = &mut allocator as *mut _ as *mut c_void;
        let sts = unsafe { MFXVideoCORE_SetFrameAllocator(session, &mut allocator) };
        println!("Set frame allocator: {:?}", check_error(sts));
    }

    let info = mfxVideoParam__bindgen_ty_1 {
        vpp: mfxInfoVPP::new(),
    };
    let mut VppParams = mfxVideoParam::new(info);
    VppParams.AsyncDepth = 1;
    VppParams.IOPattern = (MFX_IOPATTERN_IN_SYSTEM_MEMORY | MFX_IOPATTERN_OUT_SYSTEM_MEMORY) as u16;

    {
        let mut input = unsafe { &mut VppParams.__bindgen_anon_1.vpp.In };
        input.FourCC = MFX_FOURCC_YV12;
        input.ChromaFormat = MFX_CHROMAFORMAT_YUV420 as u16;
        // input.__bindgen_anon_1.__bindgen_anon_1.CropX = 0;
        // input.__bindgen_anon_1.__bindgen_anon_1.CropY = 0;
        input.__bindgen_anon_1.__bindgen_anon_1.CropW = params.width as u16;
        input.__bindgen_anon_1.__bindgen_anon_1.CropH = params.height as u16;
        input.PicStruct = MFX_PICSTRUCT_PROGRESSIVE as u16;
        input.FrameRateExtN = 30;
        input.FrameRateExtD = 1;
        input.__bindgen_anon_1.__bindgen_anon_1.Width = align16(params.width as u16);
        input.__bindgen_anon_1.__bindgen_anon_1.Height = align16(params.height as u16);

        let mut output = unsafe { &mut VppParams.__bindgen_anon_1.vpp.Out };
        output.FourCC = MFX_FOURCC_NV12;
        output.ChromaFormat = MFX_CHROMAFORMAT_YUV420 as u16;
        // output.__bindgen_anon_1.__bindgen_anon_1.CropX = 0;
        // output.__bindgen_anon_1.__bindgen_anon_1.CropY = 0;
        output.__bindgen_anon_1.__bindgen_anon_1.CropW = params.width as u16;
        output.__bindgen_anon_1.__bindgen_anon_1.CropH = params.height as u16;
        output.PicStruct = MFX_PICSTRUCT_PROGRESSIVE as u16;
        output.FrameRateExtN = 30;
        output.FrameRateExtD = 1;
        output.__bindgen_anon_1.__bindgen_anon_1.Width = align16(params.width as u16);
        output.__bindgen_anon_1.__bindgen_anon_1.Height = align16(params.height as u16);
    }

    // Check which of the requested fields are supported
    {
        let info = mfxVideoParam__bindgen_ty_1 {
            vpp: mfxInfoVPP::new(),
        };
        let mut supported = mfxVideoParam::new(info);
        let sts = unsafe { MFXVideoVPP_Query(session, &mut VppParams, &mut supported) };
        unsafe {
            dbg!(
                supported
                    .__bindgen_anon_1
                    .vpp
                    .In
                    .__bindgen_anon_1
                    .__bindgen_anon_1
                    .Height
            );
        }
        // let recorder = treediff::tools::Recorder::default();
        // treediff::diff(&VppParams, &supported, &mut recorder);
        assert_error_msg(sts, "VPP query");
    }

    let mut VPPRequest = [mfxFrameAllocRequest::new(), mfxFrameAllocRequest::new()];
    let sts = unsafe {
        MFXVideoVPP_QueryIOSurf(
            session,
            &mut VppParams,
            &mut VPPRequest as *mut mfxFrameAllocRequest,
        )
    };
    println!("Checking VPP surfaces: {:?}", check_error(sts));

    let nVPPSurfNumIn: usize = VPPRequest[0].NumFrameSuggested as usize;
    let nVPPSurfNumOut: usize = VPPRequest[1].NumFrameSuggested as usize;

    println!("VPP Surfaces: {}->{}", nVPPSurfNumIn, nVPPSurfNumOut);

    // allocate surfaces for VPP in
    let input = unsafe { &mut VppParams.__bindgen_anon_1.vpp.In };
    let width_vpp_in: usize =
        align32(unsafe { input.__bindgen_anon_1.__bindgen_anon_1.Width as u32 }) as usize;
    let height_vpp_in: usize =
        align32(unsafe { input.__bindgen_anon_1.__bindgen_anon_1.Height as u32 }) as usize;
    let bitsPerPixel = 12; // This is YV12/NV12 bpp
    let surfaceSizeIn = width_vpp_in * height_vpp_in * bitsPerPixel / 8;

    let mut surface_buffers_in: Vec<u8> = Vec::with_capacity(nVPPSurfNumIn * surfaceSizeIn);
    surface_buffers_in.resize(nVPPSurfNumIn * surfaceSizeIn, 0);

    let mut vpp_surfaces_in: Vec<mfxFrameSurface1> = Vec::new();
    for i in 0..nVPPSurfNumIn {
        let mut surface = mfxFrameSurface1::new();
        surface.Info = unsafe { VppParams.__bindgen_anon_1.vpp.In.clone() };
        surface.Data.__bindgen_anon_3.Y = unsafe {
            surface_buffers_in
                .as_mut_ptr()
                .offset((surfaceSizeIn * i) as isize)
        };
        surface.Data.__bindgen_anon_4.UV = unsafe {
            surface
                .Data
                .__bindgen_anon_3
                .Y
                .offset((width_vpp_in * height_vpp_in) as isize)
        };
        surface.Data.__bindgen_anon_5.V = unsafe {
            surface
                .Data
                .__bindgen_anon_4
                .UV
                .offset((width_vpp_in * height_vpp_in / 4) as isize)
        };
        surface.Data.__bindgen_anon_2.PitchLow = width_vpp_in as u16;
        unsafe {
            println!(
                "VPP input surface {}, size: {} x {}",
                i,
                surface.Info.__bindgen_anon_1.__bindgen_anon_1.Width,
                surface.Info.__bindgen_anon_1.__bindgen_anon_1.Height
            );
        }
        vpp_surfaces_in.push(surface);
    }

    // allocate surfaces for VPP out
    let output = unsafe { &mut VppParams.__bindgen_anon_1.vpp.Out };
    let width_vpp_out: usize =
        align32(unsafe { output.__bindgen_anon_1.__bindgen_anon_1.Width as u32 }) as usize;
    let height_vpp_out: usize =
        align32(unsafe { output.__bindgen_anon_1.__bindgen_anon_1.Height as u32 }) as usize;
    let surfaceSizeOut = width_vpp_out * height_vpp_out * bitsPerPixel / 8;

    let mut surface_buffers_out: Vec<u8> = Vec::with_capacity(nVPPSurfNumOut * surfaceSizeOut);
    surface_buffers_out.resize(nVPPSurfNumOut * surfaceSizeOut, 0);

    let mut vpp_surfaces_out: Vec<mfxFrameSurface1> = Vec::new();
    for i in 0..nVPPSurfNumOut {
        let mut surface = mfxFrameSurface1::new();
        surface.Info = unsafe { VppParams.__bindgen_anon_1.vpp.Out.clone() };
        surface.Data.__bindgen_anon_3.Y = unsafe {
            surface_buffers_out
                .as_mut_ptr()
                .offset((surfaceSizeOut * i) as isize)
        };
        surface.Data.__bindgen_anon_4.UV = unsafe {
            surface
                .Data
                .__bindgen_anon_3
                .Y
                .offset((width_vpp_out * height_vpp_out) as isize)
        };
        surface.Data.__bindgen_anon_5.V = unsafe { surface.Data.__bindgen_anon_4.UV.offset(1) };
        surface.Data.__bindgen_anon_2.PitchLow = width_vpp_in as u16;
        unsafe {
            println!(
                "VPP output surface {}, size: {} x {}",
                i,
                surface.Info.__bindgen_anon_1.__bindgen_anon_1.Width,
                surface.Info.__bindgen_anon_1.__bindgen_anon_1.Height
            );
        }
        vpp_surfaces_out.push(surface);
    }

    let sts = unsafe { MFXVideoVPP_Init(session, &mut VppParams) };
    assert_error_msg(sts, "VPP init");

    // let mut EncParams = mfxVideoParam::new();
    // EncParams.u.mfx.CodecId = MFX_CODEC_AVC;
    // EncParams.u.mfx.TargetUsage = MFX_TARGETUSAGE_BALANCED;
    // EncParams.u.mfx.u2.TargetKbps = params.bitrate;
    // EncParams.u.mfx.RateControlMethod = MFX_RATECONTROL_VBR;
    // EncParams.u.mfx.FrameInfo.FrameRateExtN = 30;
    // EncParams.u.mfx.FrameInfo.FrameRateExtD = 1;
    // EncParams.u.mfx.FrameInfo.FourCC = MFX_FOURCC_NV12;
    // EncParams.u.mfx.FrameInfo.ChromaFormat = MFX_CHROMAFORMAT_YUV420;
    // EncParams.u.mfx.FrameInfo.PicStruct = MFX_PICSTRUCT_PROGRESSIVE;
    // EncParams.u.mfx.FrameInfo.CropX = 0;
    // EncParams.u.mfx.FrameInfo.CropY = 0;
    // EncParams.u.mfx.FrameInfo.CropW = params.width as u16;
    // EncParams.u.mfx.FrameInfo.CropH = params.height as u16;
    // EncParams.u.mfx.FrameInfo.Width = align16(params.width as u16);
    // EncParams.u.mfx.FrameInfo.Height = align16(params.height as u16);
    // EncParams.IOPattern = MFX_IOPATTERN_IN_SYSTEM_MEMORY;

    // sts = unsafe { MFXVideoENCODE_Query(session, &EncParams, &mut EncParams) };
    // println!("Checking encoding parameters: {}", sts);

    // let mut encRequest = mfxFrameAllocRequest::new();
    // sts = unsafe { MFXVideoENCODE_QueryIOSurf(session, &EncParams, &mut encRequest) };
    // println!("Checking surfaces: {}", sts);

    // let encSurfNum: usize = encRequest.NumFrameSuggested as usize;
    // let width: usize = align32(encRequest.Info.Width as u32) as usize;
    // let height: usize = align32(encRequest.Info.Height as u32) as usize;
    // let bitsPerPixel = 12;
    // let surfaceSize = (width) * (height) * bitsPerPixel / 8;

    // println!("Surfaces: {}, size: {}", encSurfNum, surfaceSize);

    // let mut surface_buffers_enc: Vec<u8> = Vec::with_capacity(encSurfNum * surfaceSize);
    // surface_buffers_enc.resize(encSurfNum * surfaceSize, 0);

    // let mut enc_surfaces: Vec<mfxFrameSurface1> = Vec::new();
    // for i in 0..encSurfNum {
    //     let mut surface = mfxFrameSurface1::new();
    //     surface.Info = unsafe { EncParams.u.mfx.FrameInfo.clone() };
    //     surface.Data.Y = unsafe {
    //         surface_buffers_enc
    //             .as_mut_ptr()
    //             .offset((surfaceSize * i) as isize)
    //     };
    //     surface.Data.UV = unsafe { surface.Data.Y.offset((width * height) as isize) };
    //     surface.Data.V = unsafe { surface.Data.UV.offset(1) };
    //     surface.Data.PitchLow = width as u16;
    //     println!(
    //         "Encoder surface {}, size: {} x {}",
    //         i, surface.Info.Width, surface.Info.Height
    //     );
    //     enc_surfaces.push(surface);
    // }

    // sts = unsafe { MFXVideoENCODE_Init(session, &EncParams) };
    // println!("Initializing encoder: {}", sts);

    // let mut par = mfxVideoParam::new();
    // let getParam = unsafe { MFXVideoENCODE_GetVideoParam(session, &mut par) };
    // println!("Getting encoder parameters: {}", getParam);
    // let bufferSizeInKB = unsafe { par.u.mfx.BufferSizeInKB } as u32;
    // println!("Buffer BufferSizeInKB: {}", bufferSizeInKB);

    // let mut mfxBS = mfxBitstream::new();
    // mfxBS.MaxLength = 1000 * bufferSizeInKB;
    // let mut encoded: Vec<u8> = Vec::with_capacity(mfxBS.MaxLength as usize);
    // encoded.resize(mfxBS.MaxLength as usize, 0);
    // mfxBS.Data = encoded.as_ptr();

    // let mut syncp_vpp: mfxSyncPoint = ptr::null_mut();
    // let mut syncp_enc: mfxSyncPoint = ptr::null_mut();
    // let mut nFrame: mfxU32 = 0;

    // let mut file_in = File::open(params.input)?;
    // let mut file_out = File::create(params.output)?;

    // // Stage 1: Main encoding loop
    // while mfxStatus_MFX_ERR_NONE <= sts || mfxStatus_MFX_ERR_MORE_DATA == sts {
    //     let mut get_surface_status = GetFreeSurfaceIndex(&vpp_surfaces_in);
    //     if get_surface_status.is_err() {
    //         println!("Error getting VPP in surface");
    //         return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
    //     }
    //     let nSurfIdxIn = get_surface_status.unwrap();

    //     let read_status = LoadRawFrame(&mut vpp_surfaces_in[nSurfIdxIn], &mut file_in);
    //     if read_status.is_err() {
    //         sts = read_status.unwrap_err();
    //         break;
    //     }

    //     get_surface_status = GetFreeSurfaceIndex(&vpp_surfaces_out);
    //     if get_surface_status.is_err() {
    //         println!("Error getting VPP out surface");
    //         return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
    //     }
    //     let nSurfIdxOut = get_surface_status.unwrap();

    //     sts = unsafe {
    //         MFXVideoVPP_RunFrameVPPAsync(
    //             session,
    //             &vpp_surfaces_in[nSurfIdxIn],
    //             &mut vpp_surfaces_out[nSurfIdxOut],
    //             ptr::null(),
    //             &mut syncp_vpp,
    //         )
    //     };

    //     println!(
    //         "VPP result for {} -> {}: {}, sync: {:#?}",
    //         nSurfIdxIn, nSurfIdxOut, sts, syncp_vpp
    //     );

    //     if sts == mfxStatus_MFX_ERR_MORE_DATA {
    //         continue;
    //     }

    //     sts = unsafe { MFXVideoCORE_SyncOperation(session, syncp_vpp, 6000) };
    //     println!("VPP sync result: {}", sts);

    //     get_surface_status = GetFreeSurfaceIndex(&enc_surfaces);
    //     if get_surface_status.is_err() {
    //         println!("Error getting ENC surface");
    //         return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
    //     }
    //     let nEncSurfIdx = get_surface_status.unwrap();

    //     let copy_status = VppToEncSurface(
    //         &vpp_surfaces_out[nSurfIdxOut],
    //         &mut enc_surfaces[nEncSurfIdx],
    //     );

    //     if copy_status.is_err() {
    //         println!("Error copying VPP to ENC");
    //         return Err(Error::new(ErrorKind::Other, "Frame copy error"));
    //     }

    //     sts = unsafe {
    //         MFXVideoENCODE_EncodeFrameAsync(
    //             session,
    //             ptr::null(),
    //             &enc_surfaces[nEncSurfIdx],
    //             &mut mfxBS,
    //             &mut syncp_enc,
    //         )
    //     };

    //     println!("Encode result: {}, sync: {:#?}", sts, syncp_enc);

    //     if mfxStatus_MFX_ERR_NONE < sts {
    //         println!("Encode warning: {}", sts);
    //     }
    //     if mfxStatus_MFX_ERR_NOT_ENOUGH_BUFFER == sts {
    //         println!("Encode not enough buffers");
    //     }
    //     if mfxStatus_MFX_ERR_NONE == sts {
    //         sts = unsafe { MFXVideoCORE_SyncOperation(session, syncp_enc, 6000) };
    //         println!("Encode sync resut: {}", sts);
    //         nFrame += 1;
    //         println!("Processed frame {}", nFrame);

    //         WriteBitStreamFrame(&mut mfxBS, &mut file_out)?;
    //     }
    // }

    // // MFX_ERR_MORE_DATA means that the input file has ended, we do not care flushing encode buffers
    // if sts != mfxStatus_MFX_ERR_MORE_DATA {
    //     return Err(Error::new(ErrorKind::Other, "Encode error"));
    // }

    unsafe { MFXVideoENCODE_Close(session) };

    Ok(())
}

#[cfg(test)]
mod test {}
