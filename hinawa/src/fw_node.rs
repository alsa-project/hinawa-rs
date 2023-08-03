// SPDX-License-Identifier: MIT
use crate::*;

/// Trait containing the rest of [`struct@FwNode`] methods.
///
/// # Implementors
///
/// [`FwNode`][struct@crate::FwNode]
pub trait FwNodeExtManual {
    /// Get cached content of configuration ROM aligned to big-endian.
    ///
    /// # Returns
    ///
    /// ## `image`
    /// The content of configuration ROM.
    #[doc(alias = "hinawa_fw_node_get_config_rom")]
    #[doc(alias = "get_config_rom")]
    fn config_rom(&self) -> Result<&[u8], glib::Error>;

    /// Read current value of CYCLE_TIME register in 1394 OHCI hardware dedicated to communicate with
    /// the associated node in IEEE 1394 bus.
    /// ## `clock_id`
    /// The numeric ID of clock source for the reference timestamp. One of CLOCK_REALTIME(0),
    ///       CLOCK_MONOTONIC(1), and CLOCK_MONOTONIC_RAW(4) is available in UAPI of Linux kernel.
    /// ## `cycle_time`
    /// A [`CycleTime`][crate::CycleTime].
    #[doc(alias = "hinawa_fw_node_read_cycle_time")]
    fn read_cycle_time(&self, clock_id: i32, cycle_time: &mut CycleTime)
        -> Result<(), glib::Error>;
}

impl<O: IsA<FwNode>> FwNodeExtManual for O {
    fn config_rom(&self) -> Result<&[u8], glib::Error> {
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

    fn read_cycle_time(
        &self,
        clock_id: i32,
        cycle_time: &mut CycleTime,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::hinawa_fw_node_read_cycle_time(
                self.as_ref().to_glib_none().0,
                clock_id,
                &mut cycle_time.to_glib_none_mut().0,
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
