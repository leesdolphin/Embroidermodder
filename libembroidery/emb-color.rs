extern {
    fn embLog_error(format : *const u8, ...);
    fn malloc(__size : usize) -> *mut ::std::os::raw::c_void;
    fn strtol(
        __nptr : *const u8, __endptr : *mut *mut u8, __base : i32
    ) -> isize;
}

#[derive(Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EmbColor_ {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

impl fmt::Display for EmbColor_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:X}{:X}{:X}", self.r, self.g, self.b)
    }
}

impl Clone for EmbColor_ {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern "C" fn embColor_make(
    mut r : u8, mut g : u8, mut b : u8
) -> EmbColor_ {
    let mut stackColor : EmbColor_;
    stackColor.r = r;
    stackColor.g = g;
    stackColor.b = b;
    stackColor
}

#[no_mangle]
pub unsafe extern fn embColor_create(
    mut r : u8, mut g : u8, mut b : u8
) -> *mut EmbColor_ {
    let mut heapColor
        : *mut EmbColor_
        = malloc(::std::mem::size_of::<EmbColor_>()) as (*mut EmbColor_);
    if heapColor.is_null() {
        embLog_error(
            (*b"emb-color.c embColor_create(), cannot allocate memory for heapColor\n\0").as_ptr(
            )
        );
        0i32 as (*mut EmbColor_)
    } else {
        (*heapColor).r = r;
        (*heapColor).g = g;
        (*heapColor).b = b;
        heapColor
    }
}

#[no_mangle]
pub unsafe extern fn embColor_fromHexStr(
    mut val : *mut u8
) -> EmbColor_ {
    let mut color : EmbColor_;
    let mut r : [u8; 3];
    let mut g : [u8; 3];
    let mut b : [u8; 3];
    r[0usize] = *val.offset(0isize);
    r[1usize] = *val.offset(1isize);
    r[2usize] = 0u8;
    g[0usize] = *val.offset(2isize);
    g[1usize] = *val.offset(3isize);
    g[2usize] = 0u8;
    b[0usize] = *val.offset(4isize);
    b[1usize] = *val.offset(5isize);
    b[2usize] = 0u8;
    color.r = strtol(
                  r.as_mut_ptr() as (*const u8),
                  0i32 as (*mut *mut u8),
                  16i32
              ) as (u8);
    color.g = strtol(
                  g.as_mut_ptr() as (*const u8),
                  0i32 as (*mut *mut u8),
                  16i32
              ) as (u8);
    color.b = strtol(
                  b.as_mut_ptr() as (*const u8),
                  0i32 as (*mut *mut u8),
                  16i32
              ) as (u8);
    color
}
