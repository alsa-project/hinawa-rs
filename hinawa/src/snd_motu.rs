// SPDX-License-Identifier: MIT
use crate::*;

pub trait SndMotuExtManual {
    fn read_register_dsp_parameter(
        &self,
        param: &mut SndMotuRegisterDspParameter,
    ) -> Result<(), glib::Error>;
    fn read_register_dsp_meter(&self, meter: &mut [u8; 48]) -> Result<(), glib::Error>;
    fn read_command_dsp_meter(&self, meter: &mut [f32; 400]) -> Result<(), glib::Error>;
    fn connect_register_dsp_changed<F: Fn(&Self, &[u32]) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<SndMotu>> SndMotuExtManual for O {
    fn read_register_dsp_parameter(
        &self,
        param: &mut SndMotuRegisterDspParameter,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            ffi::hinawa_snd_motu_read_register_dsp_parameter(
                self.as_ref().to_glib_none().0,
                &param.to_glib_none_mut().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_register_dsp_meter(&self, meter: &mut [u8; 48]) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *mut [u8; 48] = meter;
            let mut error = std::ptr::null_mut();

            ffi::hinawa_snd_motu_read_register_dsp_meter(
                self.as_ref().to_glib_none().0,
                &ptr,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_command_dsp_meter(&self, meter: &mut [f32; 400]) -> Result<(), glib::Error> {
        unsafe {
            let ptr: *mut [f32; 400] = meter;
            let mut error = std::ptr::null_mut();

            ffi::hinawa_snd_motu_read_command_dsp_meter(
                self.as_ref().to_glib_none().0,
                &ptr,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_register_dsp_changed<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &[u32]) + 'static,
    {
        unsafe extern "C" fn register_dsp_changed_trampoline<P, F>(
            this: *mut ffi::HinawaSndMotu,
            events: *const u32,
            length: libc::c_uint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SndMotu>,
            F: Fn(&P, &[u32]) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(
                &SndMotu::from_glib_borrow(this).unsafe_cast_ref(),
                std::slice::from_raw_parts(events, length as usize),
            )
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"register-dsp-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    register_dsp_changed_trampoline::<Self, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
            )
        }
    }
}
