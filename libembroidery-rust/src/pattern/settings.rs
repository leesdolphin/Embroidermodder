use pattern::point::EmbPoint;

#[derive(Copy)]
#[repr(C)]
pub struct EmbSettings {
    pub dstJumpsPerTrim: u32,
    pub home: EmbPoint,
}

impl Clone for EmbSettings {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embSettings_init() -> EmbSettings {
    EmbSettings {
        dstJumpsPerTrim: 6u32,
        home: EmbPoint::new(0.0f64, 0.0f64),
    }
}

#[no_mangle]
pub unsafe extern "C" fn embSettings_home(mut settings: *mut EmbSettings) -> EmbPoint {
    (*settings).home
}

#[no_mangle]
pub unsafe extern "C" fn embSettings_setHome(mut settings: *mut EmbSettings, mut point: EmbPoint) {
    (*settings).home = point;
}
