use ar_compress::structures::*;
use ar_compress::structures::archive_entry;
use ar_compress::external::*;


// #[no_mangle]
// pub unsafe extern "C" fn archive_read_support_format_ar(mut _a: *mut archive) -> i32 {
//     let mut a: *mut archive_read = _a as (*mut archive_read);
//     let mut ar: *mut ar;
//     let mut r: i32;
//     let mut magic_test: i32 = __archive_check_magic(
//         _a,
//         0xdeb0c5u32,
//         1u32,
//         (*b"archive_read_support_format_ar\0").as_ptr(),
//     );
//     if magic_test == -30i32 {
//         -30i32
//     } else {
//         ar = calloc(1usize, ::std::mem::size_of::<ar>()) as (*mut ar);
//         (if ar == 0i32 as (*mut ::std::os::raw::c_void) as (*mut ar) {
//             archive_set_error(
//                 &mut (*a).archive as (*mut archive) as (*mut archive),
//                 12i32,
//                 (*b"Can\'t allocate ar data\0").as_ptr(),
//             );
//             -30i32
//         } else {
//             (*ar).strtab = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
//             r = __archive_read_register_format(
//                 a,
//                 ar as (*mut ::std::os::raw::c_void),
//                 (*b"ar\0").as_ptr(),
//                 archive_read_format_ar_bid,
//                 0i32 as (*mut ::std::os::raw::c_void)
//                     as (unsafe extern "C" fn(*mut archive_read, *const u8, *const u8) -> i32),
//                 archive_read_format_ar_read_header,
//                 archive_read_format_ar_read_data,
//                 archive_read_format_ar_skip,
//                 0i32 as (*mut ::std::os::raw::c_void)
//                     as (unsafe extern "C" fn(*mut archive_read, isize, i32) -> isize),
//                 archive_read_format_ar_cleanup,
//                 0i32 as (*mut ::std::os::raw::c_void)
//                     as (unsafe extern "C" fn(*mut archive_read) -> i32),
//                 0i32 as (*mut ::std::os::raw::c_void)
//                     as (unsafe extern "C" fn(*mut archive_read) -> i32),
//             );
//             (if r != 0i32 {
//                 free(ar as (*mut ::std::os::raw::c_void));
//                 r
//             } else {
//                 0i32
//             })
//         })
//     }
// }

unsafe extern "C" fn archive_read_format_ar_cleanup(mut a: *mut archive_read) -> i32 {
    let mut ar: *mut ar;
    ar = (*(*a).format).data as (*mut ar);
    if !(*ar).strtab.is_null() {
        free((*ar).strtab as (*mut ::std::os::raw::c_void));
    }
    free(ar as (*mut ::std::os::raw::c_void));
    (*(*a).format).data = 0i32 as (*mut ::std::os::raw::c_void);
    0i32
}

unsafe extern "C" fn archive_read_format_ar_bid(
    mut a: *mut archive_read,
    mut best_bid: i32,
) -> i32 {
    let mut h: *const ::std::os::raw::c_void;
    best_bid;
    if {
        h = __archive_read_ahead(
            a,
            8usize,
            0i32 as (*mut ::std::os::raw::c_void) as (*mut isize),
        );
        h
    } as (*mut ::std::os::raw::c_void) == 0i32 as (*mut ::std::os::raw::c_void)
    {
        -1i32
    } else if memcmp(
        h,
        (*b"!<arch>\n\0").as_ptr() as (*const ::std::os::raw::c_void),
        8usize,
    ) == 0i32
    {
        64i32
    } else {
        -1i32
    }
}

