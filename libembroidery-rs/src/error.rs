

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        LibEmbroideryFailure(t: String) {
            description("Call into libembroidery failed")
            display("Call into libembroidery failed: '{}'", t)
        }
    }
}