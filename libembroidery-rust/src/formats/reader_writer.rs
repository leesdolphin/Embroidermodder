// use pattern::pattern::EmbPattern;
//
// #[derive(Copy)]
// #[repr(C)]
// pub struct EmbReaderWriter {
//     pub reader: unsafe extern "C" fn(*mut EmbPattern, *const u8) -> i32,
//     pub writer: unsafe extern "C" fn(*mut EmbPattern, *const u8) -> i32,
// }
//
// impl Clone for EmbReaderWriter {
//     fn clone(&self) -> Self {
//         *self
//     }
// }
