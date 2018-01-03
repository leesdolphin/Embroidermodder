use pattern::color::EmbColor;

#[macro_use]
use error::macros;

extern "C" {
    fn free(__ptr: *mut ::std::os::raw::c_void);
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbArc {
    pub startX: f64,
    pub startY: f64,
    pub midX: f64,
    pub midY: f64,
    pub endX: f64,
    pub endY: f64,
}

impl Clone for EmbArc {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbArcObject {
    pub arc: EmbArc,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbArcObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embArcObject_make(
    sx: f64,
    sy: f64,
    mx: f64,
    my: f64,
    ex: f64,
    ey: f64,
) -> EmbArcObject {
    EmbArcObject {
        arc: EmbArc {
            startX: sx,
            startY: sy,
            midX: mx,
            midY: my,
            endX: ex,
            endY: ey,
        },
        lineType: 0,
        color: EmbColor::black(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embArcObject_create(
    mut sx: f64,
    mut sy: f64,
    mut mx: f64,
    mut my: f64,
    mut ex: f64,
    mut ey: f64,
) -> *mut EmbArcObject {
    let mut heapArcObj: *mut EmbArcObject =
        malloc(::std::mem::size_of::<EmbArcObject>()) as (*mut EmbArcObject);
    if heapArcObj.is_null() {
        0i32 as (*mut EmbArcObject)
    } else {
        (*heapArcObj).arc.startX = sx;
        (*heapArcObj).arc.startY = sy;
        (*heapArcObj).arc.midX = mx;
        (*heapArcObj).arc.midY = my;
        (*heapArcObj).arc.endX = ex;
        (*heapArcObj).arc.endY = ey;
        heapArcObj
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbArcObjectList {
    pub arcObj: EmbArcObject,
    pub next: *mut EmbArcObjectList,
}

impl Clone for EmbArcObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embArcObjectList_add(
    mut pointer: *mut EmbArcObjectList,
    mut data: EmbArcObject,
) -> *mut EmbArcObjectList {
    if pointer.is_null() {
        embLog_error!("emb-arc.c embArcObjectList_add(), pointer argument is null\n");
        0i32 as (*mut EmbArcObjectList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-arc.c embArcObjectList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbArcObjectList)
    } else {
        (*pointer).next = malloc(::std::mem::size_of::<EmbArcObjectList>())
            as (*mut EmbArcObjectList) as (*mut EmbArcObjectList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-arc.c embArcObjectList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbArcObjectList)
        } else {
            pointer = (*pointer).next as (*mut EmbArcObjectList);
            (*pointer).arcObj = data;
            (*pointer).next = 0i32 as (*mut EmbArcObjectList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embArcObjectList_count(mut pointer: *mut EmbArcObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbArcObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embArcObjectList_empty(mut pointer: *mut EmbArcObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embArcObjectList_free(mut pointer: *mut EmbArcObjectList) {
    let mut tempPointer: *mut EmbArcObjectList = pointer;
    let mut nextPointer: *mut EmbArcObjectList = 0i32 as (*mut EmbArcObjectList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbArcObjectList);
        free(tempPointer as (*mut ::std::os::raw::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbArcObjectList);
}
