extern crate chrono;
extern crate libc;

#[macro_use]
pub mod error;
#[macro_use]
pub mod utils;

pub mod helpers;
pub use helpers::*;
pub use helpers::binary::*;
pub use helpers::hash::*; // Used in 1 location.
pub use helpers::time::*;

pub mod pattern;
pub use pattern::*;
pub use pattern::utils::*;
pub use pattern::utils::color::*;
pub use pattern::utils::hoop::*;
pub use pattern::utils::flag::*;

pub use pattern::arc::*;
pub use pattern::circle::*;
pub use pattern::ellipse::*;
pub use pattern::line::*;
pub use pattern::path::*;
pub use pattern::polygon::*;
pub use pattern::polyline::*;
pub use pattern::point::*;
pub use pattern::rect::*;
pub use pattern::settings::*;
pub use pattern::spline::*;
pub use pattern::stitch::*;

// pub use ::pattern::polyline::*;

#[cfg(test)]
mod tests {
    pub use pattern;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        embLog_error!("Hello world");
    }
    #[test]
    fn check_color() {
        use std::ffi::CString;
        let color = CString::new("ffaa22").unwrap();
        assert_eq!(
            pattern::color::embColor_fromHexStr(color.as_ptr()),
            pattern::color::EmbColor {
                r: 0xff,
                g: 0xaa,
                b: 0x22,
            }
        );
    }
}