unsafe extern "C" fn _ar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut ar: *mut ar,
    mut h: *const u8,
    mut unconsumed: *mut usize,
) -> i32 {
    let mut filename: [u8; 17];
    let mut number: usize;
    let mut bsd_name_length: usize;
    let mut entry_size: usize;
    let mut p: *mut u8;
    let mut st: *mut u8;
    let mut b: *const ::std::os::raw::c_void;
    let mut r: i32;
    if strncmp(h.offset(58isize), (*b"`\n\0").as_ptr(), 2usize) != 0i32 {
        archive_set_error(
            &mut (*a).archive as (*mut archive) as (*mut archive),
            22i32,
            (*b"Incorrect file header signature\0").as_ptr(),
        );
        -30i32
    } else {
        strncpy(filename.as_mut_ptr(), h.offset(0isize), 16usize);
        filename[16usize] = b'\0';
        if (*a).archive.archive_format == 0x70000i32 {
            if strncmp(
                filename.as_mut_ptr() as (*const u8),
                (*b"#1/\0").as_ptr(),
                3usize,
            ) == 0i32
            {
                (*a).archive.archive_format = 0x70000i32 | 2i32;
            } else if strchr(filename.as_mut_ptr() as (*const u8), b'/' as (i32))
                != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
            {
                (*a).archive.archive_format = 0x70000i32 | 1i32;
            } else if strncmp(
                filename.as_mut_ptr() as (*const u8),
                (*b"__.SYMDEF\0").as_ptr(),
                9usize,
            ) == 0i32
            {
                (*a).archive.archive_format = 0x70000i32 | 2i32;
            }
        }
        if (*a).archive.archive_format == 0x70000i32 | 1i32 {
            (*a).archive.archive_format_name = (*b"ar (GNU/SVR4)\0").as_ptr();
        } else if (*a).archive.archive_format == 0x70000i32 | 2i32 {
            (*a).archive.archive_format_name = (*b"ar (BSD)\0").as_ptr();
        } else {
            (*a).archive.archive_format_name = (*b"ar\0").as_ptr();
        }
        p = filename.as_mut_ptr().offset(16isize).offset(-1isize);
        'loop14: loop {
            if !(p >= filename.as_mut_ptr() && (*p as (i32) == b' ' as (i32))) {
                break;
            }
            *p = b'\0';
            p = p.offset(-1isize);
        }
        if filename[0usize] as (i32) != b'/' as (i32) && (p > filename.as_mut_ptr())
            && (*p as (i32) == b'/' as (i32))
        {
            *p = b'\0';
        }
        (if p < filename.as_mut_ptr() {
            archive_set_error(
                &mut (*a).archive as (*mut archive) as (*mut archive),
                -1i32,
                (*b"Found entry with empty filename\0").as_ptr(),
            );
            -30i32
        } else if strcmp(filename.as_mut_ptr() as (*const u8), (*b"//\0").as_ptr()) == 0i32 {
            ar_parse_common_header(ar, entry, h);
            archive_entry_copy_pathname(entry, filename.as_mut_ptr() as (*const u8));
            archive_entry_set_filetype(entry, 0o100000u32);
            number = ar_atol10(h.offset(48isize), 10u32);
            (if number as (u64) > 18446744073709551615u64
                || number > (1024i32 * 1024i32 * 1024i32) as (usize)
            {
                archive_set_error(
                    &mut (*a).archive as (*mut archive) as (*mut archive),
                    -1i32,
                    (*b"Filename table too large\0").as_ptr(),
                );
                -30i32
            } else {
                entry_size = number;
                (if entry_size == 0usize {
                    archive_set_error(
                        &mut (*a).archive as (*mut archive) as (*mut archive),
                        22i32,
                        (*b"Invalid string table\0").as_ptr(),
                    );
                    -30i32
                } else if (*ar).strtab != 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                    archive_set_error(
                        &mut (*a).archive as (*mut archive) as (*mut archive),
                        22i32,
                        (*b"More than one string tables exist\0").as_ptr(),
                    );
                    -30i32
                } else {
                    st = malloc(entry_size) as (*mut u8);
                    (if st == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                        archive_set_error(
                            &mut (*a).archive as (*mut archive) as (*mut archive),
                            12i32,
                            (*b"Can\'t allocate filename table buffer\0").as_ptr(),
                        );
                        -30i32
                    } else {
                        (*ar).strtab = st;
                        (*ar).strtab_size = entry_size;
                        if *unconsumed != 0 {
                            __archive_read_consume(a, *unconsumed as (isize));
                            *unconsumed = 0usize;
                        }
                        (if {
                            b = __archive_read_ahead(
                                a,
                                entry_size,
                                0i32 as (*mut ::std::os::raw::c_void) as (*mut isize),
                            );
                            b
                        } as (*mut ::std::os::raw::c_void)
                            == 0i32 as (*mut ::std::os::raw::c_void)
                        {
                            -30i32
                        } else {
                            memcpy(st as (*mut ::std::os::raw::c_void), b, entry_size);
                            __archive_read_consume(a, entry_size as (isize));
                            (*ar).entry_bytes_remaining = 0isize;
                            archive_entry_set_size(entry, (*ar).entry_bytes_remaining);
                            ar_parse_gnu_filename_table(a)
                        })
                    })
                })
            })
        } else if filename[0usize] as (i32) == b'/' as (i32)
            && (filename[1usize] as (i32) >= b'0' as (i32))
            && (filename[1usize] as (i32) <= b'9' as (i32))
        {
            number = ar_atol10(h.offset(0isize).offset(1isize), (16i32 - 1i32) as (u32));
            (if (*ar).strtab == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                || number >= (*ar).strtab_size
            {
                archive_set_error(
                    &mut (*a).archive as (*mut archive) as (*mut archive),
                    22i32,
                    (*b"Can\'t find long filename for GNU/SVR4 archive entry\0").as_ptr(),
                );
                archive_entry_copy_pathname(entry, filename.as_mut_ptr() as (*const u8));
                ar_parse_common_header(ar, entry, h);
                -30i32
            } else {
                archive_entry_copy_pathname(
                    entry,
                    &mut *(*ar).strtab.offset(number as (isize)) as (*mut u8) as (*const u8),
                );
                ar_parse_common_header(ar, entry, h)
            })
        } else if strncmp(
            filename.as_mut_ptr() as (*const u8),
            (*b"#1/\0").as_ptr(),
            3usize,
        ) == 0i32
        {
            ar_parse_common_header(ar, entry, h);
            number = ar_atol10(h.offset(0isize).offset(3isize), (16i32 - 3i32) as (u32));
            (if number as (u64) > 18446744073709551615u64.wrapping_sub(1u64)
                || number > (1024i32 * 1024i32) as (usize)
                || number as (isize) > (*ar).entry_bytes_remaining
            {
                archive_set_error(
                    &mut (*a).archive as (*mut archive) as (*mut archive),
                    -1i32,
                    (*b"Bad input file size\0").as_ptr(),
                );
                -30i32
            } else {
                bsd_name_length = number;
                (*ar).entry_bytes_remaining = ((*ar).entry_bytes_remaining as (usize))
                    .wrapping_sub(bsd_name_length)
                    as (isize);
                archive_entry_set_size(entry, (*ar).entry_bytes_remaining);
                if *unconsumed != 0 {
                    __archive_read_consume(a, *unconsumed as (isize));
                    *unconsumed = 0usize;
                }
                (if {
                    b = __archive_read_ahead(
                        a,
                        bsd_name_length,
                        0i32 as (*mut ::std::os::raw::c_void) as (*mut isize),
                    );
                    b
                } as (*mut ::std::os::raw::c_void)
                    == 0i32 as (*mut ::std::os::raw::c_void)
                {
                    archive_set_error(
                        &mut (*a).archive as (*mut archive) as (*mut archive),
                        -1i32,
                        (*b"Truncated input file\0").as_ptr(),
                    );
                    -30i32
                } else {
                    p = malloc(bsd_name_length.wrapping_add(1usize)) as (*mut u8);
                    (if p == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
                        archive_set_error(
                            &mut (*a).archive as (*mut archive) as (*mut archive),
                            12i32,
                            (*b"Can\'t allocate fname buffer\0").as_ptr(),
                        );
                        -30i32
                    } else {
                        strncpy(p, b as (*const u8), bsd_name_length);
                        *p.offset(bsd_name_length as (isize)) = b'\0';
                        __archive_read_consume(a, bsd_name_length as (isize));
                        archive_entry_copy_pathname(entry, p as (*const u8));
                        free(p as (*mut ::std::os::raw::c_void));
                        0i32
                    })
                })
            })
        } else if strcmp(filename.as_mut_ptr() as (*const u8), (*b"/\0").as_ptr()) == 0i32 {
            archive_entry_copy_pathname(entry, (*b"/\0").as_ptr());
            r = ar_parse_common_header(ar, entry, h);
            archive_entry_set_filetype(entry, 0o100000u32);
            r
        } else if strcmp(
            filename.as_mut_ptr() as (*const u8),
            (*b"__.SYMDEF\0").as_ptr(),
        ) == 0i32
        {
            archive_entry_copy_pathname(entry, filename.as_mut_ptr() as (*const u8));
            ar_parse_common_header(ar, entry, h)
        } else {
            archive_entry_copy_pathname(entry, filename.as_mut_ptr() as (*const u8));
            ar_parse_common_header(ar, entry, h)
        })
    }
}

