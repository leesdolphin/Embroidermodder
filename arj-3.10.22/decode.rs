extern {
    fn _setjmp(env : *mut ::std::os::raw::c_void) -> i32;
    static mut aistream : *mut _IO_FILE;
    fn arj_delay(seconds : u32);
    static mut bitbuf : u16;
    static mut bitcount : i32;
    static mut byte_buf : u8;
    static mut c_len : [u8; 510];
    fn calloc(
        __nmemb : usize, __size : usize
    ) -> *mut ::std::os::raw::c_void;
    static mut compsize : usize;
    static mut dec_text : *mut u8;
    fn decode_end_stub();
    fn decode_start_stub();
    fn display_indicator(bytes : isize);
    fn error_proc(errmsg : *mut u8, ...) -> i32;
    fn extraction_stub(
        block : *mut u8, block_len : i32, action : i32
    ) -> i32;
    fn fgetc(__stream : *mut _IO_FILE) -> i32;
    static mut file_garbled : i32;
    static mut file_packing : i32;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn garble_decode(data : *mut u8, len : i32);
    static mut left : [u16; 1019];
    fn longjmp(env : *mut ::std::os::raw::c_void, value : i32) -> i32;
    fn malloc_msg(size : u32) -> *mut ::std::os::raw::c_void;
    fn msg_cprintf(ccode : i32, fmt : *mut u8, ...) -> i32;
    static mut ntext : *mut u8;
    static mut origsize : usize;
    static mut packblock_ptr : *mut u8;
    static mut packmem_remain : u32;
    static mut pt_len : [u8; 19];
    static mut right : [u16; 1019];
}

enum _IO_FILE {
}

static mut decode_proc
    : *mut ::std::os::raw::c_void
    = 0 as (*mut ::std::os::raw::c_void);

#[no_mangle]
pub static mut c_table : *mut u16 = 0 as (*mut u16);

#[no_mangle]
pub static mut pt_table : *mut u16 = 0 as (*mut u16);

#[no_mangle]
pub static mut blocksize : i16 = 0i16;

static mut count : isize = 0isize;

#[no_mangle]
pub unsafe extern fn fillbuf(mut n : i32) {
    'loop0: loop {
        if !(bitcount < n) {
            break;
        }
        bitbuf = ((bitbuf as (i32) << bitcount) as (u32) | byte_buf as (u32) >> 8i32 - bitcount) as (u16);
        n = n - bitcount;
        if compsize > 0usize {
            compsize = compsize.wrapping_sub(1usize);
            if file_packing != 0 {
                byte_buf = fgetc(aistream) as (u8);
            } else {
                byte_buf = *{
                                let _old = packblock_ptr;
                                packblock_ptr = packblock_ptr.offset(1isize);
                                _old
                            };
                packmem_remain = packmem_remain.wrapping_sub(1u32);
            }
            if file_garbled != 0 {
                garble_decode(&mut byte_buf as (*mut u8),1i32);
            }
        } else {
            byte_buf = 0u8;
        }
        bitcount = 8i32;
    }
    bitcount = bitcount - n;
    bitbuf = (bitbuf as (i32) << n | byte_buf as (i32) >> 8i32 - n) as (u16);
    byte_buf = (byte_buf as (i32) << n) as (u8);
}

unsafe extern fn getbits(mut n : i32) -> i32 {
    let mut rc : i32;
    rc = bitbuf as (i32) >> 16i32 - n;
    fillbuf(n);
    rc
}

