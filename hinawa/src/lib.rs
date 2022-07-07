// SPDX-License-Identifier: MIT

//! Rust bindings for hinawa library
//!
//! Rust bindings and wrappers for [hinawa library](https://github.com/alsa-project/libhinawa) to
//! operate Linux FireWire character device for asynchronous packet and topology generation in
//! IEEE 1394 bus.
//!
//! The hinawa library v2.5.0 is the minimum supported version for underlying library.
//!
//! The crate depends on [glib crate v0.15](https://crates.io/crates/glib) provided by
//! [gtk-rs project](https://gtk-rs.org/) for type/object system, event loop, and dispacher.
//!
//! # License
//!
//! Released under MIT license.
//!
//! # Sample programs
//!
//! Some programs are available under `examples` directory.
//!
//! `read-quadlet`
//! : demonstration to read quadlet data from node in IEEE 1394 bus.

mod auto;
mod fw_fcp;
mod fw_node;
mod fw_req;
mod fw_resp;

// For convenience to provide structures and functions.
pub use crate::auto::*;

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{auto::traits::*, fw_fcp::*, fw_node::*, fw_req::*, fw_resp::*};
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to hinawa-sys crate for FFI.
pub use ffi;

// For links in documentation.
use glib;

use glib::{signal::*, translate::*, Cast, IsA, SignalHandlerId};
