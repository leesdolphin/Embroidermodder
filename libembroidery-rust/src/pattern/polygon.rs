use libc;

use pattern::utils::color::EmbColor;
use pattern::point::{EmbPointList, embPointList_free};

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolygonObject {
    pub pointList: *mut EmbPointList,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbPolygonObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObject_create(
    mut pointList: *mut EmbPointList,
    mut color: EmbColor,
    mut lineType: i32,
) -> *mut EmbPolygonObject {
    let mut heapPolygonObj: *mut EmbPolygonObject = 0i32 as (*mut EmbPolygonObject);
    if pointList.is_null() {
        embLog_error!("emb-polygon.c embPolygonObject_create(), pointList argument is null\n");
        0i32 as (*mut EmbPolygonObject)
    } else {
        heapPolygonObj =
            libc::malloc(::std::mem::size_of::<EmbPolygonObject>()) as (*mut EmbPolygonObject);
        (if heapPolygonObj.is_null() {
            embLog_error!(
                 "emb-polygon.c embPolygonObject_create(), cannot allocate memory for heapPolygonObj\n"
             );
            0i32 as (*mut EmbPolygonObject)
        } else {
            (*heapPolygonObj).pointList = pointList;
            (*heapPolygonObj).color = color;
            (*heapPolygonObj).lineType = lineType;
            heapPolygonObj
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObject_free(mut pointer: *mut EmbPolygonObject) {
    embPointList_free((*pointer).pointList);
    (*pointer).pointList = 0i32 as (*mut EmbPointList);
    libc::free(pointer as (*mut libc::c_void));
    pointer = 0i32 as (*mut EmbPolygonObject);
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolygonObjectList {
    pub polygonObj: *mut EmbPolygonObject,
    pub next: *mut EmbPolygonObjectList,
}

impl Clone for EmbPolygonObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObjectList_create(
    mut data: *mut EmbPolygonObject,
) -> *mut EmbPolygonObjectList {
    let mut heapPolygonObjList: *mut EmbPolygonObjectList = 0i32 as (*mut EmbPolygonObjectList);
    if data.is_null() {
        embLog_error!("emb-polygon.c embPolygonObjectList_create(), data argument is null\n");
        0i32 as (*mut EmbPolygonObjectList)
    } else {
        heapPolygonObjList = libc::malloc(::std::mem::size_of::<EmbPolygonObjectList>())
            as (*mut EmbPolygonObjectList);
        (if heapPolygonObjList.is_null() {
            embLog_error!(
                 "emb-polygon.c embPolygonObjectList_create(), cannot allocate memory for heapPolygonObjList\n"
             );
            0i32 as (*mut EmbPolygonObjectList)
        } else {
            (*heapPolygonObjList).polygonObj = data;
            (*heapPolygonObjList).next = 0i32 as (*mut EmbPolygonObjectList);
            heapPolygonObjList
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObjectList_add(
    mut pointer: *mut EmbPolygonObjectList,
    mut data: *mut EmbPolygonObject,
) -> *mut EmbPolygonObjectList {
    if pointer.is_null() {
        embLog_error!("emb-polygon.c embPolygonObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbPolygonObjectList)
    } else if data.is_null() {
        embLog_error!("emb-polygon.c embPolygonObjectList_add(), data argument is null\n");
        0i32 as (*mut EmbPolygonObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-polygon.c embPolygonObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbPolygonObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbPolygonObjectList>())
            as (*mut EmbPolygonObjectList) as (*mut EmbPolygonObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                 "emb-polygon.c embPolygonObjectList_add(), cannot allocate memory for pointer->next\n"
             );
            0i32 as (*mut EmbPolygonObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbPolygonObjectList);
            (*pointer).polygonObj = data;
            (*pointer).next = 0i32 as (*mut EmbPolygonObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObjectList_count(mut pointer: *mut EmbPolygonObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbPolygonObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObjectList_empty(mut pointer: *mut EmbPolygonObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolygonObjectList_free(mut pointer: *mut EmbPolygonObjectList) {
    let mut tempPointer: *mut EmbPolygonObjectList = pointer;
    let mut nextPointer: *mut EmbPolygonObjectList = 0i32 as (*mut EmbPolygonObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbPolygonObjectList);
        embPolygonObject_free((*tempPointer).polygonObj);
        (*tempPointer).polygonObj = 0i32 as (*mut EmbPolygonObject);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbPolygonObjectList);
}