unsafe extern fn make_table(
    mut nchar : i32,
    mut bitlen : *mut u8,
    mut tablebits : i32,
    mut table : *mut u16,
    mut tablesize : i32
) {
    let mut count : [u16; 17];
    let mut weight : [u16; 17];
    let mut start : [u16; 18];
    let mut p : *mut u16;
    let mut i : u32;
    let mut k : u32;
    let mut len : u32;
    let mut ch : u32;
    let mut jutbits : u32;
    let mut avail : u32;
    let mut nextcode : u32;
    let mut mask : u32;
    i = 1u32;
    'loop1: loop {
        if !(i <= 16u32) {
            break;
        }
        count[i as (usize)] = 0u16;
        i = i.wrapping_add(1u32);
    }
    i = 0u32;
    'loop3: loop {
        if !(i as (i32) < nchar) {
            break;
        }
        let _rhs = 1;
        let _lhs = &mut count[*bitlen.offset(i as (isize)) as (usize)];
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
        i = i.wrapping_add(1u32);
    }
    start[1usize] = 0u16;
    i = 1u32;
    'loop5: loop {
        if !(i <= 16u32) {
            break;
        }
        start[i.wrapping_add(1u32) as (usize)] = (start[
                                                      i as (usize)
                                                  ] as (i32) + (count[
                                                                    i as (usize)
                                                                ] as (i32) << 16u32.wrapping_sub(
                                                                                  i
                                                                              ))) as (u16);
        i = i.wrapping_add(1u32);
    }
    if start[17usize] as (i32) != (1i32 << 16i32) as (u16) as (i32) {
        if file_garbled != 0 {
            arj_delay(2u32);
            msg_cprintf(4i32 | 0x40i32,(*b"\0").as_ptr() as (*mut u8));
        } else {
            msg_cprintf(4i32 | 0x40i32,(*b"\0").as_ptr() as (*mut u8));
        }
        longjmp(decode_proc,1i32);
    }
    jutbits = (16i32 - tablebits) as (u32);
    i = 1u32;
    'loop12: loop {
        if !(i as (i32) <= tablebits) {
            break;
        }
        let _rhs = jutbits;
        let _lhs = &mut start[i as (usize)];
        *_lhs = (*_lhs as (i32) >> _rhs) as (u16);
        weight[i as (usize)] = (1i32 << (tablebits as (u32)).wrapping_sub(
                                            i
                                        )) as (u16);
        i = i.wrapping_add(1u32);
    }
    'loop13: loop {
        if !(i <= 16u32) {
            break;
        }
        weight[i as (usize)] = (1i32 << 16u32.wrapping_sub(i)) as (u16);
        i = i.wrapping_add(1u32);
    }
    i = (start[
             (tablebits + 1i32) as (usize)
         ] as (i32) >> jutbits) as (u32);
    if i != (1i32 << 16i32) as (u16) as (u32) {
        k = (1i32 << tablebits) as (u32);
        'loop16: loop {
            if !(i != k) {
                break;
            }
            *table.offset(
                 {
                     let _old = i;
                     i = i.wrapping_add(1u32);
                     _old
                 } as (isize)
             ) = 0u16;
        }
    }
    avail = nchar as (u32);
    mask = (1i32 << 15i32 - tablebits) as (u32);
    ch = 0u32;
    'loop18: loop {
        if !(ch as (i32) < nchar) {
            break;
        }
        if {
               len = *bitlen.offset(ch as (isize)) as (u32);
               len
           } != 0u32 {
            k = start[len as (usize)] as (u32);
            nextcode = k.wrapping_add(weight[len as (usize)] as (u32));
            if len as (i32) <= tablebits {
                if nextcode > tablesize as (u32) {
                    if file_garbled != 0 {
                        arj_delay(2u32);
                        msg_cprintf(4i32 | 0x40i32,(*b"\0").as_ptr() as (*mut u8));
                    } else {
                        msg_cprintf(4i32 | 0x40i32,(*b"\0").as_ptr() as (*mut u8));
                    }
                    longjmp(decode_proc,1i32);
                }
                i = start[len as (usize)] as (u32);
                'loop37: loop {
                    if !(i < nextcode) {
                        break;
                    }
                    *table.offset(i as (isize)) = ch as (u16);
                    i = i.wrapping_add(1u32);
                }
            } else {
                p = &mut *table.offset((k >> jutbits) as (isize)) as (*mut u16);
                i = len.wrapping_sub(tablebits as (u32));
                'loop23: loop {
                    if !(i != 0u32) {
                        break;
                    }
                    if *p as (i32) == 0i32 {
                        right[avail as (usize)] = {
                                                      let _rhs = 0i32;
                                                      let _lhs = &mut left[avail as (usize)];
                                                      *_lhs = _rhs as (u16);
                                                      *_lhs
                                                  };
                        *p = avail as (u16);
                        avail = avail.wrapping_add(1u32);
                    }
                    if k & mask != 0 {
                        p = &mut right[*p as (usize)] as (*mut u16);
                    } else {
                        p = &mut left[*p as (usize)] as (*mut u16);
                    }
                    k = k << 1i32;
                    i = i.wrapping_sub(1u32);
                }
                *p = ch as (u16);
            }
            start[len as (usize)] = nextcode as (u16);
        }
        ch = ch.wrapping_add(1u32);
    }
}

