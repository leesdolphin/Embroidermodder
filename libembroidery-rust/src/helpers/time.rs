use chrono::{DateTime, Datelike, Local, Timelike};

#[derive(Copy)]
#[repr(C)]
pub struct EmbTime {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Clone for EmbTime {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embTime_initNow(mut t: *mut EmbTime) {
    let dt: DateTime<Local> = Local::now();

    (*t).year = dt.year() as u32;
    (*t).month = dt.month0();
    (*t).day = dt.day();
    (*t).hour = dt.hour();
    (*t).minute = dt.minute();
    (*t).second = dt.second();
}
