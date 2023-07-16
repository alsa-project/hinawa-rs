// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwFcp`] methods.
///
/// # Implementors
///
/// [`FwFcp`][struct@crate::FwFcp]
pub trait FwFcpExtManual {
    /// Transfer command frame for FCP. When receiving response frame for FCP, `signal::FwFcp::responded`
    /// signal is emitted.
    ///
    /// Each value of @tstamp is unsigned 16 bit integer including higher 3 bits for three low order bits
    /// of second field and the rest 13 bits for cycle field in the format of IEEE 1394 CYCLE_TIMER register.
    ///
    /// If the version of kernel ABI for Linux FireWire subsystem is less than 6, each element of @tstamp
    /// has invalid value (=G_MAXUINT16).
    /// ## `cmd_frame`
    /// An array with elements for request byte data. The value of this
    ///  argument should point to the array and immutable.
    /// ## `timeout_ms`
    /// The timeout to wait for response subaction of transaction for command frame.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    ///
    /// ## `tstamp`
    /// The array with two elements for time stamps.
    ///     The first element is for the isochronous cycle at which the request arrived. The second
    ///     element is for the isochronous cycle at which the response was sent.
    #[doc(alias = "hinawa_fw_fcp_command_with_tstamp")]
    fn command_with_tstamp(
        &self,
        cmd_frame: &[u8],
        timeout_ms: u32,
    ) -> Result<[u32; 2], glib::Error>;

    /// Finish the pair of asynchronous transaction for AV/C command and response transactions. The
    /// timeout_ms parameter is used to wait for response transaction since the command transaction
    /// is initiated, ignoring [property@FwFcp:timeout] property of instance. The timeout is not
    /// expanded in the case that AV/C INTERIM status is arrived, thus the caller should expand the
    /// timeout in advance for the case.
    ///
    /// ## `cmd_frame`
    /// An array with elements for request byte data. The value of
    ///  this argument should point to the array and immutable.
    /// ## `resp_frame`
    /// An array with elements for response byte data. Callers
    ///   should give it for buffer with enough space against the request since this library
    ///   performs no reallocation. Due to the reason, the value of this argument should point to
    ///   the pointer to the array and immutable. The content of array is mutable.
    /// ## `timeout_ms`
    /// The timeout to wait for response transaction since command transactions finishes.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    ///
    /// ## `resp_size`
    /// The size of response buffer consumed for response frame.
    ///
    #[doc(alias = "hinawa_fw_fcp_avc_transaction")]
    fn avc_transaction(
        &self,
        cmd_frame: &[u8],
        resp_frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<usize, glib::Error>;

    /// Finish the pair of asynchronous transaction for AV/C command and response transactions. The
    /// timeout_ms parameter is used to wait for response transaction since the command transaction is
    /// initiated, ignoring `property::FwFcp::timeout` property of instance. The timeout is not expanded in
    /// the case that AV/C INTERIM status is arrived, thus the caller should expand the timeout in
    /// advance for the case.
    /// ## `cmd_frame`
    /// An array with elements for request byte data. The value of
    ///  this argument should point to the array and immutable.
    /// ## `resp_frame`
    /// An array with elements for response byte data. Callers
    ///   should give it for buffer with enough space against the request since this library
    ///   performs no reallocation. Due to the reason, the value of this argument should point to
    ///   the pointer to the array and immutable. The content of array is mutable.
    /// ## `timeout_ms`
    /// The timeout to wait for response transaction since command transactions finishes.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    ///
    /// ## `tstamp`
    /// The array with three elements for time
    ///     stamps. The first element is for the isochronous cycle at which the request was sent
    ///     for the command of FCP transaction. The second element is for the isochronous cycle at
    ///     which the response arrived for the command of FCP transaction. The third element is for
    ///     the isochronous cycle at which the request was sent for the response of FCP transaction.
    #[doc(alias = "hinawa_fw_fcp_avc_transaction_with_tstamp")]
    fn avc_transaction_with_tstamp(
        &self,
        cmd_frame: &[u8],
        resp_frame: &mut Vec<u8>,
        timeout_ms: u32,
    ) -> Result<[u32; 3], glib::Error>;

    /// Emitted when the node transfers asynchronous packet as response for FCP and the process
    /// successfully read the content of packet, except for the case that `signal@FwFcp::responded2`
    /// signal handler is already assigned.
    ///
    /// ## `frame`
    /// The array with elements for byte
    ///    data of response for FCP.
    #[doc(alias = "responded")]
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &[u8]) + 'static;

