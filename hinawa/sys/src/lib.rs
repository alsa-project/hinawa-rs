// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type HinawaFwFcpError = c_int;
pub const HINAWA_FW_FCP_ERROR_TIMEOUT: HinawaFwFcpError = 0;
pub const HINAWA_FW_FCP_ERROR_LARGE_RESP: HinawaFwFcpError = 1;
pub const HINAWA_FW_FCP_ERROR_ABORTED: HinawaFwFcpError = 2;

pub type HinawaFwNodeError = c_int;
pub const HINAWA_FW_NODE_ERROR_DISCONNECTED: HinawaFwNodeError = 0;
pub const HINAWA_FW_NODE_ERROR_OPENED: HinawaFwNodeError = 1;
pub const HINAWA_FW_NODE_ERROR_NOT_OPENED: HinawaFwNodeError = 2;
pub const HINAWA_FW_NODE_ERROR_FAILED: HinawaFwNodeError = 3;

pub type HinawaFwRcode = c_int;
pub const HINAWA_FW_RCODE_COMPLETE: HinawaFwRcode = 0;
pub const HINAWA_FW_RCODE_CONFLICT_ERROR: HinawaFwRcode = 4;
pub const HINAWA_FW_RCODE_DATA_ERROR: HinawaFwRcode = 5;
pub const HINAWA_FW_RCODE_TYPE_ERROR: HinawaFwRcode = 6;
pub const HINAWA_FW_RCODE_ADDRESS_ERROR: HinawaFwRcode = 7;
pub const HINAWA_FW_RCODE_SEND_ERROR: HinawaFwRcode = 16;
pub const HINAWA_FW_RCODE_CANCELLED: HinawaFwRcode = 17;
pub const HINAWA_FW_RCODE_BUSY: HinawaFwRcode = 18;
pub const HINAWA_FW_RCODE_GENERATION: HinawaFwRcode = 19;
pub const HINAWA_FW_RCODE_NO_ACK: HinawaFwRcode = 20;
pub const HINAWA_FW_RCODE_INVALID: HinawaFwRcode = 21;

pub type HinawaFwReqError = c_int;
pub const HINAWA_FW_REQ_ERROR_CONFLICT_ERROR: HinawaFwReqError = 4;
pub const HINAWA_FW_REQ_ERROR_DATA_ERROR: HinawaFwReqError = 5;
pub const HINAWA_FW_REQ_ERROR_TYPE_ERROR: HinawaFwReqError = 6;
pub const HINAWA_FW_REQ_ERROR_ADDRESS_ERROR: HinawaFwReqError = 7;
pub const HINAWA_FW_REQ_ERROR_SEND_ERROR: HinawaFwReqError = 16;
pub const HINAWA_FW_REQ_ERROR_CANCELLED: HinawaFwReqError = 17;
pub const HINAWA_FW_REQ_ERROR_BUSY: HinawaFwReqError = 18;
pub const HINAWA_FW_REQ_ERROR_GENERATION: HinawaFwReqError = 19;
pub const HINAWA_FW_REQ_ERROR_NO_ACK: HinawaFwReqError = 20;
pub const HINAWA_FW_REQ_ERROR_INVALID: HinawaFwReqError = 21;

pub type HinawaFwRespError = c_int;
pub const HINAWA_FW_RESP_ERROR_FAILED: HinawaFwRespError = 0;
pub const HINAWA_FW_RESP_ERROR_RESERVED: HinawaFwRespError = 1;
pub const HINAWA_FW_RESP_ERROR_ADDR_SPACE_USED: HinawaFwRespError = 2;

