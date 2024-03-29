// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwResp`] methods.
///
/// # Implementors
///
/// [`FwResp`][struct@crate::FwResp]
pub trait FwRespExtManual {
    /// Emitted when any node transfers request subaction to local nodes within the address
    /// range reserved in Linux system.
    ///
    /// The handler is expected to call [`FwRespExt::set_resp_frame()`][crate::prelude::FwRespExt::set_resp_frame()] with frame and return
    /// [`FwRcode`][crate::FwRcode] for response subaction.
    ///
    /// The value of @tstamp is unsigned 16 bit integer including higher 3 bits for three low
    /// order bits of second field and the rest 13 bits for cycle field in the format of IEEE
    /// 1394 CYCLE_TIMER register.
    ///
    /// If the version of kernel ABI for Linux FireWire subsystem is less than 6, the value of
    /// tstamp argument has invalid value (=G_MAXUINT). Furthermore, if the version is less than
    /// 4, the src, dst, card, generation arguments have invalid value (=G_MAXUINT).
    /// ## `tcode`
    /// One of [`FwTcode`][crate::FwTcode] enumerations
    /// ## `offset`
    /// The address offset at which the transaction arrives.
    /// ## `src_node_id`
    /// The node ID of source for the transaction.
    /// ## `dst_node_id`
    /// The node ID of destination for the transaction.
    /// ## `card_id`
    /// The index of card specific to the 1394 OHCI hardware at which the request
    ///      subaction arrived.
    /// ## `generation`
    /// The generation of bus when the transaction is transferred.
    /// ## `tstamp`
    /// The isochronous cycle at which the request arrived.
    /// ## `frame`
    /// The array with elements for byte
    ///    data.
    ///
    /// # Returns
    ///
    /// One of [`FwRcode`][crate::FwRcode] enumerations corresponding to rcodes defined in IEEE 1394
    ///     specification.
    #[doc(alias = "requested")]
    fn connect_requested<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwTcode, u64, u32, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static;
}

impl<O: IsA<FwResp>> FwRespExtManual for O {
    fn connect_requested<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwTcode, u64, u32, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static,
    {
        unsafe extern "C" fn requested_trampoline<P, F>(
            this: *mut ffi::HinawaFwResp,
            tcode: ffi::HinawaFwTcode,
            offset: u64,
            src: libc::c_uint,
            dst: libc::c_uint,
            card: libc::c_uint,
            generation: libc::c_uint,
            tstamp: libc::c_uint,
            frame: *const u8,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) -> ffi::HinawaFwRcode
        where
            P: IsA<FwResp>,
            F: Fn(&P, FwTcode, u64, u32, u32, u32, u32, u32, &[u8]) -> FwRcode + 'static,
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
                tstamp,
                std::slice::from_raw_parts(frame, length as usize),
            )
            .into_glib()
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"requested\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    requested_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
