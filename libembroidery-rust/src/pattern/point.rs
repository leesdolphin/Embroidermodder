use libc;

use pattern::utils::color::EmbColor;

#[derive(Copy)]
#[repr(C)]
pub struct EmbPoint {
    pub xx: f64,
    pub yy: f64,
}

impl Clone for EmbPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl EmbPoint {
    pub fn new(xx: f64, yy: f64) -> Self {
        Self { xx: xx, yy: yy }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPoint_x(mut point: EmbPoint) -> f64 {
    point.xx
}

#[no_mangle]
pub unsafe extern "C" fn embPoint_y(mut point: EmbPoint) -> f64 {
    point.yy
}

#[no_mangle]
pub unsafe extern "C" fn embPoint_make(mut x: f64, mut y: f64) -> EmbPoint {
    EmbPoint::new(x, y)
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPointList {
    pub point: EmbPoint,
    pub next: *mut EmbPointList,
}

impl Clone for EmbPointList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointList_create(mut x: f64, mut y: f64) -> *mut EmbPointList {
    let mut heapPointList: *mut EmbPointList =
        libc::malloc(::std::mem::size_of::<EmbPointList>()) as (*mut EmbPointList);
    if heapPointList.is_null() {
        embLog_error!(
            "emb-point.c embPointList_create(), cannot allocate memory for heapPointList\n"
        );
        0i32 as (*mut EmbPointList)
    } else {
        (*heapPointList).point.xx = x;
        (*heapPointList).point.yy = y;
        (*heapPointList).next = 0i32 as (*mut EmbPointList);
        heapPointList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointList_add(
    mut pointer: *mut EmbPointList,
    mut data: EmbPoint,
) -> *mut EmbPointList {
    if pointer.is_null() {
        embLog_error!("emb-point.c embPointList_add(), pointer argument is null\n");
        0i32 as (*mut EmbPointList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-point.c embPointList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbPointList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbPointList>()) as (*mut EmbPointList)
            as (*mut EmbPointList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-point.c embPointList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbPointList)
        } else {
            pointer = (*pointer).next as (*mut EmbPointList);
            (*pointer).point = data;
            (*pointer).next = 0i32 as (*mut EmbPointList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointList_count(mut pointer: *mut EmbPointList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbPointList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointList_empty(mut pointer: *mut EmbPointList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointList_free(mut pointer: *mut EmbPointList) {
    let mut tempPointer: *mut EmbPointList = pointer;
    let mut nextPointer: *mut EmbPointList = 0i32 as (*mut EmbPointList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbPointList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbPointList);
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPointObject {
    pub point: EmbPoint,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbPointObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObject_make(mut x: f64, mut y: f64) -> EmbPointObject {
    EmbPointObject {
        point: EmbPoint::new(x, y),
        lineType: 0,
        color: EmbColor::black(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObject_create(mut x: f64, mut y: f64) -> *mut EmbPointObject {
    let mut heapPointObj: *mut EmbPointObject =
        libc::malloc(::std::mem::size_of::<EmbPointObject>()) as (*mut EmbPointObject);
    if heapPointObj.is_null() {
        embLog_error!(
            "emb-point.c embPointObject_create(), cannot allocate memory for heapPointObj\n"
        );
        0i32 as (*mut EmbPointObject)
    } else {
        (*heapPointObj).point.xx = x;
        (*heapPointObj).point.yy = y;
        heapPointObj
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPointObjectList {
    pub pointObj: EmbPointObject,
    pub next: *mut EmbPointObjectList,
}

impl Clone for EmbPointObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObjectList_create(
    mut data: EmbPointObject,
) -> *mut EmbPointObjectList {
    let mut heapPointObjList: *mut EmbPointObjectList =
        libc::malloc(::std::mem::size_of::<EmbPointObjectList>()) as (*mut EmbPointObjectList);
    if heapPointObjList.is_null() {
        embLog_error!(
            "emb-point.c embPointObjectList_create(), cannot allocate memory for heapPointObjList\n"
        );
        0i32 as (*mut EmbPointObjectList)
    } else {
        (*heapPointObjList).pointObj = data;
        (*heapPointObjList).next = 0i32 as (*mut EmbPointObjectList);
        heapPointObjList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObjectList_add(
    mut pointer: *mut EmbPointObjectList,
    mut data: EmbPointObject,
) -> *mut EmbPointObjectList {
    if pointer.is_null() {
        embLog_error!("emb-point.c embPointObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbPointObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-point.c embPointObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbPointObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbPointObjectList>())
            as (*mut EmbPointObjectList) as (*mut EmbPointObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-point.c embPointObjectList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbPointObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbPointObjectList);
            (*pointer).pointObj = data;
            (*pointer).next = 0i32 as (*mut EmbPointObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObjectList_count(mut pointer: *mut EmbPointObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbPointObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObjectList_empty(mut pointer: *mut EmbPointObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPointObjectList_free(mut pointer: *mut EmbPointObjectList) {
    let mut tempPointer: *mut EmbPointObjectList = pointer;
    let mut nextPointer: *mut EmbPointObjectList = 0i32 as (*mut EmbPointObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbPointObjectList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbPointObjectList);
}
