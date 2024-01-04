// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing all [`struct@FwReq`] methods.
///
/// # Implementors
///
/// [`FwReq`][struct@crate::FwReq]
pub trait FwReqExtManual {
    /// Execute request subaction of transaction to the given node according to given code. When the
    /// response subaction arrives and running event dispatcher reads the contents,
    /// [`responded`][struct@crate::FwReq#responded] signal handler is called.
    /// ## `node`
    /// A [`FwNode`][crate::FwNode].
    /// ## `tcode`
    /// A transaction code of [`FwTcode`][crate::FwTcode].
    /// ## `addr`
    /// A destination address of target device
    /// ## `length`
    /// The range of address in byte unit.
    /// ## `frame`
    /// An array with elements for byte data. Callers should
    ///    give it for buffer with enough space against the request since this library performs no
    ///    reallocation. Due to the reason, the value of this argument should point to the pointer
    ///    to the array and immutable. The content of array is mutable for read and lock
    ///    transaction.
    #[doc(alias = "hinawa_fw_req_request")]
    fn request<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
    ) -> Result<(), glib::Error>;

    /// Execute request subaction of transaction to the given node according to given code, then wait
    /// for response subaction within the given timeout.
    ///
    /// Each value of @tstamp is unsigned 16 bit integer including higher 3 bits for three low order bits
    /// of second field and the rest 13 bits for cycle field in the format of IEEE 1394 CYCLE_TIMER register.
    ///
    /// If the version of kernel ABI for Linux FireWire subsystem is less than 6, each element of @tstamp
    /// has invalid value (=G_MAXUINT).
    /// ## `node`
    /// A [`FwNode`][crate::FwNode].
    /// ## `tcode`
    /// A transaction code of [`FwTcode`][crate::FwTcode].
    /// ## `addr`
    /// A destination address of target device
    /// ## `length`
    /// The range of address in byte unit.
    /// ## `frame`
    /// An array with elements for byte data. Callers should
    ///    give it for buffer with enough space against the request since this library performs no
    ///    reallocation. Due to the reason, the value of this argument should point to the pointer
    ///    to the array and immutable. The content of array is mutable for read and lock
    ///    transaction.
    /// ## `timeout_ms`
    /// The timeout to wait for the response subaction of transaction since the request
    ///     subaction is initiated, in milliseconds.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    ///
    /// ## `tstamp`
    /// The array with two elements for time stamps.
    ///     The first element is for the isochronous cycle at which the request subaction was sent.
    ///     The second element is for the isochronous cycle at which the response subaction arrived.
    #[doc(alias = "hinawa_fw_req_transaction_with_tstamp")]
    fn transaction_with_tstamp<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<[u32; 2], glib::Error>;

    /// Execute request subaction of transaction to the given node according to given code, then wait
    /// for response subaction within the value of timeout argument. The function is a thin wrapper to
    /// [`FwReqExtManual::transaction_with_tstamp()`][crate::prelude::FwReqExtManual::transaction_with_tstamp()].
    /// ## `node`
    /// A [`FwNode`][crate::FwNode].
    /// ## `tcode`
    /// A transaction code of [`FwTcode`][crate::FwTcode].
    /// ## `addr`
    /// A destination address of target device
    /// ## `length`
    /// The range of address in byte unit.
    /// ## `frame`
    /// An array with elements for byte data. Callers should
    ///    give it for buffer with enough space against the request since this library performs no
    ///    reallocation. Due to the reason, the value of this argument should point to the pointer
    ///    to the array and immutable. The content of array is mutable for read and lock
    ///    transaction.
    /// ## `timeout_ms`
    /// The timeout to wait for response subaction of the transaction since request
    ///     subaction is initiated, in milliseconds.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinawa_fw_req_transaction")]
    fn transaction<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<(), glib::Error>;

    /// Emitted when the node transfers asynchronous packet as response subaction for the
    /// transaction and the process successfully reads the content of packet from Linux firewire
    /// subsystem.
    ///
    /// The values of @request_tstamp and @response_tstamp are unsigned 16 bit integer including
    /// higher 3 bits for three low order bits of second field and the rest 13 bits for cycle
    /// field in the format of IEEE 1394 CYCLE_TIMER register.
    ///
    /// If the version of kernel ABI for Linux FireWire subsystem is less than 6, the
    /// @request_tstamp and @response_tstamp argument has invalid value (=G_MAXUINT).
    ///
    /// ## `rcode`
    /// One of [`FwRcode`][crate::FwRcode].
    /// ## `request_tstamp`
    /// The isochronous cycle at which the request was sent.
    /// ## `response_tstamp`
    /// The isochronous cycle at which the response arrived.
    /// ## `frame`
    /// The array with elements for byte data of response subaction for transaction.
    #[doc(alias = "responded")]
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwRcode, u32, u32, &[u8]) + 'static;
}

impl<O: IsA<FwReq>> FwReqExtManual for O {
    fn request<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut frame_size = frame.len();
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hinawa_fw_req_request(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                tcode.into_glib(),
                addr,
                length,
                &mut frame.as_mut_ptr(),
                &mut frame_size,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn transaction_with_tstamp<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<[u32; 2], glib::Error> {
        unsafe {
            let mut frame_size = frame.len();
            let mut tstamp = [0; 2];
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hinawa_fw_req_transaction_with_tstamp(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                tcode.into_glib(),
                addr,
                length,
                &mut frame.as_mut_ptr(),
                &mut frame_size,
                &mut tstamp,
                timeout_ms,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(tstamp)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn transaction<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut frame_size = frame.len();
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::hinawa_fw_req_transaction(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                tcode.into_glib(),
                addr,
                length,
                &mut frame.as_mut_ptr(),
                &mut frame_size,
                timeout_ms,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwRcode, u32, u32, &[u8]) + 'static,
    {
        unsafe extern "C" fn responded_trampoline<P, F>(
            this: *mut ffi::HinawaFwReq,
            rcode: ffi::HinawaFwRcode,
            request_tstamp: libc::c_uint,
            response_tstamp: libc::c_uint,
            frame: *const u8,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FwReq>,
            F: Fn(&P, FwRcode, u32, u32, &[u8]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwReq::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(rcode),
                request_tstamp,
                response_tstamp,
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
