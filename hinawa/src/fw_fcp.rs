// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;

use FwFcp;

pub trait FwFcpExtManual {
    fn transaction(&self, req_frame: &[u8], resp_frame: &mut [u8])
        -> Result<usize, glib::Error>;
}

impl<O: IsA<FwFcp>> FwFcpExtManual for O {
    fn transaction(&self, req_frame: &[u8], resp_frame: &mut [u8])
        -> Result<usize, glib::Error>
    {
        unsafe {
            let mut resp_frame_size = resp_frame.len();
            let mut error = std::ptr::null_mut();

            hinawa_sys::hinawa_fw_fcp_transaction(
                            self.as_ref().to_glib_none().0,
                            req_frame.as_ptr(),
                            req_frame.len(),
                            &mut resp_frame.as_mut_ptr(),
                            &mut resp_frame_size,
                            &mut error);

            if error.is_null() {
                Ok(resp_frame_size)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