pub type HinawaFwTcode = c_int;
pub const HINAWA_FW_TCODE_WRITE_QUADLET_REQUEST: HinawaFwTcode = 0;
pub const HINAWA_FW_TCODE_WRITE_BLOCK_REQUEST: HinawaFwTcode = 1;
pub const HINAWA_FW_TCODE_WRITE_RESPONSE: HinawaFwTcode = 2;
pub const HINAWA_FW_TCODE_READ_QUADLET_REQUEST: HinawaFwTcode = 4;
pub const HINAWA_FW_TCODE_READ_BLOCK_REQUEST: HinawaFwTcode = 5;
pub const HINAWA_FW_TCODE_READ_QUADLET_RESPONSE: HinawaFwTcode = 6;
pub const HINAWA_FW_TCODE_READ_BLOCK_RESPONSE: HinawaFwTcode = 7;
pub const HINAWA_FW_TCODE_CYCLE_START: HinawaFwTcode = 8;
pub const HINAWA_FW_TCODE_LOCK_REQUEST: HinawaFwTcode = 9;
pub const HINAWA_FW_TCODE_STREAM_DATA: HinawaFwTcode = 10;
pub const HINAWA_FW_TCODE_LOCK_RESPONSE: HinawaFwTcode = 11;
pub const HINAWA_FW_TCODE_LOCK_MASK_SWAP: HinawaFwTcode = 17;
pub const HINAWA_FW_TCODE_LOCK_COMPARE_SWAP: HinawaFwTcode = 18;
pub const HINAWA_FW_TCODE_LOCK_FETCH_ADD: HinawaFwTcode = 19;
pub const HINAWA_FW_TCODE_LOCK_LITTLE_ADD: HinawaFwTcode = 20;
pub const HINAWA_FW_TCODE_LOCK_BOUNDED_ADD: HinawaFwTcode = 21;
pub const HINAWA_FW_TCODE_LOCK_WRAP_ADD: HinawaFwTcode = 22;
pub const HINAWA_FW_TCODE_LOCK_VENDOR_DEPENDENT: HinawaFwTcode = 23;

// Records
#[repr(C)]
pub struct HinawaCycleTime {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for HinawaCycleTime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaCycleTime @ {self:p}"))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwFcpClass {
    pub parent_class: HinawaFwRespClass,
    pub responded:
        Option<unsafe extern "C" fn(*mut HinawaFwFcp, c_uint, c_uint, *const u8, c_uint)>,
}

impl ::std::fmt::Debug for HinawaFwFcpClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwFcpClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("responded", &self.responded)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwNodeClass {
    pub parent_class: gobject::GObjectClass,
    pub bus_update: Option<unsafe extern "C" fn(*mut HinawaFwNode)>,
    pub disconnected: Option<unsafe extern "C" fn(*mut HinawaFwNode)>,
}

impl ::std::fmt::Debug for HinawaFwNodeClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwNodeClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("bus_update", &self.bus_update)
            .field("disconnected", &self.disconnected)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwReqClass {
    pub parent_class: gobject::GObjectClass,
    pub responded: Option<
        unsafe extern "C" fn(*mut HinawaFwReq, HinawaFwRcode, c_uint, c_uint, *const u8, c_uint),
    >,
}

impl ::std::fmt::Debug for HinawaFwReqClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwReqClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("responded", &self.responded)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwRespClass {
    pub parent_class: gobject::GObjectClass,
    pub requested: Option<
        unsafe extern "C" fn(
            *mut HinawaFwResp,
            HinawaFwTcode,
            u64,
            c_uint,
            c_uint,
            c_uint,
            c_uint,
            c_uint,
            *const u8,
            c_uint,
        ) -> HinawaFwRcode,
    >,
}

impl ::std::fmt::Debug for HinawaFwRespClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwRespClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .field("requested", &self.requested)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwFcp {
    pub parent_instance: HinawaFwResp,
}

