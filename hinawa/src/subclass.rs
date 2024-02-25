// SPDX-License-Identifier: MIT

mod fw_fcp;

mod fw_node;

mod fw_req;

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
