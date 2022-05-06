// SPDX-License-Identifier: MIT

use super::*;

pub trait FwRespImpl: ObjectImpl + FwRespImplExt {
    fn requested(&self, resp: &FwResp, tcode: FwTcode) -> FwRcode {
        self.parent_requested(resp, tcode)
    }

    fn requested2(
        &self,
        resp: &FwResp,
        tcode: FwTcode,
        offset: u64,
        src: u32,
        dst: u32,
        card: u32,
        generation: u32,
        frame: &[u8],
    ) -> FwRcode {
        self.parent_requested2(resp, tcode, offset, src, dst, card, generation, frame)
    }
}

pub trait FwRespImplExt: ObjectSubclass {
    fn parent_requested(&self, resp: &FwResp, tcode: FwTcode) -> FwRcode;
    fn parent_requested2(
        &self,
        resp: &FwResp,
        tcode: FwTcode,
        offset: u64,
        src: u32,
        dst: u32,
        card: u32,
        generation: u32,
        frame: &[u8],
    ) -> FwRcode;
}

impl<T: FwRespImpl> FwRespImplExt for T {
    fn parent_requested(&self, resp: &FwResp, tcode: FwTcode) -> FwRcode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwRespClass;
            let f = (*parent_class)
                .requested
                .expect("No parent class implementation for \"requested\"");
            from_glib(f(resp.to_glib_none().0, tcode.into_glib()))
        }
    }

    fn parent_requested2(
        &self,
        resp: &FwResp,
        tcode: FwTcode,
        offset: u64,
        src: u32,
        dst: u32,
        card: u32,
        generation: u32,
        frame: &[u8],
    ) -> FwRcode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwRespClass;
            let f = (*parent_class)
                .requested2
                .expect("No parent class implementation for \"requested\"");
            from_glib(f(
                resp.to_glib_none().0,
                tcode.into_glib(),
                offset,
                src,
                dst,
                card,
                generation,
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
        klass.requested2 = Some(fw_resp_requested2::<T>);
    }
}

unsafe extern "C" fn fw_resp_requested<T: FwRespImpl>(
    ptr: *mut ffi::HinawaFwResp,
    tcode: ffi::HinawaFwTcode,
) -> ffi::HinawaFwRcode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwResp> = from_glib_borrow(ptr);

    imp.requested(&wrap, from_glib(tcode)).into_glib()
}

unsafe extern "C" fn fw_resp_requested2<T: FwRespImpl>(
    ptr: *mut ffi::HinawaFwResp,
    tcode: ffi::HinawaFwTcode,
    offset: u64,
    src: u32,
    dst: u32,
    card: u32,
    generation: u32,
    frame: *const u8,
    length: c_uint,
) -> ffi::HinawaFwRcode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwResp> = from_glib_borrow(ptr);

    imp.requested2(
        &wrap,
        from_glib(tcode),
        offset,
        src,
        dst,
        card,
        generation,
        std::slice::from_raw_parts(frame, length as usize),
    )
    .into_glib()
}