#[no_mangle]
pub unsafe extern fn read_pt_len(
    mut nn : i32, mut nbit : i32, mut i_special : i32
) {
    let mut i : i32;
    let mut n : i32;
    let mut c : i16;
    let mut mask : u16;
    n = getbits(nbit);
    if n == 0i32 {
        c = getbits(nbit) as (i16);
        i = 0i32;
        'loop16: loop {
            if !(i < nn) {
                break;
            }
            pt_len[i as (usize)] = 0u8;
            i = i + 1;
        }
        i = 0i32;
        'loop18: loop {
            if !(i < 256i32) {
                break;
            }
            *pt_table.offset(i as (isize)) = c as (u16);
            i = i + 1;
        }
    } else {
        i = 0i32;
        if n >= 16i32 + 3i32 {
            n = 16i32 + 3i32;
        }
        'loop3: loop {
            if !(i < n) {
                break;
            }
            c = (bitbuf as (i32) >> 13i32) as (i16);
            if c as (i32) == 7i32 {
                mask = (1i32 << 12i32) as (u16);
                'loop9: loop {
                    if mask as (i32) & bitbuf as (i32) == 0 {
                        break;
                    }
                    mask = (mask as (i32) >> 1i32) as (u16);
                    c = (c as (i32) + 1) as (i16);
                }
            }
            fillbuf(if c as (i32) < 7i32 { 3i32 } else { c as (i32) - 3i32 });
            pt_len[
                {
                    let _old = i;
                    i = i + 1;
                    _old
                } as (usize)
            ] = c as (u8);
            if !(i == i_special) {
                continue;
            }
            c = getbits(2i32) as (i16);
            'loop12: loop {
                if !({
                         c = (c as (i32) - 1) as (i16);
                         c
                     } as (i32) >= 0i32) {
                    break;
                }
                pt_len[
                    {
                        let _old = i;
                        i = i + 1;
                        _old
                    } as (usize)
                ] = 0u8;
            }
        }
        'loop4: loop {
            if !(i < nn) {
                break;
            }
            pt_len[
                {
                    let _old = i;
                    i = i + 1;
                    _old
                } as (usize)
            ] = 0u8;
        }
        make_table(nn,pt_len.as_mut_ptr(),8i32,pt_table,256i32);
    }
}

