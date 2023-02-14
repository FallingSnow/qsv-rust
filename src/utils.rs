use std::io::{Error, ErrorKind, self};

use crate::mfxStatus;

pub fn align16(x: u16) -> u16 {
    ((x + 15) >> 4) << 4
}

pub fn align32(x: u32) -> u32 {
    (x + 31) & !31
}

/// See (intel sdk)[https://github.com/Intel-Media-SDK/MediaSDK/blob/510d19dcace1d8c57567fdd40b557155ab11ab8e/api/include/mfxdefs.h] for additional error meanings
pub fn check_error(status: mfxStatus) -> io::Result<()> {
    dbg!(status);
    match status {
        mfxStatus_MFX_ERR_NONE => Ok(()),
        mfxStatus_MFX_ERR_UNKNOWN => Err(Error::new(ErrorKind::Other, "unknown")),
        mfxStatus_MFX_ERR_NULL_PTR => Err(Error::new(ErrorKind::Other, "null pointer")),
        mfxStatus_MFX_ERR_UNSUPPORTED => Err(Error::new(ErrorKind::Other, "unsupported")),
        mfxStatus_MFX_ERR_NOT_ENOUGH_BUFFER => Err(Error::new(ErrorKind::Other, "not enough buffer")),
        mfxStatus_MFX_ERR_NOT_FOUND => Err(Error::new(ErrorKind::Other, "not found")),
        mfxStatus_MFX_ERR_MORE_DATA => Err(Error::new(ErrorKind::Other, "more data")),
        mfxStatus_MFX_ERR_INVALID_VIDEO_PARAM => Err(Error::new(ErrorKind::Other, "invalid video param")),
        mfxStatus_MFX_ERR_UNDEFINED_BEHAVIOR => Err(Error::new(ErrorKind::Other, "undefined behavior")),
        // _ => Err(Error::new(ErrorKind::Other, format!("unknown error = {status}"))),
    }
}

pub fn assert_error_msg(status: mfxStatus, msg: &'static str) {
    let err = check_error(status);
    if status < 0 {
        panic!("{}: {status} - {err:?}", msg);
    }
}