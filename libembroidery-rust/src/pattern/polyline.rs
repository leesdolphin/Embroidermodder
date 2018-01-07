use libc;

use pattern::utils::color::EmbColor;
use pattern::point::{EmbPointList, embPointList_free};

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolylineObject {
    pub pointList: *mut EmbPointList,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbPolylineObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObject_create(
    mut pointList: *mut EmbPointList,
    mut color: EmbColor,
    mut lineType: i32,
) -> *mut EmbPolylineObject {
    let mut heapPolylineObj: *mut EmbPolylineObject = 0i32 as (*mut EmbPolylineObject);
    if pointList.is_null() {
        embLog_error!("emb-polyline.c embPolylineObject_create(), pointList argument is null\n");
        0i32 as (*mut EmbPolylineObject)
    } else {
        heapPolylineObj =
            libc::malloc(::std::mem::size_of::<EmbPolylineObject>()) as (*mut EmbPolylineObject);
        (if heapPolylineObj.is_null() {
            embLog_error!(
                 "emb-polyline.c embPolylineObject_create(), cannot allocate memory for heapPolylineObj\n"
             );
            0i32 as (*mut EmbPolylineObject)
        } else {
            (*heapPolylineObj).pointList = pointList;
            (*heapPolylineObj).color = color;
            (*heapPolylineObj).lineType = lineType;
            heapPolylineObj
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObject_free(mut pointer: *mut EmbPolylineObject) {
    embPointList_free((*pointer).pointList);
    (*pointer).pointList = 0i32 as (*mut EmbPointList);
    libc::free(pointer as (*mut libc::c_void));
    pointer = 0i32 as (*mut EmbPolylineObject);
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolylineObjectList {
    pub polylineObj: *mut EmbPolylineObject,
    pub next: *mut EmbPolylineObjectList,
}

impl Clone for EmbPolylineObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObjectList_create(
    mut data: *mut EmbPolylineObject,
) -> *mut EmbPolylineObjectList {
    let mut heapPolylineObjList: *mut EmbPolylineObjectList = 0i32 as (*mut EmbPolylineObjectList);
    if data.is_null() {
        embLog_error!("emb-polyline.c embPolylineObjectList_create(), data argument is null\n");
        0i32 as (*mut EmbPolylineObjectList)
    } else {
        heapPolylineObjList = libc::malloc(::std::mem::size_of::<EmbPolylineObjectList>())
            as (*mut EmbPolylineObjectList);
        (if heapPolylineObjList.is_null() {
            embLog_error!(
                 "emb-polyline.c embPolylineObjectList_create(), cannot allocate memory for heapPolylineObjList\n"
             );
            0i32 as (*mut EmbPolylineObjectList)
        } else {
            (*heapPolylineObjList).polylineObj = data;
            (*heapPolylineObjList).next = 0i32 as (*mut EmbPolylineObjectList);
            heapPolylineObjList
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObjectList_add(
    mut pointer: *mut EmbPolylineObjectList,
    mut data: *mut EmbPolylineObject,
) -> *mut EmbPolylineObjectList {
    if pointer.is_null() {
        embLog_error!("emb-polyline.c embPolylineObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbPolylineObjectList)
    } else if data.is_null() {
        embLog_error!("emb-polyline.c embPolylineObjectList_add(), data argument is null\n");
        0i32 as (*mut EmbPolylineObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-polyline.c embPolylineObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbPolylineObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbPolylineObjectList>())
            as (*mut EmbPolylineObjectList)
            as (*mut EmbPolylineObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                 "emb-polyline.c embPolylineObjectList_add(), cannot allocate memory for pointer->next\n"
             );
            0i32 as (*mut EmbPolylineObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbPolylineObjectList);
            (*pointer).polylineObj = data;
            (*pointer).next = 0i32 as (*mut EmbPolylineObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObjectList_count(
    mut pointer: *mut EmbPolylineObjectList,
) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbPolylineObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObjectList_empty(
    mut pointer: *mut EmbPolylineObjectList,
) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPolylineObjectList_free(mut pointer: *mut EmbPolylineObjectList) {
    let mut tempPointer: *mut EmbPolylineObjectList = pointer;
    let mut nextPointer: *mut EmbPolylineObjectList = 0i32 as (*mut EmbPolylineObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbPolylineObjectList);
        embPolylineObject_free((*tempPointer).polylineObj);
        (*tempPointer).polylineObj = 0i32 as (*mut EmbPolylineObject);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbPolylineObjectList);
}