unsafe extern "C" fn archive_read_format_ar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> i32 {
    let mut ar: *mut ar = (*(*a).format).data as (*mut ar);
    let mut unconsumed: usize;
    let mut header_data: *const ::std::os::raw::c_void;
    let mut ret: i32;
    if (*ar).read_global_header == 0 {
        __archive_read_consume(a, 8isize);
        (*ar).read_global_header = 1u8;
        (*a).archive.archive_format = 0x70000i32;
    }
    if {
        header_data = __archive_read_ahead(
            a,
            60usize,
            0i32 as (*mut ::std::os::raw::c_void) as (*mut isize),
        );
        header_data
    } as (*mut ::std::os::raw::c_void) == 0i32 as (*mut ::std::os::raw::c_void)
    {
        1i32
    } else {
        unconsumed = 60usize;
        ret = _ar_read_header(
            a,
            entry,
            ar,
            header_data as (*const u8),
            &mut unconsumed as (*mut usize),
        );
        if unconsumed != 0 {
            __archive_read_consume(a, unconsumed as (isize));
        }
        ret
    }
}

unsafe extern "C" fn ar_parse_common_header(
    mut ar: *mut ar,
    mut entry: *mut archive_entry,
    mut h: *const u8,
) -> i32 {
    let mut n: usize;
    archive_entry_set_mtime(
        entry,
        ar_atol10(h.offset(16isize), 12u32) as (isize),
        0isize,
    );
    archive_entry_set_uid(
        entry,
        ar_atol10(h.offset(28isize), 6u32) as (u32) as (isize),
    );
    archive_entry_set_gid(
        entry,
        ar_atol10(h.offset(34isize), 6u32) as (u32) as (isize),
    );
    archive_entry_set_mode(entry, ar_atol8(h.offset(40isize), 8u32) as (u32));
    n = ar_atol10(h.offset(48isize), 10u32);
    (*ar).entry_offset = 0isize;
    (*ar).entry_padding = n.wrapping_rem(2usize) as (isize);
    archive_entry_set_size(entry, n as (isize));
    (*ar).entry_bytes_remaining = n as (isize);
    0i32
}

