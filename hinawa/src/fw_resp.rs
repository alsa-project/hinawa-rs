// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwResp`] methods.
///
/// # Implementors
///
/// [`FwResp`][struct@crate::FwResp]
pub trait FwRespExtManual {
    /// Emitted when any node transfers request subaction to the range of address to which this
    /// object listening, the `signal::FwResp::requested` signal handler is called with arrived
    /// frame for the subaction. The handler is expected to call [`FwRespExt::set_resp_frame()`][crate::prelude::FwRespExt::set_resp_frame()]
    /// with frame and return [`FwRcode`][crate::FwRcode] for response subaction.
    /// ## `tcode`
    /// One of [`FwTcode`][crate::FwTcode] enumerations
    /// ## `offset`
    /// The address offset at which the transaction arrives.
    /// ## `src`
    /// The node ID of source for the transaction.
    /// ## `dst`
    /// The node ID of destination for the transaction.
    /// ## `card`
    /// The index of card corresponding to 1394 OHCI controller.
    /// ## `generation`
    /// The generation of bus when the transaction is transferred.
    /// ## `frame`
    /// The array with elements for byte
    ///    data.
    ///
    /// # Returns
    ///
    /// One of [`FwRcode`][crate::FwRcode] enumerations corresponding to rcodes defined in IEEE 1394
    ///     specification.
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
