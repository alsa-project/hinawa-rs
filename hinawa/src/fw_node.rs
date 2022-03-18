// SPDX-License-Identifier: MIT
use crate::*;

pub trait FwNodeExtManual {
    fn get_config_rom(&self) -> Result<&[u8], glib::Error>;
}

impl<O: IsA<FwNode>> FwNodeExtManual for O {
    fn get_config_rom(&self) -> Result<&[u8], glib::Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const u8;
            let mut len = 0 as usize;
            let mut error = std::ptr::null_mut();

            ffi::hinawa_fw_node_get_config_rom(
                self.as_ref().to_glib_none().0,
                &mut ptr,
                &mut len,
                &mut error,
            );

            if error.is_null() {
                Ok(std::slice::from_raw_parts(ptr, len))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
