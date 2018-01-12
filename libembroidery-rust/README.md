

Conversion notes
----------------

- A bajillion warnings about names, mutablility(and probably someone leaving the tap on in the bathroom):
    1. Ignore them.
    2. Eventually clean them up, maybe.

- Converting to `libc` types and calls.
    1. Use this regex: `::std::os::raw` -> `libc`
    2. Use this regex: ` (free|malloc)` -> `libc::$1`

- Converting struct names:
    1. use this regex for Emb\* types: `(Emb.*?)_` -> `$1`

- Converting logging:
    1. use this regex: `\(\*b"(.*)\\0"\)\.as_ptr\(\n\s+\)` -> `"$1"` 
    2. then `embLog_error\(` -> `embLog_error!(`.

- Error: `use of possibly uninitialized <name>`
    1. Find the line(usually a `_make` method).
    2. Replace the multiple assignments with returning a generated struct.
    3. Missing fields should recieve the following defaults:
        - `lineType: 0`
        - `color: EmbColor::black()`
