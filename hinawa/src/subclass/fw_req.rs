// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwReq`][crate::FwReq].
pub trait FwReqImpl: ObjectImpl {
    fn responded(&self, req: &Self::Type, rcode: FwRcode, frame: &[u8]) {
        self.parent_responded(req, rcode, frame)
    }
}

/// Trait which is automatically implemented to implementator of [`FwReqImpl`][self::FwReqImpl].
pub trait FwReqImplExt: ObjectSubclass {
    fn parent_responded(&self, req: &Self::Type, rcode: FwRcode, frame: &[u8]);
}

impl<T: FwReqImpl> FwReqImplExt for T {
    fn parent_responded(&self, req: &Self::Type, rcode: FwRcode, frame: &[u8]) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwReqClass;
            let f = (*parent_class)
                .responded
                .expect("No parent class implementation for \"responded\"");
            f(
                req.unsafe_cast_ref::<FwReq>().to_glib_none().0,
                rcode.into_glib(),
                frame.as_ptr(),
                frame.len() as u32,
            )
        }
    }
}

unsafe impl<T: FwReqImpl> IsSubclassable<T> for FwReq {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.responded = Some(fw_req_responded::<T>);
    }
}

unsafe extern "C" fn fw_req_responded<T: FwReqImpl>(
    ptr: *mut ffi::HinawaFwReq,
    rcode: ffi::HinawaFwRcode,
    frame: *const u8,
    length: c_uint,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwReq> = from_glib_borrow(ptr);

    imp.responded(
        wrap.unsafe_cast_ref(),
        from_glib(rcode),
        std::slice::from_raw_parts(frame, length as usize),
    )
}
