
all: build test

.PHONY: build-libembroidery build-libembroidery-convert build-libembroidery-test build-embroidermodder2

prebuild:
	cd libembroidery; qmake
	cd libembroidery-convert; qmake
	cd libembroidery-test; qmake
	cd embroidermodder2; qmake

build: prebuild
	$(MAKE) -C libembroidery
	$(MAKE) -C libembroidery-convert
	$(MAKE) -C libembroidery-test
	$(MAKE) -C embroidermodder2
	cd libembroidery-sys; cargo build
	cd libembroidery-rs; cargo build
	cd libembroidery-convert-rs; cargo build

test: build
	cd libembroidery-test; ./libembroidery-test
	cd libembroidery-sys; cargo test
	cd libembroidery-rs; cargo test
	cd libembroidery-convert-rs; cargo test



