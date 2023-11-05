// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FwNode;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    /// A transaction responder for request subaction initiated by node in IEEE 1394 bus.
    ///
    /// [`FwResp`][crate::FwResp] responds to request subaction initiated by node in IEEE 1394 bus.
    ///
    /// # Implements
    ///
    /// [`FwRespExt`][trait@crate::prelude::FwRespExt], [`FwRespExtManual`][trait@crate::prelude::FwRespExtManual]
    #[doc(alias = "HinawaFwResp")]
    pub struct FwResp(Object<ffi::HinawaFwResp, ffi::HinawaFwRespClass>);

    match fn {
        type_ => || ffi::hinawa_fw_resp_get_type(),
    }
}

impl FwResp {
    pub const NONE: Option<&'static FwResp> = None;

    /// Instantiate [`FwResp`][crate::FwResp] object and return the instance.
    ///
    /// # Returns
    ///
    /// a new instance of [`FwResp`][crate::FwResp].
    #[doc(alias = "hinawa_fw_resp_new")]
    pub fn new() -> FwResp {
        unsafe { from_glib_full(ffi::hinawa_fw_resp_new()) }
    }
}

impl Default for FwResp {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing the part of [`struct@FwResp`] methods.
///
/// # Implementors
///
/// [`FwFcp`][struct@crate::FwFcp], [`FwResp`][struct@crate::FwResp]
pub trait FwRespExt: 'static {
    /// Stop listening to the address range in Linux system for local nodes.
    #[doc(alias = "hinawa_fw_resp_release")]
    fn release(&self);

    /// Allocate an address range within Linux system for local nodes, each of which expresses 1394
    /// OHCI hardware. Once successful, `signal::FwResp::requested` signal will be emitted whenever any
    /// request subactions arrive at the 1394 OHCI hardware within the dedicated range.
    ///
    /// The range is precisely reserved at the address specified by @addr with the size indicated by
    /// @width. In essence, this function is a variant of [`reserve_within_region()`][Self::reserve_within_region()] in
    /// which the specified address range is reserved as provided.
    /// ## `node`
    /// A [`FwNode`][crate::FwNode].
    /// ## `addr`
    /// A start address to listen to in 1394 OHCI hardware.
    /// ## `width`
    /// The byte width of address to listen to 1394 OHCI hardware.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinawa_fw_resp_reserve")]
    fn reserve(&self, node: &impl IsA<FwNode>, addr: u64, width: u32) -> Result<(), glib::Error>;

    /// Allocate an address range within Linux system for local nodes, each of which expresses 1394
    /// OHCI hardware. Once successful, `signal::FwResp::requested` signal will be emitted whenever any
    /// request subactions arrive at the 1394 OHCI hardware within the dedicated range.
    ///
    /// The range is reserved between the values specified by @region_start and @region_end with the size
    /// indicated by @width. The starting offset may vary every time.
    /// ## `node`
    /// A [`FwNode`][crate::FwNode].
    /// ## `region_start`
    /// Start offset of address region in which range of address is looked up.
    /// ## `region_end`
    /// End offset of address region in which range of address is looked up.
    /// ## `width`
    /// The width for range of address to be looked up.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinawa_fw_resp_reserve_within_region")]
    fn reserve_within_region(
        &self,
        node: &impl IsA<FwNode>,
        region_start: u64,
        region_end: u64,
        width: u32,
    ) -> Result<(), glib::Error>;

    /// Register byte frame for the response subaction of transaction.
    /// ## `frame`
    /// a 8 bit array for response frame.
    #[doc(alias = "hinawa_fw_resp_set_resp_frame")]
    fn set_resp_frame(&self, frame: &[u8]);

    /// Whether a range of address is reserved or not.
    #[doc(alias = "is-reserved")]
    fn is_reserved(&self) -> bool;

    /// The start offset of reserved address range.
    fn offset(&self) -> u64;

    /// The width of reserved address range.
    fn width(&self) -> u32;

    #[doc(alias = "is-reserved")]
    fn connect_is_reserved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "offset")]
    fn connect_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FwResp>> FwRespExt for O {
    fn release(&self) {
        unsafe {
            ffi::hinawa_fw_resp_release(self.as_ref().to_glib_none().0);
        }
    }

    fn reserve(&self, node: &impl IsA<FwNode>, addr: u64, width: u32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::hinawa_fw_resp_reserve(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                addr,
                width,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn reserve_within_region(
        &self,
        node: &impl IsA<FwNode>,
        region_start: u64,
        region_end: u64,
        width: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::hinawa_fw_resp_reserve_within_region(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                region_start,
                region_end,
                width,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_resp_frame(&self, frame: &[u8]) {
        let length = frame.len() as usize;
        unsafe {
            ffi::hinawa_fw_resp_set_resp_frame(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
                length,
            );
        }
    }

    fn is_reserved(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-reserved")
    }

    fn offset(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "offset")
    }

    fn width(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "width")
    }

    fn connect_is_reserved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_reserved_trampoline<P: IsA<FwResp>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwResp,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwResp::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-reserved\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_reserved_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<P: IsA<FwResp>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwResp,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwResp::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P: IsA<FwResp>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwResp,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwResp::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FwResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FwResp")
    }
}