#[no_mangle]
pub unsafe extern fn read_c_len() {
    let mut i : i16;
    let mut c : i16;
    let mut n : i16;
    let mut mask : u16;
    n = getbits(9i32) as (i16);
    if n as (i32) == 0i32 {
        c = getbits(9i32) as (i16);
        i = 0i16;
        'loop23: loop {
            if !(i as (i32) < 0x7fi32 * 2i32 + 1i32 + 256i32 + 2i32 - 3i32) {
                break;
            }
            c_len[i as (usize)] = 0u8;
            i = (i as (i32) + 1) as (i16);
        }
        i = 0i16;
        'loop25: loop {
            if !(i as (i32) < 4096i32) {
                break;
            }
            *c_table.offset(i as (isize)) = c as (u16);
            i = (i as (i32) + 1) as (i16);
        }
    } else {
        i = 0i16;
        'loop2: loop {
            if !(i as (i32) < n as (i32)) {
                break;
            }
            c = *pt_table.offset(
                     (bitbuf as (i32) >> 8i32) as (isize)
                 ) as (i16);
            if c as (i32) >= 16i32 + 3i32 {
                mask = (1i32 << 7i32) as (u16);
                'loop8: loop {
                    if bitbuf as (i32) & mask as (i32) != 0 {
                        c = right[c as (usize)] as (i16);
                    } else {
                        c = left[c as (usize)] as (i16);
                    }
                    mask = (mask as (i32) >> 1i32) as (u16);
                    if !(c as (i32) >= 16i32 + 3i32) {
                        break;
                    }
                }
            }
            fillbuf(pt_len[c as (usize)] as (i32));
            if c as (i32) <= 2i32 {
                if c as (i32) == 0i32 {
                    c = 1i16;
                } else if c as (i32) == 1i32 {
                    c = getbits(4i32) as (i16);
                    c = (c as (i32) + 3i32) as (i16);
                } else {
                    c = getbits(9i32) as (i16);
                    c = (c as (i32) + 20i32) as (i16);
                }
                'loop20: loop {
                    if !({
                             c = (c as (i32) - 1) as (i16);
                             c
                         } as (i32) >= 0i32) {
                        break;
                    }
                    c_len[
                        {
                            let _old = i;
                            i = (i as (i32) + 1) as (i16);
                            _old
                        } as (usize)
                    ] = 0u8;
                }
            } else {
                c_len[
                    {
                        let _old = i;
                        i = (i as (i32) + 1) as (i16);
                        _old
                    } as (usize)
                ] = (c as (i32) - 2i32) as (u8);
            }
        }
        'loop3: loop {
            if !(i as (i32) < 0x7fi32 * 2i32 + 1i32 + 256i32 + 2i32 - 3i32) {
                break;
            }
            c_len[
                {
                    let _old = i;
                    i = (i as (i32) + 1) as (i16);
                    _old
                } as (usize)
            ] = 0u8;
        }
        make_table(
            0x7fi32 * 2i32 + 1i32 + 256i32 + 2i32 - 3i32,
            c_len.as_mut_ptr(),
            12i32,
            c_table,
            4096i32
        );
    }
}

unsafe extern fn decode_start() {
    blocksize = 0i16;
    if {
           c_table = calloc(
                         4096usize,
                         ::std::mem::size_of::<i16>()
                     ) as (*mut u16);
           c_table
       } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16) {
        error_proc((*b"\0").as_ptr() as (*mut u8));
    }
    if {
           pt_table = calloc(
                          256usize,
                          ::std::mem::size_of::<i16>()
                      ) as (*mut u16);
           pt_table
       } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16) {
        error_proc((*b"\0").as_ptr() as (*mut u8));
    }
    decode_start_stub();
}

unsafe extern fn decode_c() -> u16 {
    let mut j : u16;
    let mut mask : u16;
    if blocksize as (i32) == 0i32 {
        blocksize = getbits(16i32) as (i16);
        read_pt_len(16i32 + 3i32,5i32,3i32);
        read_c_len();
        read_pt_len(16i32 + 1i32,5i32,-1i32);
    }
    blocksize = (blocksize as (i32) - 1) as (i16);
    j = *c_table.offset((bitbuf as (i32) >> 4i32) as (isize));
    if j as (i32) >= 0x7fi32 * 2i32 + 1i32 + 256i32 + 2i32 - 3i32 {
        mask = (1i32 << 3i32) as (u16);
        'loop4: loop {
            if bitbuf as (i32) & mask as (i32) != 0 {
                j = right[j as (usize)];
            } else {
                j = left[j as (usize)];
            }
            mask = (mask as (i32) >> 1i32) as (u16);
            if !(j as (i32) >= 0x7fi32 * 2i32 + 1i32 + 256i32 + 2i32 - 3i32) {
                break;
            }
        }
    }
    fillbuf(c_len[j as (usize)] as (i32));
    j
}

