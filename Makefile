
QMAKE?=qmake
CARGO?=cargo

all: build test

.PHONY: prebuild build test

prebuild:
	cd libembroidery; $(QMAKE)
	cd libembroidery-convert; $(QMAKE)
	cd libembroidery-test; $(QMAKE)
	cd embroidermodder2; $(QMAKE)

build: prebuild
	$(MAKE) -C libembroidery
	$(MAKE) -C libembroidery-convert
	$(MAKE) -C libembroidery-test
	$(MAKE) -C embroidermodder2
	cd libembroidery-sys; $(CARGO) build
	cd libembroidery-rs; $(CARGO) build
	cd libembroidery-convert-rs; $(CARGO) build

test: build
	cd libembroidery-test; ./libembroidery-test
	cd libembroidery-sys; $(CARGO) test
	cd libembroidery-rs; $(CARGO) test
	cd libembroidery-convert-rs; $(CARGO) test



