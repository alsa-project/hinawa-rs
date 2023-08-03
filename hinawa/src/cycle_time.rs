// SPDX-License-Identifier: MIT
use crate::*;

impl CycleTime {
    /// Get the value of cycle time in 1394 OHCI hardware. The first element of array expresses the
    /// value of sec field, up to 127. The second element of array expresses the value of cycle field,
    /// up to 7999. The third element of array expresses the value of offset field, up to 3071.
    ///
    /// # Returns
    ///
    ///
    /// ## `fields`
    /// The value of cycle time register of 1394
    ///     OHCI hardware, including three elements; second, cycle, and offset in its order.
    #[doc(alias = "hinawa_cycle_time_get_fields")]
    pub fn fields(&mut self) -> [u16; 3] {
        let mut fields = [0; 3];
        unsafe {
            ffi::hinawa_cycle_time_get_fields(self.to_glib_none_mut().0, &mut fields);
        }
        fields
    }

    /// Compute second count and cycle count from unsigned 16 bit integer value retrieved by Asynchronous
    /// Transmit (AT), Asynchronous Receive(AR), Isochronous Transmit (IT), and Isochronous Receive (IR)
    /// contexts of 1394 OHCI. The second count is completed with the internal value read from the
    /// CYCLE_TIME register. For the precise computation, the method should be called in the condition
    /// that the timing between receipt of time stamp and access to CYCLE_TIME register is within 8
    /// seconds.
    /// ## `tstamp`
    /// The value of time stamp retrieved from each context of 1394 OHCI.
    ///
    /// # Returns
    ///
    ///
    /// ## `isoc_cycle`
    /// The result to parse the time stamp. The
    ///     first element is for 7 bits of second field in the format of IEEE 1394 CYCLE_TIME
    ///     register, up to 127. The second element is for 13 bits of cycle field in the format,
    ///     up to 7,999.
    #[doc(alias = "hinawa_cycle_time_compute_tstamp")]
    pub fn compute_tstamp(&self, tstamp: u32) -> [u32; 2] {
        let mut isoc_cycle = [0; 2];
        unsafe {
            ffi::hinawa_cycle_time_compute_tstamp(self.to_glib_none().0, tstamp, &mut isoc_cycle);
        }
        isoc_cycle
    }

    /// Parse second count and cycle count from unsigned 16 bit integer value retrieved by Asynchronous
    /// Transmit (AT), Asynchronous Receive(AR), Isochronous Transmit (IT), and Isochronous Receive (IR)
    /// contexts of 1394 OHCI.
    /// ## `tstamp`
    /// The value of time stamp retrieved from each context of 1394 OHCI.
    ///
    /// # Returns
    ///
    ///
    /// ## `isoc_cycle`
    /// The result to parse the time stamp. The
    ///     first element is for three order bits of second field in the format of IEEE 1394
    ///     CYCLE_TIME register, up to 7. The second element is for 13 bits of cycle field in
    ///     the format, up to 7,999.
    #[doc(alias = "hinawa_cycle_time_parse_tstamp")]
    pub fn parse_tstamp(tstamp: u32) -> [u32; 2] {
        let mut isoc_cycle = [0; 2];
        unsafe {
            ffi::hinawa_cycle_time_parse_tstamp(tstamp, &mut isoc_cycle);
        }
        isoc_cycle
    }
}