unsafe extern fn decode_p() -> u16 {
    let mut j : u16;
    let mut mask : u16;
    j = *pt_table.offset((bitbuf as (i32) >> 8i32) as (isize));
    if j as (i32) >= 16i32 + 1i32 {
        mask = (1i32 << 7i32) as (u16);
        'loop2: loop {
            if bitbuf as (i32) & mask as (i32) != 0 {
                j = right[j as (usize)];
            } else {
                j = left[j as (usize)];
            }
            mask = (mask as (i32) >> 1i32) as (u16);
            if !(j as (i32) >= 16i32 + 1i32) {
                break;
            }
        }
    }
    fillbuf(pt_len[j as (usize)] as (i32));
    if j as (i32) != 0i32 {
        j = (j as (i32) - 1) as (u16);
        j = ((1i32 << j as (i32)) + getbits(j as (i32))) as (u16);
    }
    j
}

unsafe extern fn decode_end() {
    free(c_table as (*mut ::std::os::raw::c_void));
    free(pt_table as (*mut ::std::os::raw::c_void));
    decode_end_stub();
}

#[no_mangle]
pub unsafe extern fn decode(mut action : i32) {
    let mut _currentBlock;
    let mut i : i16;
    let mut r : i16;
    let mut c : i16;
    static mut j : i16 = 0i16;
    if _setjmp(decode_proc) == 0 {
        dec_text = malloc_msg(26624u32) as (*mut u8);
        decode_start();
        display_indicator(0isize);
        count = origsize as (isize);
        r = 0i16;
        'loop2: loop {
            if !(count > 0isize) {
                _currentBlock = 3;
                break;
            }
            if {
                   c = decode_c() as (i16);
                   c
               } as (i32) <= 0x7fi32 * 2i32 + 1i32 {
                *dec_text.offset(r as (isize)) = c as (u8);
                count = count - 1isize;
                if !({
                         r = (r as (i32) + 1) as (i16);
                         r
                     } as (i32) >= 26624i32) {
                    continue;
                }
                r = 0i16;
                display_indicator(
                    origsize.wrapping_sub(count as (usize)) as (isize)
                );
                if extraction_stub(dec_text,26624i32,action) != 0 {
                    _currentBlock = 18;
                    break;
                }
            } else {
                j = (c as (i32) - (0x7fi32 * 2i32 + 1i32 + 1i32 - 3i32)) as (i16);
                count = (count as (usize)).wrapping_sub(j as (usize)) as (isize);
                i = (r as (i32) - decode_p() as (i32) - 1i32) as (i16);
                if i as (i32) < 0i32 {
                    i = (i as (i32) + 26624i32) as (i16);
                }
                if r as (i32) > i as (i32) && (r as (i32) < 26624i32 - 256i32 - 1i32) {
                    _currentBlock = 14;
                } else {
                    _currentBlock = 9;
                }
                'loop9: loop {
                    if _currentBlock == 9 {
                        if !({
                                 j = (j as (i32) - 1) as (i16);
                                 j
                             } as (i32) >= 0i32) {
                            break;
                        }
                        *dec_text.offset(r as (isize)) = *dec_text.offset(i as (isize));
                        if {
                               r = (r as (i32) + 1) as (i16);
                               r
                           } as (i32) >= 26624i32 {
                            r = 0i16;
                            display_indicator(
                                origsize.wrapping_sub(count as (usize)) as (isize)
                            );
                            if extraction_stub(dec_text,26624i32,action) != 0 {
                                _currentBlock = 18;
                                break 'loop2;
                            }
                        }
                        if !({
                                 i = (i as (i32) + 1) as (i16);
                                 i
                             } as (i32) >= 26624i32) {
                            _currentBlock = 9;
                            continue;
                        }
                        i = 0i16;
                        _currentBlock = 9;
                    } else {
                        if !({
                                 j = (j as (i32) - 1) as (i16);
                                 j
                             } as (i32) >= 0i32) {
                            break;
                        }
                        *dec_text.offset(
                             {
                                 let _old = r;
                                 r = (r as (i32) + 1) as (i16);
                                 _old
                             } as (isize)
                         ) = *dec_text.offset(
                                  {
                                      let _old = i;
                                      i = (i as (i32) + 1) as (i16);
                                      _old
                                  } as (isize)
                              );
                        _currentBlock = 14;
                    }
                }
            }
        }
    } else {
        _currentBlock = 3;
    }
    if _currentBlock == 3 {
        if r as (i32) > 0i32 {
            extraction_stub(dec_text,r as (i32),action);
        }
    }
    decode_end();
    free(dec_text as (*mut ::std::os::raw::c_void));
}

