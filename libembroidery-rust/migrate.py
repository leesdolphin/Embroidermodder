#!/usr/bin/env python3

import re
import pathlib
import sys


STD_OS_RAW_RE = re.compile(r'::std::os::raw')
STD_OS_RAW_RE = re.compile(r'::std::os::raw')
STD_OS_RAW_RE = re.compile(r'::std::os::raw')
STD_OS_RAW_RE = re.compile(r'::std::os::raw')


def convert_to_libc(contents):
    contents = re.sub(r'::std::os::raw::', r'libc::', contents)
    contents = re.sub(r' (free|malloc)', r' libc::\1', contents)
    return contents


def convert_struct_names(contents):
    contents = re.sub(r'(Emb.*?)_', r'\1', contents)
    return contents


def convert_logging(contents):
    contents = re.sub(r'\(\*b"(.*)\\0"\)\.as_ptr\(\n\s +\)', r'"\1"', contents)
    contents = re.sub(r'embLog_error\(', r'embLog_error!(', contents)


def main():
    file = pathlib.Path(sys.argv[1])
    output = pathlib.Path(sys.argv[2])
    contents = file.read_text()
    contents = convert_to_libc(contents)
    contents = convert_struct_names(contents)
    contents = convert_logging(contents)
    output.write_text(contents)


if __name__ == '__main__':
    main()

