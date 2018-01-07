use libc;

#[derive(Copy)]
#[repr(C)]
pub struct EmbFlagList {
    pub flag: i32,
    pub next: *mut EmbFlagList,
}

impl Clone for EmbFlagList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embFlagList_create(mut data: i32) -> *mut EmbFlagList {
    let mut heapFlagList: *mut EmbFlagList =
        libc::malloc(::std::mem::size_of::<EmbFlagList>()) as (*mut EmbFlagList);
    if heapFlagList.is_null() {
        embLog_error!("emb-flag.c embFlagList_create(), cannot allocate memory for heapFlagList\n");
        0i32 as (*mut EmbFlagList)
    } else {
        (*heapFlagList).flag = data;
        (*heapFlagList).next = 0i32 as (*mut EmbFlagList);
        heapFlagList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embFlagList_add(
    mut pointer: *mut EmbFlagList,
    mut data: i32,
) -> *mut EmbFlagList {
    if pointer.is_null() {
        embLog_error!("emb-flag.c embFlagList_add(), pointer argument is null\n");
        0i32 as (*mut EmbFlagList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-flag.c embFlagList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbFlagList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbFlagList>()) as (*mut EmbFlagList)
            as (*mut EmbFlagList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-flag.c embFlagList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbFlagList)
        } else {
            pointer = (*pointer).next as (*mut EmbFlagList);
            (*pointer).flag = data;
            (*pointer).next = 0i32 as (*mut EmbFlagList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embFlagList_count(mut pointer: *mut EmbFlagList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbFlagList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embFlagList_empty(mut pointer: *mut EmbFlagList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embFlagList_free(mut pointer: *mut EmbFlagList) {
    let mut tempPointer: *mut EmbFlagList = pointer;
    let mut nextPointer: *mut EmbFlagList = 0i32 as (*mut EmbFlagList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbFlagList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbFlagList);
}
