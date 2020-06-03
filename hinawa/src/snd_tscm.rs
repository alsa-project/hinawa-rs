// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;

use SndTscm;

pub trait SndTscmExtManual {
    fn get_state(&self) -> Result<&[u32;64], glib::Error>;
}

impl<O: IsA<SndTscm>> SndTscmExtManual for O {
    fn get_state(&self)
        -> Result<&[u32;64], glib::Error>
    {
        unsafe {
            let mut error = std::ptr::null_mut();

            let image = hinawa_sys::hinawa_snd_tscm_get_state(
                self.as_ref().to_glib_none().0,
                &mut error);

            if error.is_null() {
                Ok(&*image)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