unsafe extern "C" fn archive_read_format_ar_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const ::std::os::raw::c_void,
    mut size: *mut usize,
    mut offset: *mut isize,
) -> i32 {
    let mut bytes_read: isize;
    let mut ar: *mut ar;
    ar = (*(*a).format).data as (*mut ar);
    if (*ar).entry_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*ar).entry_bytes_unconsumed as (isize));
        (*ar).entry_bytes_unconsumed = 0usize;
    }
    if (*ar).entry_bytes_remaining > 0isize {
        *buff = __archive_read_ahead(a, 1usize, &mut bytes_read as (*mut isize));
        (if bytes_read == 0isize {
            archive_set_error(
                &mut (*a).archive as (*mut archive) as (*mut archive),
                -1i32,
                (*b"Truncated ar archive\0").as_ptr(),
            );
            -30i32
        } else if bytes_read < 0isize {
            -30i32
        } else {
            if bytes_read > (*ar).entry_bytes_remaining {
                bytes_read = (*ar).entry_bytes_remaining;
            }
            *size = bytes_read as (usize);
            (*ar).entry_bytes_unconsumed = bytes_read as (usize);
            *offset = (*ar).entry_offset;
            (*ar).entry_offset = (*ar).entry_offset + bytes_read;
            (*ar).entry_bytes_remaining = (*ar).entry_bytes_remaining - bytes_read;
            0i32
        })
    } else {
        let mut skipped: isize = __archive_read_consume(a, (*ar).entry_padding);
        if skipped >= 0isize {
            (*ar).entry_padding = (*ar).entry_padding - skipped;
        }
        (if (*ar).entry_padding != 0 {
            if skipped >= 0isize {
                archive_set_error(
                    &mut (*a).archive as (*mut archive) as (*mut archive),
                    -1i32,
                    (*b"Truncated ar archive- failed consuming padding\0").as_ptr(),
                );
            }
            -30i32
        } else {
            *buff = 0i32 as (*mut ::std::os::raw::c_void) as (*const ::std::os::raw::c_void);
            *size = 0usize;
            *offset = (*ar).entry_offset;
            1i32
        })
    }
}

