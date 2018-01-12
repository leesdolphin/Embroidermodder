#!/bin/bash
set -ex

echo "$@"

corrode -c -m64 -pipe -std=c89 -pedantic-errors -fvisibility=hidden -O2 -fPIC -DLIBEMBROIDERY_SHARED \
    -I../libembroidery \
    -I../libembroidery-rust/target/release/includes/c-header-C89 \
    -I/usr/lib/x86_64-linux-gnu/qt5/mkspecs/linux-g++-64 \
    "format-vp3.c" || exit 0

RS_FILE="format-vp3.rs"

ls -alh

grep 'extern' $RS_FILE || exit 0
exit 1
