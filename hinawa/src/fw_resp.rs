// SPDX-License-Identifier: MIT
use crate::*;

pub trait FwRespExtManual {
    #[doc(alias = "requested")]
    fn connect_requested2<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwTcode, u64, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static;
}

impl<O: IsA<FwResp>> FwRespExtManual for O {
    fn connect_requested2<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwTcode, u64, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static,
    {
        unsafe extern "C" fn requested2_trampoline<P, F>(
            this: *mut ffi::HinawaFwResp,
            tcode: ffi::HinawaFwTcode,
            offset: u64,
            src: u32,
            dst: u32,
            card: u32,
            generation: u32,
            frame: *const u8,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) -> ffi::HinawaFwRcode
        where
            P: IsA<FwResp>,
            F: Fn(&P, FwTcode, u64, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static,
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
            )
            .into_glib()
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"requested2\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    requested2_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
