// SPDX-License-Identifier: MIT
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::Quark;
use hinawa_sys;

use FwRcode;
use SndEfwStatus;

pub type FwReqError = FwRcode;

impl ErrorDomain for FwReqError {
    fn domain() -> Quark {
        unsafe { from_glib(hinawa_sys::hinawa_fw_req_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        let val = match code {
            0 => Self::Complete,
            4 => Self::ConflictError,
            5 => Self::DataError,
            6 => Self::TypeError,
            7 => Self::AddressError,
            16 => Self::SendError,
            17 => Self::Cancelled,
            18 => Self::Busy,
            19 => Self::Generation,
            20 => Self::NoAck,
            21 => Self::Invalid,
            value => Self::__Unknown(value),
        };
        Some(val)
    }
}

pub type SndEfwError = SndEfwStatus;

impl ErrorDomain for SndEfwError {
    fn domain() -> Quark {
        unsafe { from_glib(hinawa_sys::hinawa_snd_efw_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        let val = match code {
            0 => Self::Ok,
            1 => Self::Bad,
            2 => Self::BadCommand,
            3 => Self::CommErr,
            4 => Self::BadQuadCount,
            5 => Self::Unsupported,
            6 => Self::Timeout,
            7 => Self::DspTimeout,
            8 => Self::BadRate,
            9 => Self::BadClock,
            10 => Self::BadChannel,
            11 => Self::BadPan,
            12 => Self::FlashBusy,
            13 => Self::BadMirror,
            14 => Self::BadLed,
            15 => Self::BadParameter,
            16 => Self::LargeResp,
            code => Self::__Unknown(code),
        };
        Some(val)
    }
}
