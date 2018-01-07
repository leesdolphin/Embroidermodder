use libc;

use pattern::utils::color::EmbColor;

#[derive(Copy)]
#[repr(C)]
pub struct EmbEllipse {
    pub centerX: f64,
    pub centerY: f64,
    pub radiusX: f64,
    pub radiusY: f64,
}

impl Clone for EmbEllipse {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_centerX(mut ellipse: EmbEllipse) -> f64 {
    ellipse.centerX
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_centerY(mut ellipse: EmbEllipse) -> f64 {
    ellipse.centerY
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_radiusX(mut ellipse: EmbEllipse) -> f64 {
    ellipse.radiusX
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_radiusY(mut ellipse: EmbEllipse) -> f64 {
    ellipse.radiusY
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_diameterX(mut ellipse: EmbEllipse) -> f64 {
    ellipse.radiusX * 2.0f64
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_diameterY(mut ellipse: EmbEllipse) -> f64 {
    ellipse.radiusY * 2.0f64
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_width(mut ellipse: EmbEllipse) -> f64 {
    ellipse.radiusX * 2.0f64
}

#[no_mangle]
pub unsafe extern "C" fn embEllipse_height(mut ellipse: EmbEllipse) -> f64 {
    ellipse.radiusY * 2.0f64
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbEllipseObject {
    pub ellipse: EmbEllipse,
    pub rotation: f64,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbEllipseObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObject_make(
    mut cx: f64,
    mut cy: f64,
    mut rx: f64,
    mut ry: f64,
) -> EmbEllipseObject {
    EmbEllipseObject {
        ellipse: EmbEllipse {
            centerX: cx,
            centerY: cy,
            radiusX: rx,
            radiusY: ry,
        },
        rotation: 0.,
        lineType: 0,
        color: EmbColor::black(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObject_create(
    mut cx: f64,
    mut cy: f64,
    mut rx: f64,
    mut ry: f64,
) -> *mut EmbEllipseObject {
    let mut heapEllipseObj: *mut EmbEllipseObject =
        libc::malloc(::std::mem::size_of::<EmbEllipseObject>()) as (*mut EmbEllipseObject);
    if heapEllipseObj.is_null() {
        embLog_error!(
            "emb-ellipse.c embEllipseObject_create(), cannot allocate memory for heapEllipseObj\n"
        );
        0i32 as (*mut EmbEllipseObject)
    } else {
        (*heapEllipseObj).ellipse.centerX = cx;
        (*heapEllipseObj).ellipse.centerY = cy;
        (*heapEllipseObj).ellipse.radiusX = rx;
        (*heapEllipseObj).ellipse.radiusY = ry;
        heapEllipseObj
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbEllipseObjectList {
    pub ellipseObj: EmbEllipseObject,
    pub next: *mut EmbEllipseObjectList,
}

impl Clone for EmbEllipseObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObjectList_create(
    mut data: EmbEllipseObject,
) -> *mut EmbEllipseObjectList {
    let mut heapEllipseObjList: *mut EmbEllipseObjectList =
        libc::malloc(::std::mem::size_of::<EmbEllipseObjectList>()) as (*mut EmbEllipseObjectList);
    if heapEllipseObjList.is_null() {
        embLog_error!(
            "emb-ellipse.c embEllipseObjectList_create(), cannot allocate memory for heapEllipseObjList\n"
        );
        0i32 as (*mut EmbEllipseObjectList)
    } else {
        (*heapEllipseObjList).ellipseObj = data;
        (*heapEllipseObjList).next = 0i32 as (*mut EmbEllipseObjectList);
        heapEllipseObjList
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObjectList_add(
    mut pointer: *mut EmbEllipseObjectList,
    mut data: EmbEllipseObject,
) -> *mut EmbEllipseObjectList {
    if pointer.is_null() {
        embLog_error!("emb-ellipse.c embEllipseObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbEllipseObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-ellipse.c embEllipseObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbEllipseObjectList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbEllipseObjectList>())
            as (*mut EmbEllipseObjectList) as (*mut EmbEllipseObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-ellipse.c embEllipseObjectList_add(), cannot allocate memory for pointer->next\n"
             );
            0i32 as (*mut EmbEllipseObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbEllipseObjectList);
            (*pointer).ellipseObj = data;
            (*pointer).next = 0i32 as (*mut EmbEllipseObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObjectList_count(mut pointer: *mut EmbEllipseObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbEllipseObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObjectList_empty(mut pointer: *mut EmbEllipseObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embEllipseObjectList_free(mut pointer: *mut EmbEllipseObjectList) {
    let mut tempPointer: *mut EmbEllipseObjectList = pointer;
    let mut nextPointer: *mut EmbEllipseObjectList = 0i32 as (*mut EmbEllipseObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbEllipseObjectList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbEllipseObjectList);
}
