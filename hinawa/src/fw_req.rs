// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;
use glib::object::Cast;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;

use FwReq;
use FwTcode;
use FwNode;
use FwRcode;

pub trait FwReqExtManual {
    fn transaction_async<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                         addr: u64, length: usize, frame: &mut [u8])
        ->Result<(), glib::Error>;
    fn transaction_sync<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                        addr: u64, length: usize, frame: &mut [u8],
                                        timeout_ms: u32)
        ->Result<(), glib::Error>;
    fn transaction<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                   addr: u64, length: usize, frame: &mut [u8])
        ->Result<(), glib::Error>;
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
        where F: Fn(&Self, FwRcode, &[u8]) + 'static;
}

impl<O: IsA<FwReq>> FwReqExtManual for O {
    fn transaction_async<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                         addr: u64, length: usize, frame: &mut [u8])
        ->Result<(), glib::Error>
    {
        unsafe {
            let mut frame_size = frame.len();
            let mut error = std::ptr::null_mut();

            hinawa_sys::hinawa_fw_req_transaction_async(
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

    fn transaction_sync<P: IsA<FwNode>>(&self, node: &P, tcode: FwTcode,
                                        addr: u64, length: usize, frame: &mut [u8],
                                        timeout_ms: u32)
        ->Result<(), glib::Error>
    {
        unsafe {
            let mut frame_size = frame.len();
            let mut error = std::ptr::null_mut();

            hinawa_sys::hinawa_fw_req_transaction_sync(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                tcode.to_glib(), addr, length,
                &mut frame.as_mut_ptr(), &mut frame_size,
                timeout_ms, &mut error);

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

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

    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
        where F: Fn(&Self, FwRcode, &[u8]) + 'static
    {
        unsafe extern "C" fn responded_trampoline<P, F>(
            this: *mut hinawa_sys::HinawaFwReq,
            rcode: hinawa_sys::HinawaFwRcode,
            frame: *const u8,
            length: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FwReq>,
            F: Fn(&P, FwRcode, &[u8]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwReq::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(rcode),
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
