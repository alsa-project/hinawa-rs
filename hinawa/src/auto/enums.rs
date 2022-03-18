// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaFwFcpError")]
pub enum FwFcpError {
    #[doc(alias = "HINAWA_FW_FCP_ERROR_TIMEOUT")]
    Timeout,
    #[doc(alias = "HINAWA_FW_FCP_ERROR_LARGE_RESP")]
    LargeResp,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FwFcpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FwFcpError::{}",
            match *self {
                Self::Timeout => "Timeout",
                Self::LargeResp => "LargeResp",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for FwFcpError {
    type GlibType = ffi::HinawaFwFcpError;

    fn into_glib(self) -> ffi::HinawaFwFcpError {
        match self {
            Self::Timeout => ffi::HINAWA_FW_FCP_ERROR_TIMEOUT,
            Self::LargeResp => ffi::HINAWA_FW_FCP_ERROR_LARGE_RESP,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaFwFcpError> for FwFcpError {
    unsafe fn from_glib(value: ffi::HinawaFwFcpError) -> Self {
        match value {
            ffi::HINAWA_FW_FCP_ERROR_TIMEOUT => Self::Timeout,
            ffi::HINAWA_FW_FCP_ERROR_LARGE_RESP => Self::LargeResp,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for FwFcpError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::hinawa_fw_fcp_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::HINAWA_FW_FCP_ERROR_TIMEOUT => Some(Self::Timeout),
            ffi::HINAWA_FW_FCP_ERROR_LARGE_RESP => Some(Self::LargeResp),
            value => Some(Self::__Unknown(value)),
        }
    }
}

impl StaticType for FwFcpError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_fw_fcp_error_get_type()) }
    }
}

impl glib::value::ValueType for FwFcpError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FwFcpError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwFcpError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaFwNodeError")]
pub enum FwNodeError {
    #[doc(alias = "HINAWA_FW_NODE_ERROR_DISCONNECTED")]
    Disconnected,
    #[doc(alias = "HINAWA_FW_NODE_ERROR_OPENED")]
    Opened,
    #[doc(alias = "HINAWA_FW_NODE_ERROR_NOT_OPENED")]
    NotOpened,
    #[doc(alias = "HINAWA_FW_NODE_ERROR_FAILED")]
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FwNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FwNodeError::{}",
            match *self {
                Self::Disconnected => "Disconnected",
                Self::Opened => "Opened",
                Self::NotOpened => "NotOpened",
                Self::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for FwNodeError {
    type GlibType = ffi::HinawaFwNodeError;

    fn into_glib(self) -> ffi::HinawaFwNodeError {
        match self {
            Self::Disconnected => ffi::HINAWA_FW_NODE_ERROR_DISCONNECTED,
            Self::Opened => ffi::HINAWA_FW_NODE_ERROR_OPENED,
            Self::NotOpened => ffi::HINAWA_FW_NODE_ERROR_NOT_OPENED,
            Self::Failed => ffi::HINAWA_FW_NODE_ERROR_FAILED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaFwNodeError> for FwNodeError {
    unsafe fn from_glib(value: ffi::HinawaFwNodeError) -> Self {
        match value {
            ffi::HINAWA_FW_NODE_ERROR_DISCONNECTED => Self::Disconnected,
            ffi::HINAWA_FW_NODE_ERROR_OPENED => Self::Opened,
            ffi::HINAWA_FW_NODE_ERROR_NOT_OPENED => Self::NotOpened,
            ffi::HINAWA_FW_NODE_ERROR_FAILED => Self::Failed,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for FwNodeError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::hinawa_fw_node_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::HINAWA_FW_NODE_ERROR_DISCONNECTED => Some(Self::Disconnected),
            ffi::HINAWA_FW_NODE_ERROR_OPENED => Some(Self::Opened),
            ffi::HINAWA_FW_NODE_ERROR_NOT_OPENED => Some(Self::NotOpened),
            ffi::HINAWA_FW_NODE_ERROR_FAILED => Some(Self::Failed),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for FwNodeError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_fw_node_error_get_type()) }
    }
}

impl glib::value::ValueType for FwNodeError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FwNodeError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwNodeError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaFwRcode")]
pub enum FwRcode {
    #[doc(alias = "HINAWA_FW_RCODE_COMPLETE")]
    Complete,
    #[doc(alias = "HINAWA_FW_RCODE_CONFLICT_ERROR")]
    ConflictError,
    #[doc(alias = "HINAWA_FW_RCODE_DATA_ERROR")]
    DataError,
    #[doc(alias = "HINAWA_FW_RCODE_TYPE_ERROR")]
    TypeError,
    #[doc(alias = "HINAWA_FW_RCODE_ADDRESS_ERROR")]
    AddressError,
    #[doc(alias = "HINAWA_FW_RCODE_SEND_ERROR")]
    SendError,
    #[doc(alias = "HINAWA_FW_RCODE_CANCELLED")]
    Cancelled,
    #[doc(alias = "HINAWA_FW_RCODE_BUSY")]
    Busy,
    #[doc(alias = "HINAWA_FW_RCODE_GENERATION")]
    Generation,
    #[doc(alias = "HINAWA_FW_RCODE_NO_ACK")]
    NoAck,
    #[doc(alias = "HINAWA_FW_RCODE_INVALID")]
    Invalid,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FwRcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FwRcode::{}",
            match *self {
                Self::Complete => "Complete",
                Self::ConflictError => "ConflictError",
                Self::DataError => "DataError",
                Self::TypeError => "TypeError",
                Self::AddressError => "AddressError",
                Self::SendError => "SendError",
                Self::Cancelled => "Cancelled",
                Self::Busy => "Busy",
                Self::Generation => "Generation",
                Self::NoAck => "NoAck",
                Self::Invalid => "Invalid",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for FwRcode {
    type GlibType = ffi::HinawaFwRcode;

    fn into_glib(self) -> ffi::HinawaFwRcode {
        match self {
            Self::Complete => ffi::HINAWA_FW_RCODE_COMPLETE,
            Self::ConflictError => ffi::HINAWA_FW_RCODE_CONFLICT_ERROR,
            Self::DataError => ffi::HINAWA_FW_RCODE_DATA_ERROR,
            Self::TypeError => ffi::HINAWA_FW_RCODE_TYPE_ERROR,
            Self::AddressError => ffi::HINAWA_FW_RCODE_ADDRESS_ERROR,
            Self::SendError => ffi::HINAWA_FW_RCODE_SEND_ERROR,
            Self::Cancelled => ffi::HINAWA_FW_RCODE_CANCELLED,
            Self::Busy => ffi::HINAWA_FW_RCODE_BUSY,
            Self::Generation => ffi::HINAWA_FW_RCODE_GENERATION,
            Self::NoAck => ffi::HINAWA_FW_RCODE_NO_ACK,
            Self::Invalid => ffi::HINAWA_FW_RCODE_INVALID,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaFwRcode> for FwRcode {
    unsafe fn from_glib(value: ffi::HinawaFwRcode) -> Self {
        match value {
            ffi::HINAWA_FW_RCODE_COMPLETE => Self::Complete,
            ffi::HINAWA_FW_RCODE_CONFLICT_ERROR => Self::ConflictError,
            ffi::HINAWA_FW_RCODE_DATA_ERROR => Self::DataError,
            ffi::HINAWA_FW_RCODE_TYPE_ERROR => Self::TypeError,
            ffi::HINAWA_FW_RCODE_ADDRESS_ERROR => Self::AddressError,
            ffi::HINAWA_FW_RCODE_SEND_ERROR => Self::SendError,
            ffi::HINAWA_FW_RCODE_CANCELLED => Self::Cancelled,
            ffi::HINAWA_FW_RCODE_BUSY => Self::Busy,
            ffi::HINAWA_FW_RCODE_GENERATION => Self::Generation,
            ffi::HINAWA_FW_RCODE_NO_ACK => Self::NoAck,
            ffi::HINAWA_FW_RCODE_INVALID => Self::Invalid,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for FwRcode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_fw_rcode_get_type()) }
    }
}

impl glib::value::ValueType for FwRcode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FwRcode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwRcode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaFwRespError")]
pub enum FwRespError {
    #[doc(alias = "HINAWA_FW_RESP_ERROR_FAILED")]
    Failed,
    #[doc(alias = "HINAWA_FW_RESP_ERROR_RESERVED")]
    Reserved,
    #[doc(alias = "HINAWA_FW_RESP_ERROR_ADDR_SPACE_USED")]
    AddrSpaceUsed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FwRespError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FwRespError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::Reserved => "Reserved",
                Self::AddrSpaceUsed => "AddrSpaceUsed",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for FwRespError {
    type GlibType = ffi::HinawaFwRespError;

    fn into_glib(self) -> ffi::HinawaFwRespError {
        match self {
            Self::Failed => ffi::HINAWA_FW_RESP_ERROR_FAILED,
            Self::Reserved => ffi::HINAWA_FW_RESP_ERROR_RESERVED,
            Self::AddrSpaceUsed => ffi::HINAWA_FW_RESP_ERROR_ADDR_SPACE_USED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaFwRespError> for FwRespError {
    unsafe fn from_glib(value: ffi::HinawaFwRespError) -> Self {
        match value {
            ffi::HINAWA_FW_RESP_ERROR_FAILED => Self::Failed,
            ffi::HINAWA_FW_RESP_ERROR_RESERVED => Self::Reserved,
            ffi::HINAWA_FW_RESP_ERROR_ADDR_SPACE_USED => Self::AddrSpaceUsed,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for FwRespError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::hinawa_fw_resp_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::HINAWA_FW_RESP_ERROR_FAILED => Some(Self::Failed),
            ffi::HINAWA_FW_RESP_ERROR_RESERVED => Some(Self::Reserved),
            ffi::HINAWA_FW_RESP_ERROR_ADDR_SPACE_USED => Some(Self::AddrSpaceUsed),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for FwRespError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_fw_resp_error_get_type()) }
    }
}

impl glib::value::ValueType for FwRespError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FwRespError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwRespError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaFwTcode")]
pub enum FwTcode {
    #[doc(alias = "HINAWA_FW_TCODE_WRITE_QUADLET_REQUEST")]
    WriteQuadletRequest,
    #[doc(alias = "HINAWA_FW_TCODE_WRITE_BLOCK_REQUEST")]
    WriteBlockRequest,
    #[doc(alias = "HINAWA_FW_TCODE_WRITE_RESPONSE")]
    WriteResponse,
    #[doc(alias = "HINAWA_FW_TCODE_READ_QUADLET_REQUEST")]
    ReadQuadletRequest,
    #[doc(alias = "HINAWA_FW_TCODE_READ_BLOCK_REQUEST")]
    ReadBlockRequest,
    #[doc(alias = "HINAWA_FW_TCODE_READ_QUADLET_RESPONSE")]
    ReadQuadletResponse,
    #[doc(alias = "HINAWA_FW_TCODE_READ_BLOCK_RESPONSE")]
    ReadBlockResponse,
    #[doc(alias = "HINAWA_FW_TCODE_CYCLE_START")]
    CycleStart,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_REQUEST")]
    LockRequest,
    #[doc(alias = "HINAWA_FW_TCODE_STREAM_DATA")]
    StreamData,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_RESPONSE")]
    LockResponse,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_MASK_SWAP")]
    LockMaskSwap,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_COMPARE_SWAP")]
    LockCompareSwap,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_FETCH_ADD")]
    LockFetchAdd,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_LITTLE_ADD")]
    LockLittleAdd,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_BOUNDED_ADD")]
    LockBoundedAdd,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_WRAP_ADD")]
    LockWrapAdd,
    #[doc(alias = "HINAWA_FW_TCODE_LOCK_VENDOR_DEPENDENT")]
    LockVendorDependent,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FwTcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FwTcode::{}",
            match *self {
                Self::WriteQuadletRequest => "WriteQuadletRequest",
                Self::WriteBlockRequest => "WriteBlockRequest",
                Self::WriteResponse => "WriteResponse",
                Self::ReadQuadletRequest => "ReadQuadletRequest",
                Self::ReadBlockRequest => "ReadBlockRequest",
                Self::ReadQuadletResponse => "ReadQuadletResponse",
                Self::ReadBlockResponse => "ReadBlockResponse",
                Self::CycleStart => "CycleStart",
                Self::LockRequest => "LockRequest",
                Self::StreamData => "StreamData",
                Self::LockResponse => "LockResponse",
                Self::LockMaskSwap => "LockMaskSwap",
                Self::LockCompareSwap => "LockCompareSwap",
                Self::LockFetchAdd => "LockFetchAdd",
                Self::LockLittleAdd => "LockLittleAdd",
                Self::LockBoundedAdd => "LockBoundedAdd",
                Self::LockWrapAdd => "LockWrapAdd",
                Self::LockVendorDependent => "LockVendorDependent",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for FwTcode {
    type GlibType = ffi::HinawaFwTcode;

    fn into_glib(self) -> ffi::HinawaFwTcode {
        match self {
            Self::WriteQuadletRequest => ffi::HINAWA_FW_TCODE_WRITE_QUADLET_REQUEST,
            Self::WriteBlockRequest => ffi::HINAWA_FW_TCODE_WRITE_BLOCK_REQUEST,
            Self::WriteResponse => ffi::HINAWA_FW_TCODE_WRITE_RESPONSE,
            Self::ReadQuadletRequest => ffi::HINAWA_FW_TCODE_READ_QUADLET_REQUEST,
            Self::ReadBlockRequest => ffi::HINAWA_FW_TCODE_READ_BLOCK_REQUEST,
            Self::ReadQuadletResponse => ffi::HINAWA_FW_TCODE_READ_QUADLET_RESPONSE,
            Self::ReadBlockResponse => ffi::HINAWA_FW_TCODE_READ_BLOCK_RESPONSE,
            Self::CycleStart => ffi::HINAWA_FW_TCODE_CYCLE_START,
            Self::LockRequest => ffi::HINAWA_FW_TCODE_LOCK_REQUEST,
            Self::StreamData => ffi::HINAWA_FW_TCODE_STREAM_DATA,
            Self::LockResponse => ffi::HINAWA_FW_TCODE_LOCK_RESPONSE,
            Self::LockMaskSwap => ffi::HINAWA_FW_TCODE_LOCK_MASK_SWAP,
            Self::LockCompareSwap => ffi::HINAWA_FW_TCODE_LOCK_COMPARE_SWAP,
            Self::LockFetchAdd => ffi::HINAWA_FW_TCODE_LOCK_FETCH_ADD,
            Self::LockLittleAdd => ffi::HINAWA_FW_TCODE_LOCK_LITTLE_ADD,
            Self::LockBoundedAdd => ffi::HINAWA_FW_TCODE_LOCK_BOUNDED_ADD,
            Self::LockWrapAdd => ffi::HINAWA_FW_TCODE_LOCK_WRAP_ADD,
            Self::LockVendorDependent => ffi::HINAWA_FW_TCODE_LOCK_VENDOR_DEPENDENT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaFwTcode> for FwTcode {
    unsafe fn from_glib(value: ffi::HinawaFwTcode) -> Self {
        match value {
            ffi::HINAWA_FW_TCODE_WRITE_QUADLET_REQUEST => Self::WriteQuadletRequest,
            ffi::HINAWA_FW_TCODE_WRITE_BLOCK_REQUEST => Self::WriteBlockRequest,
            ffi::HINAWA_FW_TCODE_WRITE_RESPONSE => Self::WriteResponse,
            ffi::HINAWA_FW_TCODE_READ_QUADLET_REQUEST => Self::ReadQuadletRequest,
            ffi::HINAWA_FW_TCODE_READ_BLOCK_REQUEST => Self::ReadBlockRequest,
            ffi::HINAWA_FW_TCODE_READ_QUADLET_RESPONSE => Self::ReadQuadletResponse,
            ffi::HINAWA_FW_TCODE_READ_BLOCK_RESPONSE => Self::ReadBlockResponse,
            ffi::HINAWA_FW_TCODE_CYCLE_START => Self::CycleStart,
            ffi::HINAWA_FW_TCODE_LOCK_REQUEST => Self::LockRequest,
            ffi::HINAWA_FW_TCODE_STREAM_DATA => Self::StreamData,
            ffi::HINAWA_FW_TCODE_LOCK_RESPONSE => Self::LockResponse,
            ffi::HINAWA_FW_TCODE_LOCK_MASK_SWAP => Self::LockMaskSwap,
            ffi::HINAWA_FW_TCODE_LOCK_COMPARE_SWAP => Self::LockCompareSwap,
            ffi::HINAWA_FW_TCODE_LOCK_FETCH_ADD => Self::LockFetchAdd,
            ffi::HINAWA_FW_TCODE_LOCK_LITTLE_ADD => Self::LockLittleAdd,
            ffi::HINAWA_FW_TCODE_LOCK_BOUNDED_ADD => Self::LockBoundedAdd,
            ffi::HINAWA_FW_TCODE_LOCK_WRAP_ADD => Self::LockWrapAdd,
            ffi::HINAWA_FW_TCODE_LOCK_VENDOR_DEPENDENT => Self::LockVendorDependent,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for FwTcode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_fw_tcode_get_type()) }
    }
}

impl glib::value::ValueType for FwTcode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FwTcode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for FwTcode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaSndDiceError")]
pub enum SndDiceError {
    #[doc(alias = "HINAWA_SND_DICE_ERROR_TIMEOUT")]
    Timeout,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SndDiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SndDiceError::{}",
            match *self {
                Self::Timeout => "Timeout",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SndDiceError {
    type GlibType = ffi::HinawaSndDiceError;

    fn into_glib(self) -> ffi::HinawaSndDiceError {
        match self {
            Self::Timeout => ffi::HINAWA_SND_DICE_ERROR_TIMEOUT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaSndDiceError> for SndDiceError {
    unsafe fn from_glib(value: ffi::HinawaSndDiceError) -> Self {
        match value {
            ffi::HINAWA_SND_DICE_ERROR_TIMEOUT => Self::Timeout,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for SndDiceError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::hinawa_snd_dice_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::HINAWA_SND_DICE_ERROR_TIMEOUT => Some(Self::Timeout),
            value => Some(Self::__Unknown(value)),
        }
    }
}

impl StaticType for SndDiceError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_snd_dice_error_get_type()) }
    }
}

impl glib::value::ValueType for SndDiceError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SndDiceError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SndDiceError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaSndEfwStatus")]
pub enum SndEfwStatus {
    #[doc(alias = "HINAWA_SND_EFW_STATUS_OK")]
    Ok,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD")]
    Bad,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_COMMAND")]
    BadCommand,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_COMM_ERR")]
    CommErr,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_QUAD_COUNT")]
    BadQuadCount,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_UNSUPPORTED")]
    Unsupported,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_TIMEOUT")]
    Timeout,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_DSP_TIMEOUT")]
    DspTimeout,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_RATE")]
    BadRate,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_CLOCK")]
    BadClock,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_CHANNEL")]
    BadChannel,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_PAN")]
    BadPan,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_FLASH_BUSY")]
    FlashBusy,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_MIRROR")]
    BadMirror,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_LED")]
    BadLed,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_BAD_PARAMETER")]
    BadParameter,
    #[doc(alias = "HINAWA_SND_EFW_STATUS_LARGE_RESP")]
    LargeResp,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SndEfwStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SndEfwStatus::{}",
            match *self {
                Self::Ok => "Ok",
                Self::Bad => "Bad",
                Self::BadCommand => "BadCommand",
                Self::CommErr => "CommErr",
                Self::BadQuadCount => "BadQuadCount",
                Self::Unsupported => "Unsupported",
                Self::Timeout => "Timeout",
                Self::DspTimeout => "DspTimeout",
                Self::BadRate => "BadRate",
                Self::BadClock => "BadClock",
                Self::BadChannel => "BadChannel",
                Self::BadPan => "BadPan",
                Self::FlashBusy => "FlashBusy",
                Self::BadMirror => "BadMirror",
                Self::BadLed => "BadLed",
                Self::BadParameter => "BadParameter",
                Self::LargeResp => "LargeResp",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SndEfwStatus {
    type GlibType = ffi::HinawaSndEfwStatus;

    fn into_glib(self) -> ffi::HinawaSndEfwStatus {
        match self {
            Self::Ok => ffi::HINAWA_SND_EFW_STATUS_OK,
            Self::Bad => ffi::HINAWA_SND_EFW_STATUS_BAD,
            Self::BadCommand => ffi::HINAWA_SND_EFW_STATUS_BAD_COMMAND,
            Self::CommErr => ffi::HINAWA_SND_EFW_STATUS_COMM_ERR,
            Self::BadQuadCount => ffi::HINAWA_SND_EFW_STATUS_BAD_QUAD_COUNT,
            Self::Unsupported => ffi::HINAWA_SND_EFW_STATUS_UNSUPPORTED,
            Self::Timeout => ffi::HINAWA_SND_EFW_STATUS_TIMEOUT,
            Self::DspTimeout => ffi::HINAWA_SND_EFW_STATUS_DSP_TIMEOUT,
            Self::BadRate => ffi::HINAWA_SND_EFW_STATUS_BAD_RATE,
            Self::BadClock => ffi::HINAWA_SND_EFW_STATUS_BAD_CLOCK,
            Self::BadChannel => ffi::HINAWA_SND_EFW_STATUS_BAD_CHANNEL,
            Self::BadPan => ffi::HINAWA_SND_EFW_STATUS_BAD_PAN,
            Self::FlashBusy => ffi::HINAWA_SND_EFW_STATUS_FLASH_BUSY,
            Self::BadMirror => ffi::HINAWA_SND_EFW_STATUS_BAD_MIRROR,
            Self::BadLed => ffi::HINAWA_SND_EFW_STATUS_BAD_LED,
            Self::BadParameter => ffi::HINAWA_SND_EFW_STATUS_BAD_PARAMETER,
            Self::LargeResp => ffi::HINAWA_SND_EFW_STATUS_LARGE_RESP,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaSndEfwStatus> for SndEfwStatus {
    unsafe fn from_glib(value: ffi::HinawaSndEfwStatus) -> Self {
        match value {
            ffi::HINAWA_SND_EFW_STATUS_OK => Self::Ok,
            ffi::HINAWA_SND_EFW_STATUS_BAD => Self::Bad,
            ffi::HINAWA_SND_EFW_STATUS_BAD_COMMAND => Self::BadCommand,
            ffi::HINAWA_SND_EFW_STATUS_COMM_ERR => Self::CommErr,
            ffi::HINAWA_SND_EFW_STATUS_BAD_QUAD_COUNT => Self::BadQuadCount,
            ffi::HINAWA_SND_EFW_STATUS_UNSUPPORTED => Self::Unsupported,
            ffi::HINAWA_SND_EFW_STATUS_TIMEOUT => Self::Timeout,
            ffi::HINAWA_SND_EFW_STATUS_DSP_TIMEOUT => Self::DspTimeout,
            ffi::HINAWA_SND_EFW_STATUS_BAD_RATE => Self::BadRate,
            ffi::HINAWA_SND_EFW_STATUS_BAD_CLOCK => Self::BadClock,
            ffi::HINAWA_SND_EFW_STATUS_BAD_CHANNEL => Self::BadChannel,
            ffi::HINAWA_SND_EFW_STATUS_BAD_PAN => Self::BadPan,
            ffi::HINAWA_SND_EFW_STATUS_FLASH_BUSY => Self::FlashBusy,
            ffi::HINAWA_SND_EFW_STATUS_BAD_MIRROR => Self::BadMirror,
            ffi::HINAWA_SND_EFW_STATUS_BAD_LED => Self::BadLed,
            ffi::HINAWA_SND_EFW_STATUS_BAD_PARAMETER => Self::BadParameter,
            ffi::HINAWA_SND_EFW_STATUS_LARGE_RESP => Self::LargeResp,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for SndEfwStatus {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_snd_efw_status_get_type()) }
    }
}

impl glib::value::ValueType for SndEfwStatus {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SndEfwStatus {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SndEfwStatus {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaSndUnitError")]
pub enum SndUnitError {
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_DISCONNECTED")]
    Disconnected,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_USED")]
    Used,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_OPENED")]
    Opened,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_NOT_OPENED")]
    NotOpened,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_LOCKED")]
    Locked,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_UNLOCKED")]
    Unlocked,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_WRONG_CLASS")]
    WrongClass,
    #[doc(alias = "HINAWA_SND_UNIT_ERROR_FAILED")]
    Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SndUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SndUnitError::{}",
            match *self {
                Self::Disconnected => "Disconnected",
                Self::Used => "Used",
                Self::Opened => "Opened",
                Self::NotOpened => "NotOpened",
                Self::Locked => "Locked",
                Self::Unlocked => "Unlocked",
                Self::WrongClass => "WrongClass",
                Self::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SndUnitError {
    type GlibType = ffi::HinawaSndUnitError;

    fn into_glib(self) -> ffi::HinawaSndUnitError {
        match self {
            Self::Disconnected => ffi::HINAWA_SND_UNIT_ERROR_DISCONNECTED,
            Self::Used => ffi::HINAWA_SND_UNIT_ERROR_USED,
            Self::Opened => ffi::HINAWA_SND_UNIT_ERROR_OPENED,
            Self::NotOpened => ffi::HINAWA_SND_UNIT_ERROR_NOT_OPENED,
            Self::Locked => ffi::HINAWA_SND_UNIT_ERROR_LOCKED,
            Self::Unlocked => ffi::HINAWA_SND_UNIT_ERROR_UNLOCKED,
            Self::WrongClass => ffi::HINAWA_SND_UNIT_ERROR_WRONG_CLASS,
            Self::Failed => ffi::HINAWA_SND_UNIT_ERROR_FAILED,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaSndUnitError> for SndUnitError {
    unsafe fn from_glib(value: ffi::HinawaSndUnitError) -> Self {
        match value {
            ffi::HINAWA_SND_UNIT_ERROR_DISCONNECTED => Self::Disconnected,
            ffi::HINAWA_SND_UNIT_ERROR_USED => Self::Used,
            ffi::HINAWA_SND_UNIT_ERROR_OPENED => Self::Opened,
            ffi::HINAWA_SND_UNIT_ERROR_NOT_OPENED => Self::NotOpened,
            ffi::HINAWA_SND_UNIT_ERROR_LOCKED => Self::Locked,
            ffi::HINAWA_SND_UNIT_ERROR_UNLOCKED => Self::Unlocked,
            ffi::HINAWA_SND_UNIT_ERROR_WRONG_CLASS => Self::WrongClass,
            ffi::HINAWA_SND_UNIT_ERROR_FAILED => Self::Failed,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for SndUnitError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::hinawa_snd_unit_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::HINAWA_SND_UNIT_ERROR_DISCONNECTED => Some(Self::Disconnected),
            ffi::HINAWA_SND_UNIT_ERROR_USED => Some(Self::Used),
            ffi::HINAWA_SND_UNIT_ERROR_OPENED => Some(Self::Opened),
            ffi::HINAWA_SND_UNIT_ERROR_NOT_OPENED => Some(Self::NotOpened),
            ffi::HINAWA_SND_UNIT_ERROR_LOCKED => Some(Self::Locked),
            ffi::HINAWA_SND_UNIT_ERROR_UNLOCKED => Some(Self::Unlocked),
            ffi::HINAWA_SND_UNIT_ERROR_WRONG_CLASS => Some(Self::WrongClass),
            ffi::HINAWA_SND_UNIT_ERROR_FAILED => Some(Self::Failed),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for SndUnitError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_snd_unit_error_get_type()) }
    }
}

impl glib::value::ValueType for SndUnitError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SndUnitError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SndUnitError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HinawaSndUnitType")]
pub enum SndUnitType {
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_DICE")]
    Dice,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_FIREWORKS")]
    Fireworks,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_BEBOB")]
    Bebob,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_OXFW")]
    Oxfw,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_DIGI00X")]
    Digi00x,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_TASCAM")]
    Tascam,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_MOTU")]
    Motu,
    #[doc(alias = "HINAWA_SND_UNIT_TYPE_FIREFACE")]
    Fireface,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SndUnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SndUnitType::{}",
            match *self {
                Self::Dice => "Dice",
                Self::Fireworks => "Fireworks",
                Self::Bebob => "Bebob",
                Self::Oxfw => "Oxfw",
                Self::Digi00x => "Digi00x",
                Self::Tascam => "Tascam",
                Self::Motu => "Motu",
                Self::Fireface => "Fireface",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SndUnitType {
    type GlibType = ffi::HinawaSndUnitType;

    fn into_glib(self) -> ffi::HinawaSndUnitType {
        match self {
            Self::Dice => ffi::HINAWA_SND_UNIT_TYPE_DICE,
            Self::Fireworks => ffi::HINAWA_SND_UNIT_TYPE_FIREWORKS,
            Self::Bebob => ffi::HINAWA_SND_UNIT_TYPE_BEBOB,
            Self::Oxfw => ffi::HINAWA_SND_UNIT_TYPE_OXFW,
            Self::Digi00x => ffi::HINAWA_SND_UNIT_TYPE_DIGI00X,
            Self::Tascam => ffi::HINAWA_SND_UNIT_TYPE_TASCAM,
            Self::Motu => ffi::HINAWA_SND_UNIT_TYPE_MOTU,
            Self::Fireface => ffi::HINAWA_SND_UNIT_TYPE_FIREFACE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HinawaSndUnitType> for SndUnitType {
    unsafe fn from_glib(value: ffi::HinawaSndUnitType) -> Self {
        match value {
            ffi::HINAWA_SND_UNIT_TYPE_DICE => Self::Dice,
            ffi::HINAWA_SND_UNIT_TYPE_FIREWORKS => Self::Fireworks,
            ffi::HINAWA_SND_UNIT_TYPE_BEBOB => Self::Bebob,
            ffi::HINAWA_SND_UNIT_TYPE_OXFW => Self::Oxfw,
            ffi::HINAWA_SND_UNIT_TYPE_DIGI00X => Self::Digi00x,
            ffi::HINAWA_SND_UNIT_TYPE_TASCAM => Self::Tascam,
            ffi::HINAWA_SND_UNIT_TYPE_MOTU => Self::Motu,
            ffi::HINAWA_SND_UNIT_TYPE_FIREFACE => Self::Fireface,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for SndUnitType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::hinawa_snd_unit_type_get_type()) }
    }
}

impl glib::value::ValueType for SndUnitType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for SndUnitType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SndUnitType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
