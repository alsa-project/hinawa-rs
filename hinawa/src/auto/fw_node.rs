// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    /// An event listener for node in IEEE 1394 bus.
    ///
    /// [`FwNode`][crate::FwNode] listens to any events for an associated node in IEEE 1394 bus. Additionally,
    /// it provides some methods to retrieve fundamental information about the bus.
    ///
    /// # Implements
    ///
    /// [`FwNodeExt`][trait@crate::prelude::FwNodeExt], [`FwNodeExtManual`][trait@crate::prelude::FwNodeExtManual]
    #[doc(alias = "HinawaFwNode")]
    pub struct FwNode(Object<ffi::HinawaFwNode, ffi::HinawaFwNodeClass>);

    match fn {
        type_ => || ffi::hinawa_fw_node_get_type(),
    }
}

impl FwNode {
    pub const NONE: Option<&'static FwNode> = None;

    /// Instantiate [`FwNode`][crate::FwNode] object and return the instance.
    ///
    /// # Returns
    ///
    /// an instance of [`FwNode`][crate::FwNode].
    #[doc(alias = "hinawa_fw_node_new")]
    pub fn new() -> FwNode {
        unsafe { from_glib_full(ffi::hinawa_fw_node_new()) }
    }
}

impl Default for FwNode {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing the part of [`struct@FwNode`] methods.
///
/// # Implementors
///
/// [`FwNode`][struct@crate::FwNode]
pub trait FwNodeExt: 'static {
    /// Create [`glib::Source`][crate::glib::Source] for `GLib::MainContext` to dispatch events for the node on
    /// IEEE 1394 bus.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    ///
    /// ## `gsrc`
    /// A [`glib::Source`][crate::glib::Source].
    #[doc(alias = "hinawa_fw_node_create_source")]
    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    /// Open Linux FireWire character device to operate node in IEEE 1394 bus.
    /// ## `path`
    /// A path to Linux FireWire character device
    /// ## `open_flag`
    /// The flag of `open(2)` system call. `O_RDONLY` is fulfilled internally.
    ///
    /// # Returns
    ///
    /// TRUE if the overall operation finishes successfully, otherwise FALSE.
    #[doc(alias = "hinawa_fw_node_open")]
    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error>;

    /// Node ID of node which plays role of bus manager at current generation of bus topology.
    #[doc(alias = "bus-manager-node-id")]
    fn bus_manager_node_id(&self) -> u32;

    /// The numeric index for 1394 OHCI hardware used for the communication with the node. The
    /// value is stable against bus generation.
    #[doc(alias = "card-id")]
    fn card_id(&self) -> u32;

    /// Current generation of bus topology.
    fn generation(&self) -> u32;

    /// Node ID of node which plays role of isochronous resource manager at current generation
    /// of bus topology.
    #[doc(alias = "ir-manager-node-id")]
    fn ir_manager_node_id(&self) -> u32;

    /// Node ID of node which application uses to communicate to node associated to instance of
    /// object at current generation of bus topology. In general, it is for 1394 OHCI hardware.
    #[doc(alias = "local-node-id")]
    fn local_node_id(&self) -> u32;

    /// Node ID of node associated to instance of object at current generation of bus topology.
    /// This parameter is effective after the association.
    #[doc(alias = "node-id")]
    fn node_id(&self) -> u32;

    /// Node ID of root node in bus topology at current generation of bus topology.
    #[doc(alias = "root-node-id")]
    fn root_node_id(&self) -> u32;

    /// Emitted when IEEE 1394 bus is updated. Handlers can read current generation in the bus
    /// via `property::FwNode::generation` property.
    #[doc(alias = "bus-update")]
    fn connect_bus_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// Emitted when the node is not available anymore in Linux system. It's preferable to call
    /// `GObject::Object::unref()` immediately to release file descriptor.
    #[doc(alias = "disconnected")]
    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "bus-manager-node-id")]
    fn connect_bus_manager_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "card-id")]
    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "generation")]
    fn connect_generation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ir-manager-node-id")]
    fn connect_ir_manager_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "local-node-id")]
    fn connect_local_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "node-id")]
    fn connect_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "root-node-id")]
    fn connect_root_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FwNode>> FwNodeExt for O {
    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut gsrc = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::hinawa_fw_node_create_source(
                self.as_ref().to_glib_none().0,
                &mut gsrc,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(gsrc))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn open(&self, path: &str, open_flag: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::hinawa_fw_node_open(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                open_flag,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn bus_manager_node_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "bus-manager-node-id")
    }

    fn card_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "card-id")
    }

    fn generation(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "generation")
    }

    fn ir_manager_node_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "ir-manager-node-id")
    }

    fn local_node_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "local-node-id")
    }

    fn node_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "node-id")
    }

    fn root_node_id(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "root-node-id")
    }

    fn connect_bus_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn bus_update_trampoline<P: IsA<FwNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwNode,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bus-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bus_update_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn disconnected_trampoline<P: IsA<FwNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwNode,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"disconnected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    disconnected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_bus_manager_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bus_manager_node_id_trampoline<
            P: IsA<FwNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bus-manager-node-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bus_manager_node_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P: IsA<FwNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_card_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_generation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_generation_trampoline<P: IsA<FwNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::generation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_generation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ir_manager_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ir_manager_node_id_trampoline<
            P: IsA<FwNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ir-manager-node-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ir_manager_node_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_local_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_node_id_trampoline<
            P: IsA<FwNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-node-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_node_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_node_id_trampoline<P: IsA<FwNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::node-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_node_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_root_node_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_root_node_id_trampoline<P: IsA<FwNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::HinawaFwNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FwNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::root-node-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_root_node_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FwNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FwNode")
    }
}
