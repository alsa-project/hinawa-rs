================================
Rust bindings for hinawa library
================================

2022/07/08
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinawa2`` v2.5.0 or later which
  provides ``Hinawa-3.0.gir``.

  * `<https://github.com/alsa-project/libhinawa>`_

* The crates are available in `crates.io <https://crates.io/>`_ as well.

* Unfortunately, it includes no support for ``libhinawa 1`` or former (``Hinawa-2.0.gir`` or
  ``Hinawa-1.0.gir``) as well as sound unit support deprecated at libhinawa v2.5.0.

* The latest release is version 0.0.91. This is pre-release to publish crates in crates.io.

License
=======

MIT License

Dependencies
============

* `libhinawa 2.5.0 or later <https://github.com/alsa-project/libhinawa>`_
* FFI crate (``hinawa-sys``)

  * ``libc`` >= 0.2
  * ``glib-sys`` >= 0.15
  * ``gobject-sys`` >= 0.15

* API crate (`hinawa`)

  * ``libc`` >= 0.2
  * ``glib`` >= 0.15
  * FFI crate (``hinawa-sys``)

Examples
========

See ``hinawa/examples`` directory.
