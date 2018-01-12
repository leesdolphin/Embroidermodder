
# Define a path to cargo; can be overwritten
CARGO="cargo"

debug:RUST_BUILD_MODE="debug"
!debug:RUST_BUILD_MODE="release"

!debug:RUST_BUILD_FLAGS="--release"

RUST_BUILD_PATH=../libembroidery-rust/target/$$RUST_BUILD_MODE

# Custom QMake target to run `cargo build`.
pattern.h.depends = build_rust
build_rust.commands = cd ../libembroidery-rust && $$CARGO build --lib $$RUST_BUILD_FLAGS

QMAKE_EXTRA_TARGETS += build_rust pattern.h
PRE_TARGETDEPS += build_rust


INCLUDEPATH += $$RUST_BUILD_PATH/includes/c-header-C89
LIBS += "-L$$RUST_BUILD_PATH/" -lembroidery_rust

# These are printed from the Cargo build step.
LIBS += \
    -lutil \
    -lpthread \
    -ldl \

# Make C consistent with Rust's interpretation of a character type.
QMAKE_CFLAGS += \
    -funsigned-char \
