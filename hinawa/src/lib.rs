// SPDX-License-Identifier: MIT
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

use glib::{signal::*, translate::*, Cast, IsA, SignalHandlerId};
