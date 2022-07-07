// SPDX-License-Identifier: MIT
use glib::*;
use hinawa::{prelude::*, *};

const PATH: &str = "/dev/fw1";
const OFFSET: u64 = 0xfffff0000404;

fn main() {
    let node = FwNode::new();
    node.open(PATH).unwrap();

    let ctx = MainContext::new();
    let src = node.create_source().unwrap();
    src.attach(Some(&ctx));

    let dispatcher = MainLoop::new(Some(&ctx), false);
    let cntr = std::sync::Arc::new(dispatcher);
    let d = cntr.clone();
    let th = std::thread::spawn(move || {
        d.run();
        ()
    });

    let req = hinawa::FwReq::new();
    let mut frames = [0; 4];
    req.transaction(&node, FwTcode::ReadQuadletRequest, OFFSET, 4, &mut frames)
        .unwrap();

    assert_eq!(0x31333934, u32::from_be_bytes(frames));

    cntr.quit();
    th.join().unwrap();
}
