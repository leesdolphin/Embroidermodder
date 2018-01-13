use libc;

#[derive(Copy)]
#[repr(C)]
pub struct EmbVector {
    pub X: f64,
    pub Y: f64,
}

impl Clone for EmbVector {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVector_normalize(mut vector: EmbVector, mut result: *mut EmbVector) {
    let mut length: f64 = embVector_getLength(vector);
    if result.is_null() {
        embLog_error!("emb-vector.c embVector_normalize(), result argument is null\n");
    } else {
        (*result).X = vector.X / length;
        (*result).Y = vector.Y / length;
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVector_multiply(
    mut vector: EmbVector,
    mut magnitude: f64,
    mut result: *mut EmbVector,
) {
    if result.is_null() {
        embLog_error!("emb-vector.c embVector_multiply(), result argument is null\n");
    } else {
        (*result).X = vector.X * magnitude;
        (*result).Y = vector.Y * magnitude;
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVector_add(
    mut v1: EmbVector,
    mut v2: EmbVector,
    mut result: *mut EmbVector,
) {
    if result.is_null() {
        embLog_error!("emb-vector.c embVector_add(), result argument is null\n");
    } else {
        (*result).X = v1.X + v2.X;
        (*result).Y = v1.Y + v2.Y;
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVector_getLength(mut vector: EmbVector) -> f64 {
    (vector.X * vector.X + vector.Y * vector.Y).sqrt()
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbVectorList {
    pub vector: EmbVector,
    pub next: *mut EmbVectorList,
}

impl Clone for EmbVectorList {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVectorList_create(mut data: EmbVector) -> *mut EmbVectorList {
    let mut pointer: *mut EmbVectorList =
        libc::malloc(::std::mem::size_of::<EmbVectorList>()) as (*mut EmbVectorList);
    if pointer.is_null() {
        embLog_error!("emb-vector.c embVectorList_create(), cannot allocate memory for pointer\n");
        0i32 as (*mut EmbVectorList)
    } else {
        (*pointer).vector = data;
        (*pointer).next = 0i32 as (*mut EmbVectorList);
        pointer
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVectorList_add(
    mut pointer: *mut EmbVectorList,
    mut data: EmbVector,
) -> *mut EmbVectorList {
    if pointer.is_null() {
        embLog_error!("emb-vector.c embVectorList_add(), pointer argument is null\n");
        0i32 as (*mut EmbVectorList)
    } else if !(*pointer).next.is_null() {
        embLog_error!("emb-vector.c embVectorList_add(), pointer->next should be null\n");
        0i32 as (*mut EmbVectorList)
    } else {
        (*pointer).next = libc::malloc(::std::mem::size_of::<EmbVectorList>())
            as (*mut EmbVectorList) as (*mut EmbVectorList);
        (if (*pointer).next.is_null() {
            embLog_error!(
                "emb-vector.c embVectorList_add(), cannot allocate memory for pointer->next\n"
            );
            0i32 as (*mut EmbVectorList)
        } else {
            pointer = (*pointer).next as (*mut EmbVectorList);
            (*pointer).vector = data;
            (*pointer).next = 0i32 as (*mut EmbVectorList);
            pointer
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVectorList_count(mut pointer: *mut EmbVectorList) -> i32 {
    let mut i: i32 = 1i32;
    if pointer.is_null() {
        0i32
    } else {
        'loop1: loop {
            if (*pointer).next.is_null() {
                break;
            }
            pointer = (*pointer).next as (*mut EmbVectorList);
            i = i + 1;
        }
        i
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVectorList_empty(mut pointer: *mut EmbVectorList) -> i32 {
    if pointer.is_null() {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embVectorList_free(mut pointer: *mut EmbVectorList) {
    'loop0: loop {
        if pointer.is_null() {
            break;
        }
        let mut temp: *mut EmbVectorList = pointer;
        pointer = (*pointer).next as (*mut EmbVectorList);
        libc::free(temp as (*mut libc::c_void));
        temp = 0i32 as (*mut EmbVectorList);
    }
}
