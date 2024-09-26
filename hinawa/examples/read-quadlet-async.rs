// SPDX-License-Identifier: MIT
use glib::{prelude::*, *};
use hinawa::{prelude::*, *};

use std::{
    cell::RefCell,
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration,
};
use std::rc::Rc;

const PATH: &str = "/dev/fw1";
const OFFSET: u64 = 0xfffff0000404;
const LENGTH: usize = 4;

struct TransactionData {
    rcode: FwRcode,
    request_tstamp: u32,
    response_tstamp: u32,
    payload: Vec<u8>,
}

impl Default for TransactionData {
    fn default() -> Self {
        Self {
            rcode: FwRcode::Invalid,
            request_tstamp: Default::default(),
            response_tstamp: Default::default(),
            payload: Default::default(),
        }
    }
}

impl TransactionData {
    fn new(length: usize) -> Self {
        let mut data = Self::default();
        data.payload.resize(length, 0);
        data
    }
}

struct TransactionState(RefCell<(bool, TransactionData, Option<Waker>)>);

impl TransactionState {
    fn new(data: TransactionData) -> Self {
        Self(RefCell::new((false, data, None)))
    }
}

impl Future for &TransactionState {
    type Output = TransactionData;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        // The borrow checker detects no issue since a single GLib::MainContext executes both the
        // call and the resopnded closure sequentially. Nevertheless just for safe.
        if let Ok(mut data) = self.0.try_borrow_mut() {
            if data.0 {
                // Reduce duplication overhead as possible.
                Poll::Ready(mem::take(&mut data.1))
            } else {
                data.2 = Some(ctx.waker().clone());
                Poll::Pending
            }
        } else {
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn rcode_to_glib_error(rcode: &FwRcode) -> glib::Error {
    let (error, label) = match rcode {
        FwRcode::ConflictError => (FwReqError::ConflictError, "conflict error"),
        FwRcode::DataError => (FwReqError::DataError, "data error"),
        FwRcode::TypeError => (FwReqError::TypeError, "type error"),
        FwRcode::AddressError => (FwReqError::AddressError, "address error"),
        FwRcode::SendError => (FwReqError::SendError, "send error"),
        FwRcode::Cancelled => (FwReqError::Cancelled, "timeout"),
        FwRcode::Busy => (FwReqError::Busy, "busy"),
        FwRcode::Generation => (FwReqError::Generation, "bus reset"),
        FwRcode::NoAck => (FwReqError::NoAck, "no ack"),
        FwRcode::Invalid | _ => (FwReqError::Invalid, "invalid"),
    };

    glib::Error::new(error, label)
}

async fn transaction_with_tstamp_async<P: IsA<FwNode>>(
    req: &FwReq,
    node: &P,
    tcode: FwTcode,
    addr: u64,
    length: usize,
    frame: &mut [u8],
    timeout_ms: u32,
) -> Result<[u32; 2], glib::Error> {
    let data = TransactionData::new(length);
    let state = Rc::new(TransactionState::new(data));

    let s = state.clone();
    let handler_id = req.connect_responded(
        move |_req, rcode, request_tstamp, response_tstamp, payload| {
            // The borrow checker detects no issue since a single GLib::MainContext executes both the
            // call and Future::poll() sequentially. Nevertheless just for safe.
            if let Ok(mut data) = s.0.try_borrow_mut() {
                data.0 = true;
                data.1.rcode = rcode;
                data.1.request_tstamp = request_tstamp;
                data.1.response_tstamp = response_tstamp;
                data.1.payload.copy_from_slice(payload);

                if let Some(waker) = &data.2 {
                    waker.wake_by_ref();
                }
            }
        },
    );

    let future =
        glib::future_with_timeout(Duration::from_millis(timeout_ms.into()), state.as_ref());
    req.request(node, tcode, addr, length, &mut vec![0; length])?;
    let result = future.await;
    req.disconnect(handler_id);

    result
        .map_err(|timeout_error| {
            let msg = format!("{}", timeout_error);
            glib::Error::new(FwReqError::Cancelled, &msg)
        })
        .and_then(|data| {
            if data.rcode == FwRcode::Complete {
                frame.copy_from_slice(&data.payload);
                Ok([data.request_tstamp, data.response_tstamp])
            } else {
                Err(rcode_to_glib_error(&data.rcode))
            }
        })
}

async fn async_main(ctx: &MainContext, node: &FwNode) -> Result<(), glib::Error> {
    let src = node.create_source()?;
    let _ = src.attach(Some(&ctx));

    let req = FwReq::new();
    let mut frame = [0; LENGTH];

    let _tstamps = transaction_with_tstamp_async(
        &req,
        node,
        FwTcode::ReadQuadletRequest,
        OFFSET,
        LENGTH,
        &mut frame,
        100,
    )
    .await?;

    assert_eq!(0x31333934, u32::from_be_bytes(frame));

    Ok(())
}

fn main() {
    let node = FwNode::new();
    node.open(PATH, 0).unwrap();

    let ctx = MainContext::default();

    ctx.block_on(async_main(&ctx, &node)).unwrap()
}
