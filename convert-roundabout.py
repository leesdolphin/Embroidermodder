#!/bin/env python3

import subprocess
import os
import pathlib
import shutil
import sys


ALL_MULTIWAY_FORMATS = [
    'col',
    'csv',
    'dst',
    'edr',
    'exp',
    'hus',
    'inf',
    'jef',
    'ksm',
    'pec',
    'pes',
    'rgb',
    'sew',
    'svg',
    'tap',
    'thr',
]


BASE_DIR = pathlib.Path(os.path.dirname(os.path.realpath(__file__)))
EMBROIDERY_DIR = BASE_DIR / 'libembroidery'
CONVERT_DIR = BASE_DIR / 'libembroidery-convert'
CONVERT_EXE = CONVERT_DIR / 'libembroidery-convert'

def build():
    for directory in [EMBROIDERY_DIR, CONVERT_DIR]:
        subprocess.run(
            ['qmake'],
            cwd=str(directory),
            check=True
        )
        subprocess.run(
            ['make', '-j4'],
            cwd=str(directory),
            check=True
        )


def convert(input, output):
    if output.exists():
        output.unlink()
    subprocess.run(
        [str(CONVERT_EXE), str(input), str(output)],
        check=True,
    )
    if not output.exists():
        raise ValueError(f"Generation of file {output} failed")


def convert_to_text(input):
    output = input.with_suffix('.txt')
    return convert(input, output)


def diff_text(prev, curr, diff=None):
    prev = prev.with_suffix('.txt')
    curr = curr.with_suffix('.txt')
    if diff is None:
        diff = curr.with_suffix('.diff')
    if diff.exists():
        diff.unlink()
    subprocess.run(
        ['diff', '--expand-tabs', '--report-identical-files',
            '--side-by-side', '--ignore-all-space', str(prev), str(curr)],
        universal_newlines=True,
        stdout=diff.open('w'),
        stderr=subprocess.STDOUT
    )


def roundabout(first, out_folder, all_formats):
    prev = first
    for idx, fmt in enumerate(all_formats, start=1):
        curr = out_folder / f'{idx:04}.{fmt}'
        print(f"\n\nConverting from {prev.name} to {curr.name}\n")
        convert(prev, curr)
        convert_to_text(curr)
        diff_text(prev, curr)
        diff_text(first, curr, diff=curr.with_name(f'0000-{idx:04}.diff'))
        prev = curr


def main():
    build()
    start = pathlib.Path(sys.argv[1]).resolve()
    name = start.stem
    start_fmt = start.suffix[1:]
    out_folder = start.parent / 'roundabout-convert' / name
    out_folder.mkdir(parents=True, exist_ok=True)

    first = out_folder / f'0000.{start_fmt}'
    shutil.copyfile(str(start), first)
    convert_to_text(first)
    all_formats = [] + sys.argv[2:]
    try:
        roundabout(first, out_folder, all_formats)
    except:
        pass
    for fmt in ALL_MULTIWAY_FORMATS:
        curr = out_folder / f'conversion-{start_fmt}-{fmt}.{fmt}'
        print(f"\n\nConverting from {first.name} to {curr.name}\n")
        try:
            convert(first, curr)
            convert_to_text(curr)
            diff_text(first, curr)
            diff_text(first, curr, diff=curr.with_name(f'conversion-{start_fmt}-{fmt}.diff'))
        except:
            pass




if __name__ == '__main__':
    main()
