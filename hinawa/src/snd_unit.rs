// SPDX-License-Identifier: MIT
use crate::*;

pub trait SndUnitExtManual {
    fn open(&self, path: &str) -> Result<(), glib::Error>;
}

impl<O: IsA<SndUnit>> SndUnitExtManual for O {
    fn open(&self, path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::hinawa_snd_unit_open(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
