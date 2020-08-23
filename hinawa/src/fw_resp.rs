// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;
use glib::object::Cast;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;

use FwResp;
use FwTcode;
use FwRcode;

pub trait FwRespExtManual {
    fn get_req_frames(&self) -> &[u8];
    fn connect_requested2<F>(&self, f: F) -> SignalHandlerId
        where F: Fn(&Self, FwTcode, u64, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static;
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

    fn connect_requested2<F>(&self, f: F) -> SignalHandlerId
        where F: Fn(&Self, FwTcode, u64, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static
    {
        unsafe extern "C" fn requested_trampoline<P, F>(
            this: *mut hinawa_sys::HinawaFwResp,
            tcode: hinawa_sys::HinawaFwTcode,
            offset: u64,
            src: u32,
            dst: u32,
            card: u32,
            generation: u32,
            frame: *const u8,
            length: libc::c_uint,
            f: glib_sys::gpointer,
        ) -> hinawa_sys::HinawaFwRcode
            where P: IsA<FwResp>,
                  F: Fn(&P, FwTcode, u64, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static
        {
            let f: &F = &*(f as *const F);
            f(
                &FwResp::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(tcode),
                offset,
                src,
                dst,
                card,
                generation,
                std::slice::from_raw_parts(frame, length as usize),
            ).to_glib()
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"requested2\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                        requested_trampoline::<Self, F> as *const ()
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