    /// Emitted when the node transfers asynchronous packet as response for FCP and the process
    /// successfully read the content of packet.
    ///
    /// The values of @tstamp is unsigned 16 bit integer including higher 3 bits for three low
    /// order bits of second field and the rest 13 bits for cycle field in the format of IEEE
    /// 1394 CYCLE_TIMER register.
    ///
    /// If the version of kernel ABI for Linux FireWire subsystem is less than 6, the value of
    /// @tstamp argument has invalid value (=G_MAXUINT).
    ///
    /// ## `tstamp`
    /// The time stamp at which the request arrived for the response of FCP
    ///     transaction.
    /// ## `frame`
    /// The array with elements for byte
    ///    data of response for FCP.
    #[doc(alias = "responded2")]
    fn connect_responded2<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, &[u8]) + 'static;
}

impl<O: IsA<FwFcp>> FwFcpExtManual for O {
    fn command_with_tstamp(
        &self,
        cmd_frame: &[u8],
        timeout_ms: u32,
    ) -> Result<[u32; 2], glib::Error> {
        unsafe {
            let mut tstamp = [0; 2];
            let mut error = std::ptr::null_mut();

            let _ = ffi::hinawa_fw_fcp_command_with_tstamp(
                self.as_ref().to_glib_none().0,
                cmd_frame.as_ptr(),
                cmd_frame.len(),
                &mut tstamp,
                timeout_ms,
                &mut error,
            );

            if error.is_null() {
                Ok(tstamp)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn avc_transaction(
        &self,
        cmd_frame: &[u8],
        resp_frame: &mut [u8],
        timeout_ms: u32,
    ) -> Result<usize, glib::Error> {
        unsafe {
            let mut resp_frame_size = resp_frame.len();
            let mut error = std::ptr::null_mut();

            ffi::hinawa_fw_fcp_avc_transaction(
                self.as_ref().to_glib_none().0,
                cmd_frame.as_ptr(),
                cmd_frame.len(),
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

    fn avc_transaction_with_tstamp(
        &self,
        cmd_frame: &[u8],
        resp_frame: &mut Vec<u8>,
        timeout_ms: u32,
    ) -> Result<[u32; 3], glib::Error> {
        unsafe {
            let mut resp_frame_size = resp_frame.len();
            let mut tstamp = [0; 3];
            let mut error = std::ptr::null_mut();

            let _ = ffi::hinawa_fw_fcp_avc_transaction_with_tstamp(
                self.as_ref().to_glib_none().0,
                cmd_frame.as_ptr(),
                cmd_frame.len(),
                &mut resp_frame.as_mut_ptr(),
                &mut resp_frame_size,
                &mut tstamp,
                timeout_ms,
                &mut error,
            );

            if error.is_null() {
                resp_frame.truncate(resp_frame_size);
                Ok(tstamp)
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

    fn connect_responded2<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, u32, &[u8]) + 'static,
    {
        unsafe extern "C" fn responded2_trampoline<P, F>(
            this: *mut ffi::HinawaFwFcp,
            tstamp: libc::c_uint,
            frame: *const u8,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FwFcp>,
            F: Fn(&P, u32, &[u8]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &FwFcp::from_glib_borrow(this).unsafe_cast_ref(),
                tstamp,
                std::slice::from_raw_parts(frame, length as usize),
            )
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"responded2\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    responded2_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
