use libc;

#[derive(Copy)]
#[repr(C)]
pub struct EmbStitch {
    pub flags: i32,
    pub xx: f64,
    pub yy: f64,
    pub color: i32,
}

impl Clone for EmbStitch {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbStitchList {
    pub stitch: EmbStitch,
    pub next: *mut EmbStitchList,
}

impl Clone for EmbStitchList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embStitchList_create(mut data: EmbStitch) -> *mut EmbStitchList {
    let mut heapStitchList: *mut EmbStitchList =
        libc::malloc(::std::mem::size_of::<EmbStitchList>()) as (*mut EmbStitchList);
    if heapStitchList.is_null() {
        embLog_error!(
            "emb-stitch.c embStitchList_create(), cannot allocate memory for heapStitchList\n"
        );
        0i32 as (*mut EmbStitchList)
    } else {
        (*heapStitchList).stitch = data;
        (*heapStitchList).next = 0i32 as (*mut EmbStitchList);
        heapStitchList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embStitchList_add(
    mut pointer: *mut EmbStitchList,
    mut data: EmbStitch,
) -> *mut EmbStitchList {
    if pointer.is_null() {
        embLog_error!("emb-stitch.c embStitchList_add(), pointer argument is null\n");
        0i32 as (*mut EmbStitchList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-stitch.c embStitchList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbStitchList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbStitchList>())
            as (*mut EmbStitchList) as (*mut EmbStitchList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-stitch.c embStitchList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbStitchList)
        } else {
            pointer = (*pointer).next as (*mut EmbStitchList);
            (*pointer).stitch = data;
            (*pointer).next = 0i32 as (*mut EmbStitchList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embStitchList_getAt(
    mut pointer: *mut EmbStitchList,
    mut num: i32,
) -> EmbStitch {
    let mut i: i32;
    i = 0i32;
    'loop1: loop {
        if !(i < num) {
            break;
        }
        pointer = (*pointer).next as (*mut EmbStitchList);
        i = i + 1;
    }
    (*pointer).stitch
}

#[no_mangle]
pub unsafe extern "C" fn embStitchList_count(mut pointer: *mut EmbStitchList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbStitchList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embStitchList_empty(mut pointer: *mut EmbStitchList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embStitchList_free(mut pointer: *mut EmbStitchList) {
    let mut tempPointer: *mut EmbStitchList = pointer;
    let mut nextPointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbStitchList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbStitchList);
}
