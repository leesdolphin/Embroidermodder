#[derive(Copy)]
#[repr(C)]
pub struct EmbHash {}

impl Clone for EmbHash {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embHash_create() -> *mut EmbHash {
    0 as (*mut EmbHash)
}
