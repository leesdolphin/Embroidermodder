
use types::Pattern;
use flags::StitchFlags;

pub fn square_pattern(dim: f64) -> Pattern {
    let mut pattern = Pattern::create().unwrap();
    pattern.add_stitch_abs(0.0, 0.0, StitchFlags::NORMAL, true).unwrap();
    pattern.add_stitch_abs(dim, 0.0, StitchFlags::NORMAL, true).unwrap();
    pattern.add_stitch_abs(dim, dim, StitchFlags::NORMAL, true).unwrap();
    pattern.add_stitch_abs(0.0, dim, StitchFlags::NORMAL, true).unwrap();
    pattern.add_stitch_abs(0.0, 0.0, StitchFlags::NORMAL, true).unwrap();
    pattern.add_stitch_abs(0.0, 0.0, StitchFlags::END, true).unwrap();
    pattern
}
