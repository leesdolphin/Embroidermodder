use libc;

use pattern::utils::color::EmbColor;
use pattern::utils::flag::{EmbFlagList, embFlagList_free};
use pattern::point::{EmbPointList, embPointList_free};

#[derive(Copy)]
#[repr(C)]
pub struct EmbPathObject {
    pub pointList: *mut EmbPointList,
    pub flagList: *mut EmbFlagList,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbPathObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObject_create(
    mut pointList: *mut EmbPointList,
    mut flagList: *mut EmbFlagList,
    mut color: EmbColor,
    mut lineType: i32,
) -> *mut EmbPathObject {
    let mut heapPathObj: *mut EmbPathObject = 0i32 as (*mut EmbPathObject);
    if pointList.is_null() {
        embLog_error!("emb-path.c embPathObject_create(), pointList argument is null\n");
        0i32 as (*mut EmbPathObject)
    } else if flagList.is_null() {
        embLog_error!("emb-path.c embPathObject_create(), flagList argument is null\n");
        0i32 as (*mut EmbPathObject)
    } else {
        heapPathObj = libc::malloc(::std::mem::size_of::<EmbPathObject>()) as (*mut EmbPathObject);
        (if heapPathObj.is_null() {
            embLog_error!(
                "emb-path.c embPathObject_create(), cannot allocate memory for heapPathObj\n"
            );
            0i32 as (*mut EmbPathObject)
        } else {
            (*heapPathObj).pointList = pointList;
            (*heapPathObj).flagList = flagList;
            (*heapPathObj).color = color;
            (*heapPathObj).lineType = lineType;
            heapPathObj
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObject_free(mut pointer: *mut EmbPathObject) {
    embPointList_free((*pointer).pointList);
    (*pointer).pointList = 0i32 as (*mut EmbPointList);
    embFlagList_free((*pointer).flagList);
    (*pointer).flagList = 0i32 as (*mut EmbFlagList);
    libc::free(pointer as (*mut libc::c_void));
    pointer = 0i32 as (*mut EmbPathObject);
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPathObjectList {
    pub pathObj: *mut EmbPathObject,
    pub next: *mut EmbPathObjectList,
}

impl Clone for EmbPathObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObjectList_create(
    mut data: *mut EmbPathObject,
) -> *mut EmbPathObjectList {
    let mut heapPathObjList: *mut EmbPathObjectList = 0i32 as (*mut EmbPathObjectList);
    if data.is_null() {
        embLog_error!("emb-path.c embPathObjectList_create(), data argument is null\n");
        0i32 as (*mut EmbPathObjectList)
    } else {
        heapPathObjList =
            libc::malloc(::std::mem::size_of::<EmbPathObjectList>()) as (*mut EmbPathObjectList);
        (if heapPathObjList.is_null() {
            embLog_error!(
                 "emb-path.c embPathObjectList_create(), cannot allocate memory for heapPathObjList\n"
             );
            0i32 as (*mut EmbPathObjectList)
        } else {
            (*heapPathObjList).pathObj = data;
            (*heapPathObjList).next = 0i32 as (*mut EmbPathObjectList);
            heapPathObjList
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObjectList_add(
    mut pointer: *mut EmbPathObjectList,
    mut data: *mut EmbPathObject,
) -> *mut EmbPathObjectList {
    if pointer.is_null() {
        embLog_error!("emb-path.c embPathObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbPathObjectList)
    } else if data.is_null() {
        embLog_error!("emb-path.c embPathObjectList_add(), data argument is null\n");
        0i32 as (*mut EmbPathObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-path.c embPathObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbPathObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbPathObjectList>())
            as (*mut EmbPathObjectList) as (*mut EmbPathObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-path.c embPathObjectList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbPathObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbPathObjectList);
            (*pointer).pathObj = data;
            (*pointer).next = 0i32 as (*mut EmbPathObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObjectList_count(mut pointer: *mut EmbPathObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbPathObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObjectList_empty(mut pointer: *mut EmbPathObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPathObjectList_free(mut pointer: *mut EmbPathObjectList) {
    let mut tempPointer: *mut EmbPathObjectList = pointer;
    let mut nextPointer: *mut EmbPathObjectList = 0i32 as (*mut EmbPathObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbPathObjectList);
        embPathObject_free((*tempPointer).pathObj);
        (*tempPointer).pathObj = 0i32 as (*mut EmbPathObject);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbPathObjectList);
}
