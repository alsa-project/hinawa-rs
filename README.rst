====================
hinawa Rust bindings
====================

2022/03/17
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinawa2`` which provides ``Hinawa-3.0.gir``.

  * `<https://github.com/alsa-project/libhinawa>`_

* Unfortunately, it includes no support for ``libhinawa 1`` or former (``Hinawa-2.0.gir`` or ``Hinawa-1.0.gir``).

License
=======

MIT License

Dependencies
============

* Rust version 1.57 or later (edition 2021)
* `libhinawa <https://github.com/alsa-project/libhinawa>`_

Sample code
===========

Read 1 quadlet from address ``0xfffff0000404`` on the node represented as ``/dev/fw0`` ::

    extern crate hinawa;
    extern crate glib;
    
    use hinawa::FwNodeExt;
    use hinawa::FwReqExtManual;
    
    
    fn main() {
        let node = hinawa::FwNode::new();
        node.open("/dev/fw0").unwrap();
    
        let ctx = glib::MainContext::new();
        let src = node.create_source().unwrap();
        src.attach(Some(&ctx));
    
        let dispatcher = glib::MainLoop::new(Some(&ctx), false);
        let cntr = std::sync::Arc::new(dispatcher);
        let d = cntr.clone();
        let th = std::thread::spawn(move|| {
            d.run();
            ()
        });
    
        let req = hinawa::FwReq::new();
        let mut frames = [0;4];
        req.transaction(&node, FwTcode::ReadQuadletRequest, 0xfffff0000404, 4,
                        &mut frames).unwrap();
    
        assert_eq!(0x31333934, u32::from_be_bytes(frames));
    
        cntr.quit();
        th.join().unwrap();
    }

How to generate FFI and API crates
==================================

::

    $ ./generator.py

end
