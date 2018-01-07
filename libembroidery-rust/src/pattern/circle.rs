use libc;

use pattern::utils::color::EmbColor;

#[derive(Copy)]
#[repr(C)]
pub struct EmbCircle {
    pub centerX: f64,
    pub centerY: f64,
    pub radius: f64,
}

impl Clone for EmbCircle {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircle_centerX(mut circle: EmbCircle) -> f64 {
    circle.centerX
}

#[no_mangle]
pub unsafe extern "C" fn embCircle_centerY(mut circle: EmbCircle) -> f64 {
    circle.centerY
}

#[no_mangle]
pub unsafe extern "C" fn embCircle_radius(mut circle: EmbCircle) -> f64 {
    circle.radius
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbCircleObject {
    pub circle: EmbCircle,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbCircleObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObject_make(
    mut cx: f64,
    mut cy: f64,
    mut r: f64,
) -> EmbCircleObject {
    EmbCircleObject {
        circle: EmbCircle {
            centerX: cx,
            centerY: cy,
            radius: r,
        },
        lineType: 0,
        color: EmbColor::black(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObject_create(
    mut cx: f64,
    mut cy: f64,
    mut r: f64,
) -> *mut EmbCircleObject {
    let mut heapCircleObj: *mut EmbCircleObject =
        libc::malloc(::std::mem::size_of::<EmbCircleObject>()) as (*mut EmbCircleObject);
    if heapCircleObj.is_null() {
        embLog_error!(
            "emb-circle.c embCircleObject_create(), cannot allocate memory for heapCircleObj\n"
        );
        0i32 as (*mut EmbCircleObject)
    } else {
        (*heapCircleObj).circle.centerX = cx;
        (*heapCircleObj).circle.centerY = cy;
        (*heapCircleObj).circle.radius = r;
        heapCircleObj
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbCircleObjectList {
    pub circleObj: EmbCircleObject,
    pub next: *mut EmbCircleObjectList,
}

impl Clone for EmbCircleObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObjectList_create(
    mut data: EmbCircleObject,
) -> *mut EmbCircleObjectList {
    let mut heapCircleObjList: *mut EmbCircleObjectList =
        libc::malloc(::std::mem::size_of::<EmbCircleObjectList>()) as (*mut EmbCircleObjectList);
    if heapCircleObjList.is_null() {
        embLog_error!(
            "emb-circle.c embCircleObjectList_create(), cannot allocate memory for heapCircleObjList\n"
        );
        0i32 as (*mut EmbCircleObjectList)
    } else {
        (*heapCircleObjList).circleObj = data;
        (*heapCircleObjList).next = 0i32 as (*mut EmbCircleObjectList);
        heapCircleObjList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObjectList_add(
    mut pointer: *mut EmbCircleObjectList,
    mut data: EmbCircleObject,
) -> *mut EmbCircleObjectList {
    if pointer.is_null() {
        embLog_error!("emb-circle.c embCircleObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbCircleObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-circle.c embCircleObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbCircleObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbCircleObjectList>())
            as (*mut EmbCircleObjectList) as (*mut EmbCircleObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                 "emb-circle.c embCircleObjectList_add(), cannot allocate memory for pointer->next\n"
             );
            0i32 as (*mut EmbCircleObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbCircleObjectList);
            (*pointer).circleObj = data;
            (*pointer).next = 0i32 as (*mut EmbCircleObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObjectList_count(mut pointer: *mut EmbCircleObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbCircleObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObjectList_empty(mut pointer: *mut EmbCircleObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embCircleObjectList_free(mut pointer: *mut EmbCircleObjectList) {
    let mut tempPointer: *mut EmbCircleObjectList = pointer;
    let mut nextPointer: *mut EmbCircleObjectList = 0i32 as (*mut EmbCircleObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbCircleObjectList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbCircleObjectList);
}
