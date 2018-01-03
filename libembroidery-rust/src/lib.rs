extern crate libc;

#[macro_use]
pub mod error;
#[macro_use]
pub mod utils;

pub mod helpers;
pub use ::helpers::*;
pub use ::helpers::binary::*;

pub mod pattern;
pub use ::pattern::*;
pub use ::pattern::color::*;
pub use ::pattern::arc::*;



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
            pattern::color::EmbColor{
                r: 0xff,
                g: 0xaa,
                b: 0x22,
            }
        );
    }
}
