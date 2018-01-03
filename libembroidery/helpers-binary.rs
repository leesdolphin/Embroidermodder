use libc;

extern {
    fn embFile_getc(stream : *mut EmbFile_) -> i32;
    fn embFile_putc(ch : i32, stream : *mut EmbFile_) -> i32;
    fn embFile_read(
        ptr : *mut ::std::os::raw::c_void,
        size : usize,
        nmemb : usize,
        stream : *mut EmbFile_
    ) -> usize;
    fn embFile_write(
        ptr : *const ::std::os::raw::c_void,
        size : usize,
        nmemb : usize,
        stream : *mut EmbFile_
    ) -> usize;
}


#[derive(Copy)]
#[repr(C)]
pub struct EmbFile_ {
    pub file : *mut libc::FILE,
}

impl Clone for EmbFile_ {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn binaryReadByte(mut file : *mut EmbFile_) -> u8 {
    embFile_getc(file) as (u8)
}

#[no_mangle]
pub unsafe extern fn binaryReadBytes(
    mut file : *mut EmbFile_,
    mut destination : *mut u8,
    mut count : i32
) -> i32 {
    embFile_read(
        destination as (*mut ::std::os::raw::c_void),
        1usize,
        count as (usize),
        file
    ) as (i32)
}

#[no_mangle]
pub unsafe extern fn binaryReadInt16(mut file : *mut EmbFile_) -> i16 {
    let mut x : i32 = embFile_getc(file);
    x = x | embFile_getc(file) << 8i32;
    x as (i16)
}

#[no_mangle]
pub unsafe extern fn binaryReadInt32(mut file : *mut EmbFile_) -> i32 {
    let mut x : i32 = embFile_getc(file);
    x = x | embFile_getc(file) << 8i32;
    x = x | embFile_getc(file) << 16i32;
    x = x | embFile_getc(file) << 24i32;
    x
}

#[no_mangle]
pub unsafe extern fn binaryReadUInt8(mut file : *mut EmbFile_) -> u8 {
    embFile_getc(file) as (u8)
}

#[no_mangle]
pub unsafe extern fn binaryReadUInt16(
    mut file : *mut EmbFile_
) -> u16 {
    (embFile_getc(file) | embFile_getc(file) << 8i32) as (u16)
}

#[no_mangle]
pub unsafe extern fn binaryReadUInt32(
    mut file : *mut EmbFile_
) -> u32 {
    let mut x : u32 = embFile_getc(file) as (u32);
    x = x | (embFile_getc(file) << 8i32) as (u32);
    x = x | (embFile_getc(file) << 16i32) as (u32);
    x = x | (embFile_getc(file) << 24i32) as (u32);
    x
}

#[no_mangle]
pub unsafe extern fn binaryReadInt16BE(
    mut file : *mut EmbFile_
) -> i16 {
    let mut returnValue : i16 = (embFile_getc(file) << 8i32) as (i16);
    returnValue = (returnValue as (i32) | embFile_getc(file)) as (i16);
    returnValue
}

#[no_mangle]
pub unsafe extern fn binaryReadUInt16BE(
    mut file : *mut EmbFile_
) -> u16 {
    let mut returnValue : u16 = (embFile_getc(file) << 8i32) as (u16);
    returnValue = (returnValue as (i32) | embFile_getc(file)) as (u16);
    returnValue
}

#[no_mangle]
pub unsafe extern fn binaryReadInt32BE(
    mut file : *mut EmbFile_
) -> i32 {
    let mut returnValue : i32 = embFile_getc(file) << 24i32;
    returnValue = returnValue | embFile_getc(file) << 16i32;
    returnValue = returnValue | embFile_getc(file) << 8i32;
    returnValue = returnValue | embFile_getc(file);
    returnValue
}

#[no_mangle]
pub unsafe extern fn binaryReadUInt32BE(
    mut file : *mut EmbFile_
) -> u32 {
    let mut returnValue : u32 = (embFile_getc(file) << 24i32) as (u32);
    returnValue = returnValue | (embFile_getc(file) << 16i32) as (u32);
    returnValue = returnValue | (embFile_getc(file) << 8i32) as (u32);
    returnValue = returnValue | embFile_getc(file) as (u32);
    returnValue
}

#[no_mangle]
pub unsafe extern fn binaryReadString(
    mut file : *mut EmbFile_, mut buffer : *mut u8, mut maxLength : i32
) {
    let mut i : i32 = 0i32;
    'loop1: loop {
        if !(i < maxLength) {
            break;
        }
        *buffer.offset(i as (isize)) = embFile_getc(file) as (u8);
        if *buffer.offset(i as (isize)) as (i32) == b'\0' as (i32) {
            break;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn binaryReadUnicodeString(
    mut file : *mut EmbFile_, mut buffer : *mut u8, stringLength : i32
) {
    let mut i : i32 = 0i32;
    i = 0i32;
    'loop1: loop {
        if !(i < stringLength * 2i32) {
            break;
        }
        let mut input : u8 = embFile_getc(file) as (u8);
        if input as (i32) != 0i32 {
            *buffer.offset(i as (isize)) = input;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern fn binaryReadFloat(mut file : *mut EmbFile_) -> f32 {
    let mut store
        : [u8; 4]
        = [   embFile_getc(file) as (u8),
              embFile_getc(file) as (u8),
              embFile_getc(file) as (u8),
              embFile_getc(file) as (u8)
          ];
    *(store.as_mut_ptr() as (*mut f32))
}

#[no_mangle]
pub unsafe extern fn binaryWriteByte(
    mut file : *mut EmbFile_, mut data : u8
) {
    embFile_putc(data as (i32),file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteBytes(
    mut file : *mut EmbFile_, mut data : *const u8, mut size : i32
) {
    embFile_write(
        data as (*mut u8) as (*const ::std::os::raw::c_void),
        1usize,
        size as (usize),
        file
    );
}

#[no_mangle]
pub unsafe extern fn binaryWriteShort(
    mut file : *mut EmbFile_, mut data : i16
) {
    embFile_putc(data as (i32) & 0xffi32,file);
    embFile_putc(data as (i32) >> 8i32 & 0xffi32,file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteShortBE(
    mut file : *mut EmbFile_, mut data : i16
) {
    embFile_putc(data as (i32) >> 8i32 & 0xffi32,file);
    embFile_putc(data as (i32) & 0xffi32,file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteUShort(
    mut file : *mut EmbFile_, mut data : u16
) {
    embFile_putc(data as (i32) & 0xffi32,file);
    embFile_putc(data as (i32) >> 8i32 & 0xffi32,file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteUShortBE(
    mut file : *mut EmbFile_, mut data : u16
) {
    embFile_putc(data as (i32) >> 8i32 & 0xffi32,file);
    embFile_putc(data as (i32) & 0xffi32,file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteInt(
    mut file : *mut EmbFile_, mut data : i32
) {
    embFile_putc(data & 0xffi32,file);
    embFile_putc(data >> 8i32 & 0xffi32,file);
    embFile_putc(data >> 16i32 & 0xffi32,file);
    embFile_putc(data >> 24i32 & 0xffi32,file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteIntBE(
    mut file : *mut EmbFile_, mut data : i32
) {
    embFile_putc(data >> 24i32 & 0xffi32,file);
    embFile_putc(data >> 16i32 & 0xffi32,file);
    embFile_putc(data >> 8i32 & 0xffi32,file);
    embFile_putc(data & 0xffi32,file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteUInt(
    mut file : *mut EmbFile_, mut data : u32
) {
    embFile_putc((data & 0xffu32) as (i32),file);
    embFile_putc((data >> 8i32 & 0xffu32) as (i32),file);
    embFile_putc((data >> 16i32 & 0xffu32) as (i32),file);
    embFile_putc((data >> 24i32 & 0xffu32) as (i32),file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteUIntBE(
    mut file : *mut EmbFile_, mut data : u32
) {
    embFile_putc((data >> 24i32 & 0xffu32) as (i32),file);
    embFile_putc((data >> 16i32 & 0xffu32) as (i32),file);
    embFile_putc((data >> 8i32 & 0xffu32) as (i32),file);
    embFile_putc((data & 0xffu32) as (i32),file);
}

#[no_mangle]
pub unsafe extern fn binaryWriteFloat(
    mut file : *mut EmbFile_, mut data : f32
) {
}
