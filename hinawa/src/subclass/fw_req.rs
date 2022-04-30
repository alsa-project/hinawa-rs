// SPDX-License-Identifier: MIT

use super::*;

pub trait FwReqImpl: ObjectImpl + FwReqImplExt {
    fn responded(&self, req: &FwReq, rcode: FwRcode, frame: &[u8]) {
        self.parent_responded(req, rcode, frame)
    }
}

pub trait FwReqImplExt: ObjectSubclass {
    fn parent_responded(&self, req: &FwReq, rcode: FwRcode, frame: &[u8]);
}

impl<T: FwReqImpl> FwReqImplExt for T {
    fn parent_responded(&self, req: &FwReq, rcode: FwRcode, frame: &[u8]) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut hinawa_sys::HinawaFwReqClass;
            let f = (*parent_class)
                .responded
                .expect("No parent class implementation for \"responded\"");
            f(
                req.to_glib_none().0,
                rcode.to_glib(),
                frame.as_ptr(),
                frame.len() as u32,
            )
        }
    }
}

unsafe impl<T: FwReqImpl> IsSubclassable<T> for FwReqClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut hinawa_sys::HinawaFwReqClass);
            klass.responded = Some(fw_req_responded::<T>);
        }
    }
}

unsafe extern "C" fn fw_req_responded<T: FwReqImpl>(
    ptr: *mut hinawa_sys::HinawaFwReq,
    rcode: hinawa_sys::HinawaFwRcode,
    frame: *const u8,
    length: c_uint,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<FwReq> = from_glib_borrow(ptr);

    imp.responded(
        &wrap,
        from_glib(rcode),
        std::slice::from_raw_parts(frame, length as usize),
    )
}
