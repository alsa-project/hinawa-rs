// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use hinawa_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use SndUnit;

glib_wrapper! {
    pub struct SndDg00x(Object<hinawa_sys::HinawaSndDg00x, hinawa_sys::HinawaSndDg00xClass, SndDg00xClass>) @extends SndUnit;

    match fn {
        get_type => || hinawa_sys::hinawa_snd_dg00x_get_type(),
    }
}

impl SndDg00x {
    pub fn new() -> SndDg00x {
        unsafe {
            from_glib_full(hinawa_sys::hinawa_snd_dg00x_new())
        }
    }
}

impl Default for SndDg00x {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SND_DG00X: Option<&SndDg00x> = None;

pub trait SndDg00xExt: 'static {
    fn open(&self, path: &str) -> Result<(), glib::Error>;

    fn connect_message<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SndDg00x>> SndDg00xExt for O {
    fn open(&self, path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = hinawa_sys::hinawa_snd_dg00x_open(self.as_ref().to_glib_none().0, path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_message<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn message_trampoline<P, F: Fn(&P, u32) + 'static>(this: *mut hinawa_sys::HinawaSndDg00x, message: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<SndDg00x>
        {
            let f: &F = &*(f as *const F);
            f(&SndDg00x::from_glib_borrow(this).unsafe_cast_ref(), message)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"message\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(message_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SndDg00x {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SndDg00x")
    }
}
