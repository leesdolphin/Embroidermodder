

bitflags! {
    pub struct StitchFlags: u32 {
        const NORMAL = 0; /* stitch to (xx, yy) */
        const JUMP = 1; /* move to(xx, yy) */
        const TRIM = 2; /* trim + move to(xx, yy) */
        const STOP = 4; /* pause machine for thread change */
        const SEQUIN = 8; /* sequin */
        const END = 16; /* end of program */
    }
}

impl Into<i32> for StitchFlags {
    fn into(self) -> i32 {
        return self.bits as i32
    }
}

#[cfg(test)]
mod tests {
    use libembroidery_sys::{NORMAL, JUMP, TRIM, STOP, SEQUIN, END};
    use super::*;

    #[test]
    fn check_stitch_flag_values() {
        assert_eq!(StitchFlags::NORMAL, StitchFlags::from_bits(NORMAL).unwrap());
        assert_eq!(StitchFlags::JUMP, StitchFlags::from_bits(JUMP).unwrap());
        assert_eq!(StitchFlags::TRIM, StitchFlags::from_bits(TRIM).unwrap());
        assert_eq!(StitchFlags::STOP, StitchFlags::from_bits(STOP).unwrap());
        assert_eq!(StitchFlags::SEQUIN, StitchFlags::from_bits(SEQUIN).unwrap());
        assert_eq!(StitchFlags::END, StitchFlags::from_bits(END).unwrap());
    }
}
