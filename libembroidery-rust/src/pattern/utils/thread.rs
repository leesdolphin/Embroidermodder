use libc;

use pattern::utils::color::EmbColor;


#[derive(Copy)]
#[repr(C)]
pub struct EmbThread {
    pub color : EmbColor,
    pub description : *const libc::c_char,
    pub catalogNumber : *const libc::c_char,
}

impl Clone for EmbThread {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbThreadList {
    pub thread : EmbThread,
    pub next : *mut EmbThreadList,
}

impl Clone for EmbThreadList {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn embThread_findNearestColor(
    mut color : EmbColor, mut colors : *mut EmbThreadList
) -> i32 {
    let mut currentClosestValue : f64 = 9999999i32 as (f64);
    let mut closestIndex : i32 = -1i32;
    let mut red : i32 = color.r as (i32);
    let mut green : i32 = color.g as (i32);
    let mut blue : i32 = color.b as (i32);
    let mut i : i32 = 0i32;
    let mut currentThreadItem : *mut EmbThreadList = colors;
    'loop1: loop {
        if !(currentThreadItem != 0i32 as (*mut libc::c_void) as (*mut EmbThreadList)) {
            break;
        }
        let mut deltaRed : i32;
        let mut deltaBlue : i32;
        let mut deltaGreen : i32;
        let mut dist : f64;
        let mut c : EmbColor;
        c = (*currentThreadItem).thread.color;
        deltaRed = red - c.r as (i32);
        deltaBlue = green - c.g as (i32);
        deltaGreen = blue - c.b as (i32);
        dist = (
                   (deltaRed * deltaRed) as (f64) + (deltaBlue * deltaBlue) as (f64) + (deltaGreen * deltaGreen) as (f64)
               ).sqrt();
        if dist <= currentClosestValue {
            currentClosestValue = dist;
            closestIndex = i;
        }
        currentThreadItem = (*currentThreadItem).next as (*mut EmbThreadList);
        i = i + 1;
    }
    closestIndex
}

#[no_mangle]
pub unsafe extern fn embThread_findNearestColorInArray(
    mut color : EmbColor,
    mut colorArray : *mut EmbThread,
    mut count : i32
) -> i32 {
    let mut currentClosestValue : f64 = 9999999i32 as (f64);
    let mut closestIndex : i32 = -1i32;
    let mut red : i32 = color.r as (i32);
    let mut green : i32 = color.g as (i32);
    let mut blue : i32 = color.b as (i32);
    let mut i : i32 = 0i32;
    i = 0i32;
    'loop1: loop {
        if !(i < count) {
            break;
        }
        let mut deltaRed : i32;
        let mut deltaBlue : i32;
        let mut deltaGreen : i32;
        let mut dist : f64;
        let mut c : EmbColor;
        c = (*colorArray.offset(i as (isize))).color;
        deltaRed = red - c.r as (i32);
        deltaBlue = green - c.g as (i32);
        deltaGreen = blue - c.b as (i32);
        dist = (
                   (deltaRed * deltaRed) as (f64) + (deltaBlue * deltaBlue) as (f64) + (deltaGreen * deltaGreen) as (f64)
               ).sqrt();
        if dist <= currentClosestValue {
            currentClosestValue = dist;
            closestIndex = i;
        }
        i = i + 1;
    }
    closestIndex
}

#[no_mangle]
pub unsafe extern fn embThread_getRandom() -> EmbThread {
    // TODO: Replace this with nice random colour.
    EmbThread {
        color: EmbColor::random(),
        description: (*b"random\0").as_ptr() as *const i8,
        catalogNumber: (*b"\0").as_ptr() as *const i8,
    }
}

#[no_mangle]
pub unsafe extern fn embThreadList_create(
    mut data : EmbThread
) -> *mut EmbThreadList {
    let mut heapThreadList
        : *mut EmbThreadList
        = libc::malloc(
              ::std::mem::size_of::<EmbThreadList>()
          ) as (*mut EmbThreadList);
    if heapThreadList.is_null() {
        embLog_error!(
            "emb-thread.c embThreadList_create(), cannot allocate memory for heapThreadList\n"
        );
        0i32 as (*mut EmbThreadList)
    } else {
        (*heapThreadList).thread = data;
        (*heapThreadList).next = 0i32 as (*mut EmbThreadList);
        heapThreadList
    }
}

#[no_mangle]
pub unsafe extern fn embThreadList_add(
    mut pointer : *mut EmbThreadList, mut data : EmbThread
) -> *mut EmbThreadList {
    if pointer.is_null() {
        embLog_error!(
            "emb-thread.c embThreadList_add(), pointer argument is null\n"
        );
        0i32 as (*mut EmbThreadList)
    } else if !(*pointer).next.is_null() {
        embLog_error!(
            "emb-thread.c embThreadList_add(), pointer->next should be null\n"
        );
        0i32 as (*mut EmbThreadList)
    } else {
        (*pointer).next = libc::malloc(
                              ::std::mem::size_of::<EmbThreadList>()
                          ) as (*mut EmbThreadList) as (*mut EmbThreadList);
        (if (*pointer).next.is_null() {
             embLog_error!(
                 "emb-thread.c embThreadList_add(), cannot allocate memory for pointer->next\n"
             );
             0i32 as (*mut EmbThreadList)
         } else {
             pointer = (*pointer).next as (*mut EmbThreadList);
             (*pointer).thread = data;
             (*pointer).next = 0i32 as (*mut EmbThreadList);
             pointer
         })
    }
}

#[no_mangle]
pub unsafe extern fn embThreadList_getAt(
    mut pointer : *mut EmbThreadList, mut num : i32
) -> EmbThread {
    let mut i : i32 = 0i32;
    i = 0i32;
    'loop1: loop {
        if !(i < num) {
            break;
        }
        if !(*pointer).next.is_null() {
            pointer = (*pointer).next as (*mut EmbThreadList);
        }
        i = i + 1;
    }
    (*pointer).thread
}

#[no_mangle]
pub unsafe extern fn embThreadList_count(
    mut pointer : *mut EmbThreadList
) -> i32 {
    let mut i : i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbThreadList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern fn embThreadList_empty(
    mut pointer : *mut EmbThreadList
) -> i32 {
    if pointer.is_null() { 1i32 } else { 0i32 }
}

#[no_mangle]
pub unsafe extern fn embThreadList_free(
    mut pointer : *mut EmbThreadList
) {
    let mut tempPointer : *mut EmbThreadList = pointer;
    let mut nextPointer
        : *mut EmbThreadList
        = 0i32 as (*mut EmbThreadList);
    'loop1: loop {
        if tempPointer.is_null() {
            break;
        }
        nextPointer = (*tempPointer).next as (*mut EmbThreadList);
        libc::free(tempPointer as (*mut libc::c_void));
        tempPointer = nextPointer;
    }
    pointer = 0i32 as (*mut EmbThreadList);
}
