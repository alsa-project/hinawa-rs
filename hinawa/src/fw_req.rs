// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;

use FwReq;
use FwTcode;
use FwNode;

pub trait FwReqExtManual {
    fn transaction<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                   addr: u64, length: usize, frame: &mut [u8])
        ->Result<(), glib::Error>;
}

impl<O: IsA<FwReq>> FwReqExtManual for O {
    fn transaction<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                   addr: u64, length: usize, frame: &mut [u8])
        ->Result<(), glib::Error>
    {
        unsafe {
            let mut frame_size = frame.len();
            let mut error = std::ptr::null_mut();

            hinawa_sys::hinawa_fw_req_transaction(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                tcode.to_glib(), addr, length,
                &mut frame.as_mut_ptr(), &mut frame_size,
                &mut error);

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
