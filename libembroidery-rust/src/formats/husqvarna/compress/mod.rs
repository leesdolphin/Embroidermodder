pub mod constants;
//pub mod data;
//pub mod compress;
 pub mod decompress;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        errors {
            InvalidArchive(t: String) {
                description("invalid archive")
                display("invalid archive: '{}'", t)
            }
        }
    }
}
