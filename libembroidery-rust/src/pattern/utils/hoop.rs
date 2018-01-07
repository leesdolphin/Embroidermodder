#[derive(Copy)]
#[repr(C)]
pub struct EmbHoop {
    pub width: f64,
    pub height: f64,
}

impl Clone for EmbHoop {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embHoop_width(mut hoop: EmbHoop) -> f64 {
    hoop.width
}

#[no_mangle]
pub unsafe extern "C" fn embHoop_height(mut hoop: EmbHoop) -> f64 {
    hoop.height
}
