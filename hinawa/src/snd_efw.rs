// SPDX-License-Identifier: MIT
use glib::object::IsA;
use glib::translate::*;

use SndEfw;

pub trait SndEfwExtManual {
    fn transaction(&self, category: u32, command: u32, args: &[u32],
                   params: &mut [u32]) -> Result<usize, glib::Error>;
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
}
