// SPDX-License-Identifier: MIT
use crate::*;

pub trait SndTscmExtManual {
    fn get_state(&self) -> Result<&[u32; 64], glib::Error>;
}

impl<O: IsA<SndTscm>> SndTscmExtManual for O {
    fn get_state(&self) -> Result<&[u32; 64], glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let image = ffi::hinawa_snd_tscm_get_state(self.as_ref().to_glib_none().0, &mut error);

            if error.is_null() {
                Ok(&*image)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
