// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;
use glib::object::Cast;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;

use SndEfw;
use SndEfwStatus;

pub trait SndEfwExtManual {
    fn transaction(&self, category: u32, command: u32, args: &[u32],
                   params: &mut [u32]) -> Result<usize, glib::Error>;
    fn transaction_sync(&self, category: u32, command: u32, args: Option<&[u32]>,
                        params: Option<&mut [u32]>, timeout_ms: u32)
       -> Result<usize, glib::Error>;
    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
        where F: Fn(&Self, SndEfwStatus, u32, u32, u32, &[u32]) + 'static;
}

impl<O: IsA<SndEfw>> SndEfwExtManual for O {
    fn transaction(&self, category: u32, command: u32, args: &[u32],
                   params: &mut [u32])
       -> Result<usize, glib::Error> {
        unsafe {
            let mut param_count = params.len();
            let mut error = std::ptr::null_mut();

            hinawa_sys::hinawa_snd_efw_transaction(
                self.as_ref().to_glib_none().0,
                category,
                command,
                args.as_ptr(),
                args.len() as usize,
                &mut params.as_mut_ptr(),
                &mut param_count,
                &mut error);

            if error.is_null() {
                Ok(param_count)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn transaction_sync(&self, category: u32, command: u32, args: Option<&[u32]>,
                        params: Option<&mut [u32]>, timeout_ms: u32)
       -> Result<usize, glib::Error> {
        unsafe {
            let (arg_ptr, arg_count) = match args {
                Some(a) => (a.as_ptr(), a.len()),
                None => (std::ptr::null(), 0),
            };
            let (mut param_ptr, mut param_count) = match params {
                Some(p) => (p.as_mut_ptr(), p.len()),
                None => (std::ptr::null_mut(), 0),
            };
            let mut error = std::ptr::null_mut();

            hinawa_sys::hinawa_snd_efw_transaction_sync(
                self.as_ref().to_glib_none().0,
                category,
                command,
                arg_ptr,
                arg_count,
                &mut param_ptr,
                &mut param_count,
                timeout_ms,
                &mut error);

            if error.is_null() {
                Ok(param_count)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_responded<F>(&self, f: F) -> SignalHandlerId
        where F: Fn(&Self, SndEfwStatus, u32, u32, u32, &[u32]) + 'static
    {
        unsafe extern "C" fn responded_trampoline<P, F>(
            this: *mut hinawa_sys::HinawaSndEfw,
            status: hinawa_sys::HinawaSndEfwStatus,
            seqnum: u32,
            command: u32,
            category: u32,
            frame: *const u32,
            length: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SndEfw>,
            F: Fn(&P, SndEfwStatus, u32, u32, u32, &[u32]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &SndEfw::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(status),
                seqnum,
                command,
                category,
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
