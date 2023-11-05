// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    /// A transaction executor to a node in IEEE 1394 bus.
    ///
    /// [`FwReq`][crate::FwReq] supports all types of transactions defiend in IEEE 1212.
    ///
    /// # Implements
    ///
    /// [`FwReqExtManual`][trait@crate::prelude::FwReqExtManual]
    #[doc(alias = "HinawaFwReq")]
    pub struct FwReq(Object<ffi::HinawaFwReq, ffi::HinawaFwReqClass>);

    match fn {
        type_ => || ffi::hinawa_fw_req_get_type(),
    }
}

impl FwReq {
    pub const NONE: Option<&'static FwReq> = None;

    /// Instantiate [`FwReq`][crate::FwReq] object and return the instance.
    ///
    /// # Returns
    ///
    /// an instance of [`FwReq`][crate::FwReq].
    #[doc(alias = "hinawa_fw_req_new")]
    pub fn new() -> FwReq {
        unsafe { from_glib_full(ffi::hinawa_fw_req_new()) }
    }
}

impl Default for FwReq {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FwReq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FwReq")
    }
}
