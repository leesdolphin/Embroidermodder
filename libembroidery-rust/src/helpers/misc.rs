#![allow(warnings)]

use libc;

extern "C" {
    fn memmove(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: usize,
    ) -> *mut libc::c_void;
    fn sprintf(__s: *mut u8, __format: *const u8, ...) -> i32;
    fn strcpy(__dest: *mut u8, __src: *const u8) -> *mut u8;
    fn strcspn(__s: *const u8, __reject: *const u8) -> usize;
    fn strlen(__s: *const u8) -> usize;
    fn strncmp(__s1: *const u8, __s2: *const u8, __n: usize) -> i32;
    fn strspn(__s: *const u8, __accept: *const u8) -> usize;
}

#[no_mangle]
pub unsafe extern "C" fn roundDouble(mut src: f64) -> i32 {
    if src < 0.0f64 {
        (src - 0.5f64).ceil() as (i32)
    } else {
        (src + 0.5f64).floor() as (i32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn startsWith(mut pre: *const u8, mut str: *const u8) -> u8 {
    let mut result: u8 = 0u8;
    let mut lenpre: usize;
    let mut lenstr: usize;
    if pre.is_null() {
        embLog_error!("helpers-misc.c startsWith(), pre argument is null\n");
        0u8
    } else if str.is_null() {
        embLog_error!("helpers-misc.c startsWith(), str argument is null\n");
        0u8
    } else {
        lenpre = strlen(pre);
        lenstr = strlen(str);
        (if lenstr < lenpre {
            0u8
        } else {
            result = strncmp(pre, str, lenpre) as (u8);
            (if result as (i32) == 0i32 { 1u8 } else { 0u8 })
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn rTrim(str: *mut u8, mut junk: u8) -> *mut u8 {
    let mut original: *mut u8 = str.offset(strlen(str as (*const u8)) as (isize));
    'loop1: loop {
        if !(*{
            original = original.offset(-1isize);
            original
        } as (i32) == junk as (i32))
        {
            break;
        }
    }
    *original.offset(1isize) = b'\0';
    str
}

#[no_mangle]
pub unsafe extern "C" fn lTrim(str: *mut u8, mut junk: u8) -> *mut u8 {
    let mut original: *mut u8 = str;
    let mut p: *mut u8 = original;
    let mut trimmed: i32 = 0i32;
    'loop1: loop {
        if *original as (i32) != junk as (i32) || trimmed != 0 {
            trimmed = 1i32;
            *{
                let _old = p;
                p = p.offset(1isize);
                _old
            } = *original;
        }
        if !(*{
            let _old = original;
            original = original.offset(1isize);
            _old
        } as (i32) != b'\0' as (i32))
        {
            break;
        }
    }
    str
}

static mut WHITESPACE: [u8; 5] = *b" \t\n\r\0";

unsafe extern "C" fn get_trim_bounds(
    mut s: *const u8,
    mut firstWord: *mut *const u8,
    mut trailingSpace: *mut *const u8,
) {
    let mut lastWord: *const u8 = 0i32 as (*const u8);
    *firstWord = {
        lastWord = s.offset(strspn(s, WHITESPACE.as_ptr()) as (isize));
        lastWord
    };
    'loop1: loop {
        *trailingSpace = lastWord.offset(strcspn(lastWord, WHITESPACE.as_ptr()) as (isize));
        lastWord = (*trailingSpace).offset(strspn(*trailingSpace, WHITESPACE.as_ptr()) as (isize));
        if !(*lastWord as (i32) != b'\0' as (i32)) {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn copy_trim(mut s: *const u8) -> *mut u8 {
    let mut firstWord: *const u8 = 0i32 as (*const u8);
    let mut trailingSpace: *const u8 = 0i32 as (*const u8);
    let mut result: *mut u8 = 0i32 as (*mut u8);
    let mut newLength: usize;
    get_trim_bounds(
        s,
        &mut firstWord as (*mut *const u8),
        &mut trailingSpace as (*mut *const u8),
    );
    newLength = ((trailingSpace as (isize)).wrapping_sub(firstWord as (isize))
        / ::std::mem::size_of::<u8>() as (isize)) as (usize);
    result = libc::malloc(newLength.wrapping_add(1usize)) as (*mut u8);
    libc::memcpy(
        result as (*mut libc::c_void),
        firstWord as (*const libc::c_void),
        newLength,
    );
    *result.offset(newLength as (isize)) = b'\0';
    result
}

#[no_mangle]
pub unsafe extern "C" fn inplace_trim(mut s: *mut u8) {
    let mut firstWord: *const u8 = 0i32 as (*const u8);
    let mut trailingSpace: *const u8 = 0i32 as (*const u8);
    let mut newLength: usize;
    get_trim_bounds(
        s as (*const u8),
        &mut firstWord as (*mut *const u8),
        &mut trailingSpace as (*mut *const u8),
    );
    newLength = ((trailingSpace as (isize)).wrapping_sub(firstWord as (isize))
        / ::std::mem::size_of::<u8>() as (isize)) as (usize);
    memmove(
        s as (*mut libc::c_void),
        firstWord as (*const libc::c_void),
        newLength,
    );
    *s.offset(newLength as (isize)) = b'\0';
}

#[no_mangle]
pub unsafe extern "C" fn emb_optOut(mut num: f64, mut str: *mut u8) -> *mut u8 {
    sprintf(str, (*b"%.05f\0").as_ptr(), num);
    rTrim(str, b'0');
    rTrim(str, b'.');
    str
}

#[no_mangle]
pub unsafe extern "C" fn emb_optOut1d(mut num: f64, mut str: *mut u8) -> *mut u8 {
    *str.offset(0isize) = b'\0';
    sprintf(str, (*b"%.01f\0").as_ptr(), num);
    rTrim(str, b'0');
    rTrim(str, b'.');
    if strncmp(str as (*const u8), (*b"-0\0").as_ptr(), 2usize) == 0i32 {
        *str.offset(0isize) = b'0';
        *str.offset(1isize) = b'\0';
    }
    str
}

#[no_mangle]
pub unsafe extern "C" fn emb_strdup(mut src: *const u8) -> *mut u8 {
    let mut dest: *mut u8 = 0i32 as (*mut u8);
    if src.is_null() {
        embLog_error!("helpers-misc.c emb_strdup(), src argument is null\n");
        0i32 as (*mut u8)
    } else {
        dest = libc::malloc(strlen(src).wrapping_add(1usize)) as (*mut u8);
        if dest.is_null() {
            embLog_error!("helpers-misc.c emb_strdup(), cannot allocate memory\n");
        } else {
            strcpy(dest, src);
        }
        dest
    }
}
