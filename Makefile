#!/usr/bin/make -f

GIR_EXEC = gir/target/release/gir

.PHONY: all clean test

all: hinawa-sys hinawa

clean:
	rm -rf gir-files/Hinawa-3.0.gir
	rm -rf hinawa-sys
	rm -rf hinawa/src/auto hinawa/target hinawa/Cargo.lock

gir/Cargo.toml:
	git submodule update --init gir

$(GIR_EXEC): gir/Cargo.toml
	cd gir && cargo build --release

gir-files/GLib-2.0.toml:
	git submodule update --init gir-files

gir-files/Hinawa-3.0.gir: Hinawa-3.0.gir gir-files/GLib-2.0.toml
	cp Hinawa-3.0.gir gir-files/Hinawa-3.0.gir

hinawa-sys/src: conf/gir-hinawa-sys.toml gir-files/Hinawa-3.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-hinawa-sys.toml -d gir-files -m sys -o hinawa-sys

hinawa-sys: hinawa-sys/src

hinawa/src/auto: conf/gir-hinawa.toml gir-files/Hinawa-3.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c conf/gir-hinawa.toml -d gir-files -m normal -o hinawa

hinawa: hinawa-sys hinawa/src/auto
