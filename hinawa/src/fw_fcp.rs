// SPDX-License-Identifier: MIT
use crate::*;

pub trait FwFcpExtManual {
    #[doc(alias = "hinawa_fw_fcp_avc_transaction")]
    fn avc_transaction(
        &self,
        req_frame: &[u8],
        resp_frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<usize, glib::Error>;
    #[doc(alias = "responded")]
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &[u8]) + 'static;
}

impl<O: IsA<FwFcp>> FwFcpExtManual for O {
    fn avc_transaction(
        &self,
        req_frame: &[u8],
        resp_frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<usize, glib::Error> {
        unsafe {
            let mut resp_frame_size = resp_frame.len();
            let mut error = std::ptr::null_mut();

            ffi::hinawa_fw_fcp_avc_transaction(
                self.as_ref().to_glib_none().0,
                req_frame.as_ptr(),
                req_frame.len(),
                &mut resp_frame.as_mut_ptr(),
                &mut resp_frame_size,
                timeout_ms,
                &mut error,
            );

            if error.is_null() {
                Ok(resp_frame_size)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &[u8]) + 'static,
    {
        unsafe extern "C" fn responded_trampoline<P, F>(
            this: *mut ffi::HinawaFwFcp,
            frame: *const u8,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FwFcp>,
            F: Fn(&P, &[u8]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwFcp::from_glib_borrow(this).unsafe_cast_ref(),
                std::slice::from_raw_parts(frame, length as usize),
            )
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"responded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    responded_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
