use std::io::{Error, ErrorKind, self};

use crate::mfxStatus;
use crate::constants::*;

pub fn align16(x: u16) -> u16 {
    ((x + 15) >> 4) << 4
}

pub fn align32(x: u32) -> u32 {
    (x + 31) & !31
}

/// See (intel sdk)[https://github.com/Intel-Media-SDK/MediaSDK/blob/510d19dcace1d8c57567fdd40b557155ab11ab8e/api/include/mfxdefs.h] for additional error meanings
pub fn check_error(status: mfxStatus) -> io::Result<()> {
    match status {
        MFX_ERR_NONE => Ok(()),
        MFX_ERR_UNKNOWN => Err(Error::new(ErrorKind::Other, "unknown")),
        MFX_ERR_NULL_PTR => Err(Error::new(ErrorKind::Other, "null pointer")),
        MFX_ERR_UNSUPPORTED => Err(Error::new(ErrorKind::Other, "unsupported")),
        MFX_ERR_NOT_ENOUGH_BUFFER => Err(Error::new(ErrorKind::Other, "not enough buffer")),
        MFX_ERR_NOT_FOUND => Err(Error::new(ErrorKind::Other, "not found")),
        MFX_ERR_MORE_DATA => Err(Error::new(ErrorKind::Other, "more data")),
        MFX_ERR_INVALID_VIDEO_PARAM => Err(Error::new(ErrorKind::Other, "invalid video param")),
        MFX_ERR_UNDEFINED_BEHAVIOR => Err(Error::new(ErrorKind::Other, "undefined behavior")),
        _ => Err(Error::new(ErrorKind::Other, format!("unknown error = {status}"))),
    }
}

pub fn assert_error_msg(status: mfxStatus, msg: &'static str) {
    let err = check_error(status);
    if err.is_err() {
        panic!("{}: {err:?}", msg);
    }
}

macro_rules! MFX_MAKEFOURCC {
    ( $a:literal, $b:literal, $c:literal, $d:literal ) => {
        $a as u32 + (($b as u32) << 8) + (($c as u32) << 16) + (($d as u32) << 24)
    };
}
// Export macro across crate
pub(crate) use MFX_MAKEFOURCC;

#[cfg(test)]
mod test {
    #[test]
    fn make_four_cc() {
        assert_eq!(MFX_MAKEFOURCC!('N', 'V', '1', '2'), 0x3231564e);
        assert_eq!(MFX_MAKEFOURCC!('Y', 'V', '1', '2'), 0x32315659);
    }
}
