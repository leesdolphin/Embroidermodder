use libc;

use pattern::utils::color::EmbColor;

#[derive(Copy)]
#[repr(C)]
pub struct EmbRect {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl Clone for EmbRect {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRect_x(mut rect: EmbRect) -> f64 {
    rect.left
}

#[no_mangle]
pub unsafe extern "C" fn embRect_y(mut rect: EmbRect) -> f64 {
    rect.top
}

#[no_mangle]
pub unsafe extern "C" fn embRect_width(mut rect: EmbRect) -> f64 {
    rect.right - rect.left
}

#[no_mangle]
pub unsafe extern "C" fn embRect_height(mut rect: EmbRect) -> f64 {
    rect.bottom - rect.top
}

#[no_mangle]
pub unsafe extern "C" fn embRect_setX(mut rect: *mut EmbRect, mut x: f64) {
    (*rect).left = x;
}

#[no_mangle]
pub unsafe extern "C" fn embRect_setY(mut rect: *mut EmbRect, mut y: f64) {
    (*rect).top = y;
}

#[no_mangle]
pub unsafe extern "C" fn embRect_setWidth(mut rect: *mut EmbRect, mut w: f64) {
    (*rect).right = (*rect).left + w;
}

#[no_mangle]
pub unsafe extern "C" fn embRect_setHeight(mut rect: *mut EmbRect, mut h: f64) {
    (*rect).bottom = (*rect).top + h;
}

#[no_mangle]
pub unsafe extern "C" fn embRect_setCoords(
    mut rect: *mut EmbRect,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) {
    (*rect).left = x1;
    (*rect).top = y1;
    (*rect).right = x2;
    (*rect).bottom = y2;
}

#[no_mangle]
pub unsafe extern "C" fn embRect_setRect(
    mut rect: *mut EmbRect,
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) {
    (*rect).left = x;
    (*rect).top = y;
    (*rect).right = x + w;
    (*rect).bottom = y + h;
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbRectObject {
    pub rect: EmbRect,
    pub rotation: f64,
    pub radius: f64,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbRectObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObject_make(
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) -> EmbRectObject {
    EmbRectObject {
        rect: EmbRect {
            left: x,
            top: y,
            right: x + w,
            bottom: y + h,
        },
        rotation: 0.,
        radius: 0.,
        lineType: 0,
        color: EmbColor::black(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObject_create(
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) -> *mut EmbRectObject {
    let mut heapRectObj: *mut EmbRectObject =
        libc::malloc(::std::mem::size_of::<EmbRectObject>()) as (*mut EmbRectObject);
    if heapRectObj.is_null() {
        embLog_error!(
            "emb-rect.c embRectObject_create(), cannot allocate memory for heapRectObj\n"
        );
        0i32 as (*mut EmbRectObject)
    } else {
        (*heapRectObj).rect.left = x;
        (*heapRectObj).rect.top = y;
        (*heapRectObj).rect.right = x + w;
        (*heapRectObj).rect.bottom = y + h;
        heapRectObj
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbRectObjectList {
    pub rectObj: EmbRectObject,
    pub next: *mut EmbRectObjectList,
}

impl Clone for EmbRectObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObjectList_create(
    mut data: EmbRectObject,
) -> *mut EmbRectObjectList {
    let mut heapRectObjList: *mut EmbRectObjectList =
        libc::malloc(::std::mem::size_of::<EmbRectObjectList>()) as (*mut EmbRectObjectList);
    if heapRectObjList.is_null() {
        embLog_error!(
            "emb-rect.c embRectObjectList_create(), cannot allocate memory for heapRectObjList\n"
        );
        0i32 as (*mut EmbRectObjectList)
    } else {
        (*heapRectObjList).rectObj = data;
        (*heapRectObjList).next = 0i32 as (*mut EmbRectObjectList);
        heapRectObjList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObjectList_add(
    mut pointer: *mut EmbRectObjectList,
    mut data: EmbRectObject,
) -> *mut EmbRectObjectList {
    if pointer.is_null() {
        embLog_error!("emb-rect.c embRectObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbRectObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-rect.c embRectObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbRectObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbRectObjectList>())
            as (*mut EmbRectObjectList) as (*mut EmbRectObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-rect.c embRectObjectList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbRectObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbRectObjectList);
            (*pointer).rectObj = data;
            (*pointer).next = 0i32 as (*mut EmbRectObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObjectList_count(mut pointer: *mut EmbRectObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbRectObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObjectList_empty(mut pointer: *mut EmbRectObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embRectObjectList_free(mut pointer: *mut EmbRectObjectList) {
    let mut tempPointer: *mut EmbRectObjectList = pointer;
    let mut nextPointer: *mut EmbRectObjectList = 0i32 as (*mut EmbRectObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbRectObjectList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbRectObjectList);
}
