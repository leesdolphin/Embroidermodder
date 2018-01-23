use types::Pattern;


#[test]
fn empty_pattern() {
    let pattern = Pattern::create().unwrap();
    pattern.free().unwrap();
}
