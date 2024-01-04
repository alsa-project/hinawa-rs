================================
Rust bindings for hinawa library
================================

2023/11/05
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinawa`` v4.0.0 or later which
  provides ``Hinawa-4.0.gir``.

  * `<https://git.kernel.org/pub/scm/libs/ieee1394/libhinawa.git/>`_

* The crates are available in `crates.io <https://crates.io/>`_ as well.

* Inconveniently, it includes no support for ``libhinawa`` v2.6.1 or former, which provides
  ``Hinawa-3.0.gir``, ``Hinawa-2.0.gir``, and ``Hinawa-1.0.gir``.

* The latest release is version 0.9.1.

Crates
======

API bindings for safe and high-level abstractions
-------------------------------------------------

* `hinawa crate <hinawa/README.md>`_

`FFI bindings <https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages>`_
--------------------------------------------------------------------------------------------

* `hinawa-sys crate <hinawa/sys/README.md>`_

License
=======

MIT License

Dependencies
============

* `libhinawa 4.0.0 or later <https://git.kernel.org/pub/scm/libs/ieee1394/libhinawa.git/>`_
* FFI crate (``hinawa-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.15
  * ``gobject-sys`` >= 0.15

* API crate (`hinawa`)

  * ``libc`` >= 0.2
  * ``glib`` >= 0.18
  * FFI crate (``hinawa-sys``)

Examples
========

See ``hinawa/examples`` directory.
