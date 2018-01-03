
macro_rules! from_hex_or {
    ($value: expr, $range: expr) => {
        from_hex_or!($value, $range, 0)
    };
    ($value:expr, $range: expr, $other: expr) => {
        match $value.get($range) {
            Some(substr) => match u8::from_str_radix(substr, 16) {
                Ok(value) => value,
                Err(err) => {
                    embLog_error!("Invalid color given to from_hex_or macro: {:?} {:?}. Error: {:?}", $value, $range, err);
                    $other
                }
            },
            None => {
                embLog_error!("Invalid color given to from_hex_or macro: {:?} {:?}", $value, $range);
                $other
            }
        }
    };
}
