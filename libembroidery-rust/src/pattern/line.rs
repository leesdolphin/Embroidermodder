use libc;

use pattern::utils::color::EmbColor;
use pattern::utils::vector::{EmbVector, embVector_normalize};


#[derive(Copy)]
#[repr(C)]
pub struct EmbLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Clone for EmbLine {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLine_x1(mut line: EmbLine) -> f64 {
    line.x1
}

#[no_mangle]
pub unsafe extern "C" fn embLine_y1(mut line: EmbLine) -> f64 {
    line.y1
}

#[no_mangle]
pub unsafe extern "C" fn embLine_x2(mut line: EmbLine) -> f64 {
    line.x2
}

#[no_mangle]
pub unsafe extern "C" fn embLine_y2(mut line: EmbLine) -> f64 {
    line.y2
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbLineObject {
    pub line: EmbLine,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbLineObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObject_make(
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> EmbLineObject {
    EmbLineObject {
        line: EmbLine {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        },
        lineType: 0,
        color: EmbColor::black(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObject_create(
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) -> *mut EmbLineObject {
    let mut heapLineObj: *mut EmbLineObject =
        libc::malloc(::std::mem::size_of::<EmbLineObject>()) as (*mut EmbLineObject);
    if heapLineObj.is_null() {
        embLog_error!(
            "emb-line.c embLineObject_create(), cannot allocate memory for heapLineObj\n"
        );
        0i32 as (*mut EmbLineObject)
    } else {
        (*heapLineObj).line.x1 = x1;
        (*heapLineObj).line.y1 = y1;
        (*heapLineObj).line.x2 = x2;
        (*heapLineObj).line.y2 = y2;
        heapLineObj
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbLineObjectList {
    pub lineObj: EmbLineObject,
    pub next: *mut EmbLineObjectList,
}

impl Clone for EmbLineObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObjectList_create(
    mut data: EmbLineObject,
) -> *mut EmbLineObjectList {
    let mut heapLineObjList: *mut EmbLineObjectList =
        libc::malloc(::std::mem::size_of::<EmbLineObjectList>()) as (*mut EmbLineObjectList);
    if heapLineObjList.is_null() {
        embLog_error!(
            "emb-line.c embLineObjectList_create(), cannot allocate memory for heapLineObjList\n"
        );
        0i32 as (*mut EmbLineObjectList)
    } else {
        (*heapLineObjList).lineObj = data;
        (*heapLineObjList).next = 0i32 as (*mut EmbLineObjectList);
        heapLineObjList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObjectList_add(
    mut pointer: *mut EmbLineObjectList,
    mut data: EmbLineObject,
) -> *mut EmbLineObjectList {
    if pointer.is_null() {
        embLog_error!("emb-line.c embLineObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbLineObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-line.c embLineObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbLineObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbLineObjectList>())
            as (*mut EmbLineObjectList) as (*mut EmbLineObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-line.c embLineObjectList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbLineObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbLineObjectList);
            (*pointer).lineObj = data;
            (*pointer).next = 0i32 as (*mut EmbLineObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObjectList_count(mut pointer: *mut EmbLineObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbLineObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObjectList_empty(mut pointer: *mut EmbLineObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLineObjectList_free(mut pointer: *mut EmbLineObjectList) {
    let mut tempPointer: *mut EmbLineObjectList = pointer;
    let mut nextPointer: *mut EmbLineObjectList = 0i32 as (*mut EmbLineObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbLineObjectList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbLineObjectList);
}


#[no_mangle]
pub unsafe extern "C" fn embLine_normalVector(
    mut vector1: EmbVector,
    mut vector2: EmbVector,
    mut result: *mut EmbVector,
    mut clockwise: i32,
) {
    let mut temp: f64;
    if result.is_null() {
        embLog_error!("emb-line.c embLine_normalVector(), result argument is null\n");
    } else {
        (*result).X = vector2.X - vector1.X;
        (*result).Y = vector2.Y - vector1.Y;
        embVector_normalize(*result, result);
        temp = (*result).X;
        (*result).X = (*result).Y;
        (*result).Y = -temp;
        if clockwise == 0 {
            (*result).X = -(*result).X;
            (*result).Y = -(*result).Y;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embLine_intersectionPoint(
    mut v1: EmbVector,
    mut v2: EmbVector,
    mut v3: EmbVector,
    mut v4: EmbVector,
    mut result: *mut EmbVector,
) {
    let mut A2: f64 = v2.Y - v1.Y;
    let mut B2: f64 = v1.X - v2.X;
    let mut C2: f64 = A2 * v1.X + B2 * v1.Y;
    let mut A1: f64 = v4.Y - v3.Y;
    let mut B1: f64 = v3.X - v4.X;
    let mut C1: f64 = A1 * v3.X + B1 * v3.Y;
    let mut det: f64 = A1 * B2 - A2 * B1;
    if result.is_null() {
        embLog_error!("emb-line.c embLine_intersectionPoint(), result argument is null\n");
    } else {
        if det < 1e-10f64 && (det > -1e-10f64) {
            (*result).X = -10000i32 as (f64);
            (*result).Y = -10000i32 as (f64);
        }
        (*result).X = (B2 * C1 - B1 * C2) / det;
        (*result).Y = (A1 * C2 - A2 * C1) / det;
    }
}
