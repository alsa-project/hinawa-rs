// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod cycle_time;
mod fw_fcp;
mod fw_node;
mod fw_req;
mod fw_resp;

pub use crate::auto::*;

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{auto::traits::*, fw_fcp::*, fw_node::*, fw_req::*, fw_resp::*};
}

/// For subclass implementations derived from provided class.
pub mod subclass;

pub use ffi;

pub(crate) use glib;

use glib::{object::*, signal::*, translate::*};
