// SPDX-License-Identifier: MIT

use super::*;

pub trait FwNodeImpl: ObjectImpl + FwNodeImplExt {
    fn bus_update(&self, node: &FwNode) {
        self.parent_bus_update(node)
    }

    fn disconnected(&self, node: &FwNode) {
        self.parent_disconnected(node)
    }
}

pub trait FwNodeImplExt: ObjectSubclass {
    fn parent_bus_update(&self, node: &FwNode);
    fn parent_disconnected(&self, node: &FwNode);
}

impl<T: FwNodeImpl> FwNodeImplExt for T {
    fn parent_bus_update(&self, node: &FwNode) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwNodeClass;
            let f = (*parent_class)
                .bus_update
                .expect("No parent class implementation for \"bus_update\"");
            f(node.to_glib_none().0)
        }
    }

    fn parent_disconnected(&self, node: &FwNode) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::HinawaFwNodeClass;
            let f = (*parent_class)
                .disconnected
                .expect("No parent class implementation for \"disconnected\"");
            f(node.to_glib_none().0);
        }
    }
}

unsafe impl<T: FwNodeImpl> IsSubclassable<T> for FwNode {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.bus_update = Some(fw_node_bus_update::<T>);
        klass.disconnected = Some(fw_node_disconnected::<T>);
    }
}

unsafe extern "C" fn fw_node_bus_update<T: FwNodeImpl>(ptr: *mut ffi::HinawaFwNode) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwNode> = from_glib_borrow(ptr);

    imp.bus_update(&wrap)
}

unsafe extern "C" fn fw_node_disconnected<T: FwNodeImpl>(ptr: *mut ffi::HinawaFwNode) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<FwNode> = from_glib_borrow(ptr);

    imp.disconnected(&wrap)
}
