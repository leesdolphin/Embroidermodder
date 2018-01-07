use libc;

extern "C" {
    fn embFile_getc(stream: *mut EmbFile) -> i32;
    fn embFile_putc(ch: i32, stream: *mut EmbFile) -> i32;
    fn embFile_read(
        ptr: *mut libc::c_void,
        size: usize,
        nmemb: usize,
        stream: *mut EmbFile,
    ) -> usize;
    fn embFile_write(
        ptr: *const libc::c_void,
        size: usize,
        nmemb: usize,
        stream: *mut EmbFile,
    ) -> usize;
}
pub struct EmbFile;
// #[derive(Copy)]
// #[repr(C)]
// pub struct EmbFile {
//     pub file: *mut libc::FILE,
// }

// impl Clone for EmbFile {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

#[no_mangle]
pub unsafe extern "C" fn binaryReadByte(mut file: *mut EmbFile) -> u8 {
    embFile_getc(file) as (u8)
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadBytes(
    mut file: *mut EmbFile,
    mut destination: *mut u8,
    mut count: i32,
) -> i32 {
    embFile_read(
        destination as (*mut libc::c_void),
        1usize,
        count as (usize),
        file,
    ) as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadInt16(mut file: *mut EmbFile) -> i16 {
    let mut x: i32 = embFile_getc(file);
    x = x | embFile_getc(file) << 8i32;
    x as (i16)
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadInt32(mut file: *mut EmbFile) -> i32 {
    let mut x: i32 = embFile_getc(file);
    x = x | embFile_getc(file) << 8i32;
    x = x | embFile_getc(file) << 16i32;
    x = x | embFile_getc(file) << 24i32;
    x
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadUInt8(mut file: *mut EmbFile) -> u8 {
    embFile_getc(file) as (u8)
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadUInt16(mut file: *mut EmbFile) -> u16 {
    (embFile_getc(file) | embFile_getc(file) << 8i32) as (u16)
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadUInt32(mut file: *mut EmbFile) -> u32 {
    let mut x: u32 = embFile_getc(file) as (u32);
    x = x | (embFile_getc(file) << 8i32) as (u32);
    x = x | (embFile_getc(file) << 16i32) as (u32);
    x = x | (embFile_getc(file) << 24i32) as (u32);
    x
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadInt16BE(mut file: *mut EmbFile) -> i16 {
    let mut returnValue: i16 = (embFile_getc(file) << 8i32) as (i16);
    returnValue = (returnValue as (i32) | embFile_getc(file)) as (i16);
    returnValue
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadUInt16BE(mut file: *mut EmbFile) -> u16 {
    let mut returnValue: u16 = (embFile_getc(file) << 8i32) as (u16);
    returnValue = (returnValue as (i32) | embFile_getc(file)) as (u16);
    returnValue
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadInt32BE(mut file: *mut EmbFile) -> i32 {
    let mut returnValue: i32 = embFile_getc(file) << 24i32;
    returnValue = returnValue | embFile_getc(file) << 16i32;
    returnValue = returnValue | embFile_getc(file) << 8i32;
    returnValue = returnValue | embFile_getc(file);
    returnValue
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadUInt32BE(mut file: *mut EmbFile) -> u32 {
    let mut returnValue: u32 = (embFile_getc(file) << 24i32) as (u32);
    returnValue = returnValue | (embFile_getc(file) << 16i32) as (u32);
    returnValue = returnValue | (embFile_getc(file) << 8i32) as (u32);
    returnValue = returnValue | embFile_getc(file) as (u32);
    returnValue
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadString(
    mut file: *mut EmbFile,
    mut buffer: *mut libc::c_char,
    mut maxLength: i32,
) {
    let mut i: i32 = 0i32;
    'loop1: loop {
        if !(i < maxLength) {
            break;
        }
        *buffer.offset(i as (isize)) = embFile_getc(file) as (libc::c_char);
        if *buffer.offset(i as (isize)) as (i32) == b'\0' as (i32) {
            break;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadUnicodeString(
    mut file: *mut EmbFile,
    mut buffer: *mut u8,
    stringLength: i32,
) {
    let mut i: i32 = 0i32;
    i = 0i32;
    'loop1: loop {
        if !(i < stringLength * 2i32) {
            break;
        }
        let mut input: u8 = embFile_getc(file) as (u8);
        if input as (i32) != 0i32 {
            *buffer.offset(i as (isize)) = input;
        }
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn binaryReadFloat(mut file: *mut EmbFile) -> f32 {
    let mut store: [u8; 4] = [
        embFile_getc(file) as (u8),
        embFile_getc(file) as (u8),
        embFile_getc(file) as (u8),
        embFile_getc(file) as (u8),
    ];
    *(store.as_mut_ptr() as (*mut f32))
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteByte(mut file: *mut EmbFile, mut data: i8) {
    embFile_putc(data as (i32), file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteBytes(
    mut file: *mut EmbFile,
    mut data: *const libc::c_char,
    mut size: i32,
) {
    embFile_write(
        data as (*mut u8) as (*const libc::c_void),
        1usize,
        size as (usize),
        file,
    );
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteShort(mut file: *mut EmbFile, mut data: i16) {
    embFile_putc(data as (i32) & 0xffi32, file);
    embFile_putc(data as (i32) >> 8i32 & 0xffi32, file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteShortBE(mut file: *mut EmbFile, mut data: i16) {
    embFile_putc(data as (i32) >> 8i32 & 0xffi32, file);
    embFile_putc(data as (i32) & 0xffi32, file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteUShort(mut file: *mut EmbFile, mut data: u16) {
    embFile_putc(data as (i32) & 0xffi32, file);
    embFile_putc(data as (i32) >> 8i32 & 0xffi32, file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteUShortBE(mut file: *mut EmbFile, mut data: u16) {
    embFile_putc(data as (i32) >> 8i32 & 0xffi32, file);
    embFile_putc(data as (i32) & 0xffi32, file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteInt(mut file: *mut EmbFile, mut data: i32) {
    embFile_putc(data & 0xffi32, file);
    embFile_putc(data >> 8i32 & 0xffi32, file);
    embFile_putc(data >> 16i32 & 0xffi32, file);
    embFile_putc(data >> 24i32 & 0xffi32, file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteIntBE(mut file: *mut EmbFile, mut data: i32) {
    embFile_putc(data >> 24i32 & 0xffi32, file);
    embFile_putc(data >> 16i32 & 0xffi32, file);
    embFile_putc(data >> 8i32 & 0xffi32, file);
    embFile_putc(data & 0xffi32, file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteUInt(mut file: *mut EmbFile, mut data: u32) {
    embFile_putc((data & 0xffu32) as (i32), file);
    embFile_putc((data >> 8i32 & 0xffu32) as (i32), file);
    embFile_putc((data >> 16i32 & 0xffu32) as (i32), file);
    embFile_putc((data >> 24i32 & 0xffu32) as (i32), file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteUIntBE(mut file: *mut EmbFile, mut data: u32) {
    embFile_putc((data >> 24i32 & 0xffu32) as (i32), file);
    embFile_putc((data >> 16i32 & 0xffu32) as (i32), file);
    embFile_putc((data >> 8i32 & 0xffu32) as (i32), file);
    embFile_putc((data & 0xffu32) as (i32), file);
}

#[no_mangle]
pub unsafe extern "C" fn binaryWriteFloat(mut file: *mut EmbFile, mut data: f32) {
    unimplemented!();
}
