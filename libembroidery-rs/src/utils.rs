
#[macro_export]
macro_rules! unsafe_pointer_call {
    ($block: expr, $error: expr) => {
        {
            let result = unsafe { $block };
            if result.is_null() {
                return Err(ErrorKind::LibEmbroideryFailure(($error).into()).into());
            }
            result
        }
    };
    ($block: expr) => {
        unsafe_pointer_call!($block, "")
    };
}

#[macro_export]
macro_rules! wrap_embroidery_struct {
    ($c_type: ident => $rust_type: ident) => {
        use libembroidery_sys::$c_type;
        pub struct $rust_type {
            c_struct: *mut $c_type,
        }
        impl Into<*mut $c_type> for $rust_type {
            fn into(self) -> *mut $c_type {
                self.c_struct
            }
        }
        impl Into<$rust_type> for *mut $c_type {
            fn into(self) -> $rust_type {
                $rust_type { c_struct: self }
            }
        }
    };
}
