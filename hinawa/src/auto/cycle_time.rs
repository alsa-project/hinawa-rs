// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::mem;

glib::wrapper! {
    /// A boxed object to express data of cycle time.
    ///
    /// A [`CycleTime`][crate::CycleTime] expresses the value of cycle time of 1394 OHCI as well as Linux system
    /// time referring to clock_id.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CycleTime(Boxed<ffi::HinawaCycleTime>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::hinawa_cycle_time_get_type(), ptr as *mut _) as *mut ffi::HinawaCycleTime,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::hinawa_cycle_time_get_type(), ptr as *mut _),
        type_ => || ffi::hinawa_cycle_time_get_type(),
    }
}

impl CycleTime {
    /// Allocate and return an instance of [`CycleTime`][crate::CycleTime].
    ///
    /// # Returns
    ///
    /// An instance of [`CycleTime`][crate::CycleTime].
    #[doc(alias = "hinawa_cycle_time_new")]
    pub fn new() -> CycleTime {
        unsafe { from_glib_full(ffi::hinawa_cycle_time_new()) }
    }

    /// Get the ID of clock for timestamp.
    ///
    /// # Returns
    ///
    ///
    /// ## `clock_id`
    /// The numeric ID of clock source for the reference timestamp.
    ///       One of CLOCK_REALTIME(0), CLOCK_MONOTONIC(1), and CLOCK_MONOTONIC_RAW(4) is available
    ///       UAPI of Linux kernel.
    #[doc(alias = "hinawa_cycle_time_get_clock_id")]
    #[doc(alias = "get_clock_id")]
    pub fn clock_id(&self) -> i32 {
        unsafe {
            let mut clock_id = mem::MaybeUninit::uninit();
            ffi::hinawa_cycle_time_get_clock_id(self.to_glib_none().0, clock_id.as_mut_ptr());
            let clock_id = clock_id.assume_init();
            clock_id
        }
    }

    /// Get the value of cycle time in 1394 OHCI hardware.
    ///
    /// # Returns
    ///
    ///
    /// ## `raw`
    /// The raw value for CYCLE_TIME register.
    #[doc(alias = "hinawa_cycle_time_get_raw")]
    #[doc(alias = "get_raw")]
    pub fn raw(&self) -> u32 {
        unsafe {
            let mut raw = mem::MaybeUninit::uninit();
            ffi::hinawa_cycle_time_get_raw(self.to_glib_none().0, raw.as_mut_ptr());
            let raw = raw.assume_init();
            raw
        }
    }

    /// Get system time with enough size of strorage. The timestamp refers to clock_id available by
    /// [`clock_id()`][Self::clock_id()].
    ///
    /// # Returns
    ///
    ///
    /// ## `tv_sec`
    /// The second part of timestamp.
    ///
    /// ## `tv_nsec`
    /// The nanosecond part of timestamp.
    #[doc(alias = "hinawa_cycle_time_get_system_time")]
    #[doc(alias = "get_system_time")]
    pub fn system_time(&self) -> (i64, i32) {
        unsafe {
            let mut tv_sec = mem::MaybeUninit::uninit();
            let mut tv_nsec = mem::MaybeUninit::uninit();
            ffi::hinawa_cycle_time_get_system_time(
                self.to_glib_none().0,
                tv_sec.as_mut_ptr(),
                tv_nsec.as_mut_ptr(),
            );
            let tv_sec = tv_sec.assume_init();
            let tv_nsec = tv_nsec.assume_init();
            (tv_sec, tv_nsec)
        }
    }
}

impl Default for CycleTime {
    fn default() -> Self {
        Self::new()
    }
}