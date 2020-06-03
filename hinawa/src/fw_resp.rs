// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;

use FwResp;

pub trait FwRespExtManual {
    fn get_req_frames(&self) -> &[u8];
}

impl<O: IsA<FwResp>> FwRespExtManual for O {
    fn get_req_frames(&self) -> &[u8] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const u8;
            let mut len = 0 as usize;

            hinawa_sys::hinawa_fw_resp_get_req_frame(
                self.as_ref().to_glib_none().0,
                &mut ptr,
                &mut len);

            std::slice::from_raw_parts(ptr, len)
        }
    }
}
