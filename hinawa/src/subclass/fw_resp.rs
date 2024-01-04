// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwResp`][crate::FwResp].
pub trait FwRespImpl: ObjectImpl {
    /// Class closure for the [`requested`][struct@crate::FwResp#requested] signal.
    /// ## `tcode`
    /// One of [`FwTcode`][crate::FwTcode] enumerations
    /// ## `offset`
    /// The address offset at which the transaction arrives.
    /// ## `src_node_id`
    /// The node ID of source for the transaction.
    /// ## `dst_node_id`
    /// The node ID of destination for the transaction.
    /// ## `card_id`
    /// The index of card specific to 1394 OHCI hardware at which the request
    ///      subaction arrived.
    /// ## `generation`
    /// The generation of bus when the transaction is transferred.
    /// ## `tstamp`
    /// The time stamp at which the request arrived.
    /// ## `frame`
    /// The array with elements for byte
    ///    data.
    ///
    /// # Returns
    ///
    /// One of [`FwRcode`][crate::FwRcode] enumerations corresponding to rcodes defined in IEEE 1394
    ///     specification.
    fn requested(
        &self,
        resp: &Self::Type,
        tcode: FwTcode,
        offset: u64,
        src: u32,
        dst: u32,
        card: u32,
        generation: u32,
        tstamp: u32,
        frame: &[u8],
    ) -> FwRcode {
        self.parent_requested(
            resp, tcode, offset, src, dst, card, generation, tstamp, frame,
        )
    }
}

/// Trait which is automatically implemented to implementator of [`FwRespImpl`][self::FwRespImpl].
pub trait FwRespImplExt: ObjectSubclass {
    fn parent_requested(
        &self,
        resp: &Self::Type,
        tcode: FwTcode,
        offset: u64,
        src: u32,
        dst: u32,
        card: u32,
        generation: u32,
        tstamp: u32,
        frame: &[u8],
    ) -> FwRcode;
}

impl<T: FwRespImpl> FwRespImplExt for T {
    fn parent_requested(
        &self,
        resp: &Self::Type,
        tcode: FwTcode,
        offset: u64,
        src: u32,
        dst: u32,
        card: u32,
        generation: u32,
        tstamp: u32,
        frame: &[u8],
    ) -> FwRcode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwRespClass;
            let f = (*parent_class)
                .requested
                .expect("No parent class implementation for \"requested\"");
            from_glib(f(
                resp.unsafe_cast_ref::<FwResp>().to_glib_none().0,
                tcode.into_glib(),
                offset,
                src,
                dst,
                card,
                generation,
                tstamp,
                frame.as_ptr(),
                frame.len() as u32,
            ))
        }
    }
}

unsafe impl<T: FwRespImpl> IsSubclassable<T> for FwResp {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.requested = Some(fw_resp_requested::<T>);
    }
}

unsafe extern "C" fn fw_resp_requested<T: FwRespImpl>(
    ptr: *mut ffi::HinawaFwResp,
    tcode: ffi::HinawaFwTcode,
    offset: u64,
    src: u32,
    dst: u32,
    card: u32,
    generation: u32,
    tstamp: u32,
    frame: *const u8,
    length: c_uint,
) -> ffi::HinawaFwRcode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwResp> = from_glib_borrow(ptr);

    imp.requested(
        wrap.unsafe_cast_ref(),
        from_glib(tcode),
        offset,
        src,
        dst,
        card,
        generation,
        tstamp,
        std::slice::from_raw_parts(frame, length as usize),
    )
    .into_glib()
}