unsafe extern fn decode_len() -> i16 {
    let mut c : i16;
    let mut width : i16;
    let mut plus : i16;
    let mut pwr : i16;
    plus = 0i16;
    pwr = 1i16;
    width = 0i16;
    'loop1: loop {
        if !(width as (i32) < 7i32) {
            break;
        }
        c = getbits(1i32) as (i16);
        if c as (i32) == 0i32 {
            break;
        }
        plus = (plus as (i32) + pwr as (i32)) as (i16);
        pwr = (pwr as (i32) << 1i32) as (i16);
        width = (width as (i32) + 1) as (i16);
    }
    if width as (i32) != 0i32 {
        c = getbits(width as (i32)) as (i16);
    }
    c = (c as (i32) + plus as (i32)) as (i16);
    c
}

unsafe extern fn decode_ptr() -> i16 {
    let mut c : i16;
    let mut width : i16;
    let mut plus : i16;
    let mut pwr : i16;
    plus = 0i16;
    pwr = (1i32 << 9i32) as (i16);
    width = 9i16;
    'loop1: loop {
        if !(width as (i32) < 13i32) {
            break;
        }
        c = getbits(1i32) as (i16);
        if c as (i32) == 0i32 {
            break;
        }
        plus = (plus as (i32) + pwr as (i32)) as (i16);
        pwr = (pwr as (i32) << 1i32) as (i16);
        width = (width as (i32) + 1) as (i16);
    }
    if width as (i32) != 0i32 {
        c = getbits(width as (i32)) as (i16);
    }
    c = (c as (i32) + plus as (i32)) as (i16);
    c
}

#[no_mangle]
pub unsafe extern fn decode_f(mut action : i32) {
    let mut _currentBlock;
    let mut i : i32;
    let mut j : i32;
    let mut c : i32;
    let mut r : i32;
    static mut ncount : usize = 0usize;
    if ntext == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) {
        ntext = malloc_msg(32768u32) as (*mut u8);
    }
    decode_start_stub();
    display_indicator(0isize);
    ncount = 0usize;
    r = 0i32;
    'loop3: loop {
        if !(ncount < origsize) {
            _currentBlock = 4;
            break;
        }
        c = decode_len() as (i32);
        if c == 0i32 {
            ncount = ncount.wrapping_add(1usize);
            *ntext.offset(r as (isize)) = (bitbuf as (i32) >> 8i32) as (u8);
            fillbuf(8i32);
            if !({
                     r = r + 1;
                     r
                 } >= 32768i32) {
                continue;
            }
            r = 0i32;
            display_indicator(ncount as (isize));
            if extraction_stub(ntext,32768i32,action) != 0 {
                _currentBlock = 16;
                break;
            }
        } else {
            j = c - 1i32 + 3i32;
            ncount = ncount.wrapping_add(j as (usize));
            if {
                   i = r - decode_ptr() as (i32) - 1i32;
                   i
               } < 0i32 {
                i = i + 32768i32;
            }
            'loop9: loop {
                if !({
                         let _old = j;
                         j = j - 1;
                         _old
                     } > 0i32) {
                    break;
                }
                *ntext.offset(r as (isize)) = *ntext.offset(i as (isize));
                if {
                       r = r + 1;
                       r
                   } >= 32768i32 {
                    r = 0i32;
                    display_indicator(ncount as (isize));
                    if extraction_stub(ntext,32768i32,action) != 0 {
                        _currentBlock = 16;
                        break 'loop3;
                    }
                }
                if !({
                         i = i + 1;
                         i
                     } >= 32768i32) {
                    continue;
                }
                i = 0i32;
            }
        }
    }
    if _currentBlock == 4 {
        if r > 0i32 {
            extraction_stub(ntext,r,action);
        }
    }
    decode_end_stub();
}