unsafe extern "C" fn archive_read_format_ar_skip(mut a: *mut archive_read) -> i32 {
    let mut bytes_skipped: isize;
    let mut ar: *mut ar;
    ar = (*(*a).format).data as (*mut ar);
    bytes_skipped = __archive_read_consume(
        a,
        (((*ar).entry_bytes_remaining + (*ar).entry_padding) as (usize))
            .wrapping_add((*ar).entry_bytes_unconsumed) as (isize),
    );
    if bytes_skipped < 0isize {
        -30i32
    } else {
        (*ar).entry_bytes_remaining = 0isize;
        (*ar).entry_bytes_unconsumed = 0usize;
        (*ar).entry_padding = 0isize;
        0i32
    }
}

unsafe extern "C" fn ar_parse_gnu_filename_table(mut a: *mut archive_read) -> i32 {
    let mut _currentBlock;
    let mut ar: *mut ar;
    let mut p: *mut u8;
    let mut size: usize;
    ar = (*(*a).format).data as (*mut ar);
    size = (*ar).strtab_size;
    p = (*ar).strtab;
    'loop1: loop {
        if !(p < (*ar).strtab.offset(size as (isize)).offset(-1isize)) {
            _currentBlock = 2;
            break;
        }
        if *p as (i32) == b'/' as (i32) {
            *{
                let _old = p;
                p = p.offset(1isize);
                _old
            } = b'\0';
            if *p as (i32) != b'\n' as (i32) {
                _currentBlock = 8;
                break;
            }
            *p = b'\0';
        }
        p = p.offset(1isize);
    }
    if _currentBlock == 2 {
        if !(p != (*ar).strtab.offset(size as (isize)) && (*p as (i32) != b'\n' as (i32))
            && (*p as (i32) != b'`' as (i32)))
        {
            *(*ar).strtab.offset(size.wrapping_sub(1usize) as (isize)) = b'\0';
            return 0i32;
        }
    }
    archive_set_error(
        &mut (*a).archive as (*mut archive) as (*mut archive),
        22i32,
        (*b"Invalid string table\0").as_ptr(),
    );
    free((*ar).strtab as (*mut ::std::os::raw::c_void));
    (*ar).strtab = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    -30i32
}
