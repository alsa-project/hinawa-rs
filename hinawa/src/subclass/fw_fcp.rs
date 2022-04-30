// SPDX-License-Identifier: MIT

use super::*;

pub trait FwFcpImpl: ObjectImpl + FwFcpImplExt {
    fn responded(&self, fcp: &FwFcp, frame: &[u8]) {
        self.parent_responded(fcp, frame)
    }
}

pub trait FwFcpImplExt: ObjectSubclass {
    fn parent_responded(&self, fcp: &FwFcp, frame: &[u8]);
}

impl<T: FwFcpImpl> FwFcpImplExt for T {
    fn parent_responded(&self, fcp: &FwFcp, frame: &[u8]) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut hinawa_sys::HinawaFwFcpClass;
            let f = (*parent_class)
                .responded
                .expect("No parent class implementation for \"responded\"");
            f(fcp.to_glib_none().0, frame.as_ptr(), frame.len() as u32)
        }
    }
}

unsafe impl<T: FwFcpImpl> IsSubclassable<T> for FwFcpClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut hinawa_sys::HinawaFwFcpClass);
            klass.responded = Some(fw_fcp_responded::<T>);
        }
    }
}

unsafe extern "C" fn fw_fcp_responded<T: FwFcpImpl>(
    ptr: *mut hinawa_sys::HinawaFwFcp,
    frame: *const u8,
    length: c_uint,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<FwFcp> = from_glib_borrow(ptr);

    imp.responded(&wrap, std::slice::from_raw_parts(frame, length as usize))
}
