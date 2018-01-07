#[macro_export]
macro_rules! embLog_error {
    ($fmtstr: expr) => { embLog_error!($fmtstr,) };
    ($fmtstr: expr, $( $values:expr ),*) => {{
        print!("{}:{}: ", module_path!(), line!());
        println!($fmtstr $( , $values )*);
    }}
}
