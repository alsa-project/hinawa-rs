// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`FwFcp`][crate::FwFcp].
pub trait FwFcpImpl: ObjectImpl + FwRespImpl {
    fn responded(&self, fcp: &Self::Type, frame: &[u8]) {
        self.parent_responded(fcp, frame)
    }
}

/// Trait which is automatically implemented to implementator of [`FwFcpImpl`][self::FwFcpImpl].
pub trait FwFcpImplExt: ObjectSubclass {
    fn parent_responded(&self, fcp: &Self::Type, frame: &[u8]);
}

impl<T: FwFcpImpl> FwFcpImplExt for T {
    fn parent_responded(&self, fcp: &Self::Type, frame: &[u8]) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwFcpClass;
            let f = (*parent_class)
                .responded
                .expect("No parent class implementation for \"responded\"");
            f(
                fcp.unsafe_cast_ref::<FwFcp>().to_glib_none().0,
                frame.as_ptr(),
                frame.len() as u32,
            )
        }
    }
}

unsafe impl<T: FwFcpImpl> IsSubclassable<T> for FwFcp {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.responded = Some(fw_fcp_responded::<T>);
    }
}

unsafe extern "C" fn fw_fcp_responded<T: FwFcpImpl>(
    ptr: *mut ffi::HinawaFwFcp,
    frame: *const u8,
    length: c_uint,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwFcp> = from_glib_borrow(ptr);

    imp.responded(
        wrap.unsafe_cast_ref(),
        std::slice::from_raw_parts(frame, length as usize),
    )
}