impl ::std::fmt::Debug for HinawaFwFcp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwFcp @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwNode {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinawaFwNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwNode @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwReq {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinawaFwReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwReq @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HinawaFwResp {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for HinawaFwResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("HinawaFwResp @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[link(name = "hinawa")]
extern "C" {

    //=========================================================================
    // HinawaFwFcpError
    //=========================================================================
    pub fn hinawa_fw_fcp_error_get_type() -> GType;
    pub fn hinawa_fw_fcp_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinawaFwNodeError
    //=========================================================================
    pub fn hinawa_fw_node_error_get_type() -> GType;
    pub fn hinawa_fw_node_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinawaFwRcode
    //=========================================================================
    pub fn hinawa_fw_rcode_get_type() -> GType;

    //=========================================================================
    // HinawaFwReqError
    //=========================================================================
    pub fn hinawa_fw_req_error_get_type() -> GType;
    pub fn hinawa_fw_req_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinawaFwRespError
    //=========================================================================
    pub fn hinawa_fw_resp_error_get_type() -> GType;
    pub fn hinawa_fw_resp_error_quark() -> glib::GQuark;

    //=========================================================================
    // HinawaFwTcode
    //=========================================================================
    pub fn hinawa_fw_tcode_get_type() -> GType;

    //=========================================================================
    // HinawaCycleTime
    //=========================================================================
    pub fn hinawa_cycle_time_get_type() -> GType;
    pub fn hinawa_cycle_time_new() -> *mut HinawaCycleTime;
    pub fn hinawa_cycle_time_compute_tstamp(
        self_: *const HinawaCycleTime,
        tstamp: c_uint,
        isoc_cycle: *mut [c_uint; 2],
    );
    pub fn hinawa_cycle_time_get_clock_id(self_: *const HinawaCycleTime, clock_id: *mut c_int);
    pub fn hinawa_cycle_time_get_fields(self_: *const HinawaCycleTime, fields: *mut [u16; 3]);
    pub fn hinawa_cycle_time_get_raw(self_: *const HinawaCycleTime, raw: *mut u32);
    pub fn hinawa_cycle_time_get_system_time(
        self_: *const HinawaCycleTime,
        tv_sec: *mut i64,
        tv_nsec: *mut i32,
    );
    pub fn hinawa_cycle_time_parse_tstamp(tstamp: c_uint, isoc_cycle: *mut [c_uint; 2]);

    //=========================================================================
    // HinawaFwFcp
    //=========================================================================
    pub fn hinawa_fw_fcp_get_type() -> GType;
    pub fn hinawa_fw_fcp_new() -> *mut HinawaFwFcp;
    pub fn hinawa_fw_fcp_avc_transaction(
        self_: *mut HinawaFwFcp,
        cmd: *const u8,
        cmd_size: size_t,
        resp: *mut *mut u8,
        resp_size: *mut size_t,
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_fcp_avc_transaction_with_tstamp(
        self_: *mut HinawaFwFcp,
        cmd: *const u8,
        cmd_size: size_t,
        resp: *mut *mut u8,
        resp_size: *mut size_t,
        tstamp: *mut [c_uint; 3],
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_fcp_bind(
        self_: *mut HinawaFwFcp,
        node: *mut HinawaFwNode,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_fcp_command(
        self_: *mut HinawaFwFcp,
        cmd: *const u8,
        cmd_size: size_t,
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_fcp_command_with_tstamp(
        self_: *mut HinawaFwFcp,
        cmd: *const u8,
        cmd_size: size_t,
        tstamp: *mut [c_uint; 2],
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_fcp_unbind(self_: *mut HinawaFwFcp);

    //=========================================================================
    // HinawaFwNode
    //=========================================================================
    pub fn hinawa_fw_node_get_type() -> GType;
    pub fn hinawa_fw_node_new() -> *mut HinawaFwNode;
    pub fn hinawa_fw_node_create_source(
        self_: *mut HinawaFwNode,
        gsrc: *mut *mut glib::GSource,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_node_get_config_rom(
        self_: *mut HinawaFwNode,
        image: *mut *const u8,
        length: *mut size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_node_open(
        self_: *mut HinawaFwNode,
        path: *const c_char,
        open_flag: c_int,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_node_read_cycle_time(
        self_: *mut HinawaFwNode,
        clock_id: c_int,
        cycle_time: *mut *mut HinawaCycleTime,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinawaFwReq
    //=========================================================================
    pub fn hinawa_fw_req_get_type() -> GType;
    pub fn hinawa_fw_req_new() -> *mut HinawaFwReq;
    pub fn hinawa_fw_req_request(
        self_: *mut HinawaFwReq,
        node: *mut HinawaFwNode,
        tcode: HinawaFwTcode,
        addr: u64,
        length: size_t,
        frame: *mut *mut u8,
        frame_size: *mut size_t,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_req_transaction(
        self_: *mut HinawaFwReq,
        node: *mut HinawaFwNode,
        tcode: HinawaFwTcode,
        addr: u64,
        length: size_t,
        frame: *mut *mut u8,
        frame_size: *mut size_t,
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_req_transaction_with_tstamp(
        self_: *mut HinawaFwReq,
        node: *mut HinawaFwNode,
        tcode: HinawaFwTcode,
        addr: u64,
        length: size_t,
        frame: *mut *mut u8,
        frame_size: *mut size_t,
        tstamp: *mut [c_uint; 2],
        timeout_ms: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;

    //=========================================================================
    // HinawaFwResp
    //=========================================================================
    pub fn hinawa_fw_resp_get_type() -> GType;
    pub fn hinawa_fw_resp_new() -> *mut HinawaFwResp;
    pub fn hinawa_fw_resp_release(self_: *mut HinawaFwResp);
    pub fn hinawa_fw_resp_reserve(
        self_: *mut HinawaFwResp,
        node: *mut HinawaFwNode,
        addr: u64,
        width: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_resp_reserve_within_region(
        self_: *mut HinawaFwResp,
        node: *mut HinawaFwNode,
        region_start: u64,
        region_end: u64,
        width: c_uint,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn hinawa_fw_resp_set_resp_frame(self_: *mut HinawaFwResp, frame: *mut u8, length: size_t);

}
