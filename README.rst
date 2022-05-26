====================
hinawa Rust bindings
====================

2022/05/26
Takashi Sakamoto

Introduction
============

* This repository includes FFI and API bindings for ``libhinawa2`` v2.5.0 or later which
  provides ``Hinawa-3.0.gir``.

  * `<https://github.com/alsa-project/libhinawa>`_

* Unfortunately, it includes no support for ``libhinawa 1`` or former (``Hinawa-2.0.gir`` or
  ``Hinawa-1.0.gir``) as well as sound unit support deprecated at libhinawa v2.5.0.

License
=======

MIT License

Dependencies
============

* Rust version 1.57 or later (edition 2021)
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

How to generate FFI and API crates
==================================

::

    $ ./generator.py

end
