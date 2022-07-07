// SPDX-License-Identifier: MIT

// For subclass of [`FwFcp`][crate::FwFcp].
mod fw_fcp;

// For subclass of [`FwNode`][crate::FwNode].
mod fw_node;

// For subclass of [`FwReq`][crate::FwReq].
mod fw_req;

// For subclass of [`FwResp`][crate::FwResp].
mod fw_resp;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::{fw_fcp::*, fw_node::*, fw_req::*, fw_resp::*};
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class},
    libc::*,
    prelude::*,
};
