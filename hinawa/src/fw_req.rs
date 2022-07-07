// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing all [`struct@FwReq`] methods.
///
/// # Implementors
///
/// [`FwReq`][struct@crate::FwReq]
pub trait FwReqExtManual {
    /// Execute request subaction of transactions to the given node according to given code. When the
    /// response subaction arrives and read the contents, `signal::FwReq::responded` signal handler is called
    /// as long as event dispatcher runs.
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
    #[doc(alias = "hinawa_fw_req_transaction_async")]
    fn transaction_async<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
    ) -> Result<(), glib::Error>;
    /// Execute request subaction of transaction to the given node according to given code, then wait
    /// for response subaction within the given timeout. The `property::FwReq::timeout` property of
    /// instance is ignored.
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
    #[doc(alias = "hinawa_fw_req_transaction_sync")]
    fn transaction_sync<P: IsA<FwNode>>(
        &self,
        node: &P,
        tcode: FwTcode,
        addr: u64,
        length: usize,
        frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<(), glib::Error>;
    /// Emitted when the unit transfers asynchronous packet as response subaction for the
    /// transaction and the process successfully reads the content of packet from Linux firewire
    /// subsystem.
    /// ## `rcode`
    /// One of [`FwRcode`][crate::FwRcode].
    /// ## `frame`
    /// The array with elements for
    ///    byte data of response subaction for transaction.
    #[doc(alias = "responded")]
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwRcode, &[u8]) + 'static;
}

impl<O: IsA<FwReq>> FwReqExtManual for O {
    fn transaction_async<P: IsA<FwNode>>(
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

            ffi::hinawa_fw_req_transaction_async(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                tcode.into_glib(),
                addr,
                length,
                &mut frame.as_mut_ptr(),
                &mut frame_size,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn transaction_sync<P: IsA<FwNode>>(
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

            ffi::hinawa_fw_req_transaction_sync(
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

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, FwRcode, &[u8]) + 'static,
    {
        unsafe extern "C" fn responded_trampoline<P, F>(
            this: *mut ffi::HinawaFwReq,
            rcode: ffi::HinawaFwRcode,
            frame: *const u8,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
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
