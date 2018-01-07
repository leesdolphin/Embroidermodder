use pattern::utils::color::EmbColor;

#[derive(Copy)]
#[repr(C)]
pub struct EmbBezier {
    pub startX: f64,
    pub startY: f64,
    pub control1X: f64,
    pub control1Y: f64,
    pub control2X: f64,
    pub control2Y: f64,
    pub endX: f64,
    pub endY: f64,
}

impl Clone for EmbBezier {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbSplineObject {
    pub bezier: EmbBezier,
    pub next: *mut EmbSplineObject,
    pub lineType: i32,
    pub color: EmbColor,
}

impl Clone for EmbSplineObject {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbSplineObjectList {
    pub splineObj: EmbSplineObject,
    pub next: *mut EmbSplineObjectList,
}

impl Clone for EmbSplineObjectList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embSplineObjectList_count(mut pointer: *mut EmbSplineObjectList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbSplineObjectList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embSplineObjectList_empty(mut pointer: *mut EmbSplineObjectList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}
