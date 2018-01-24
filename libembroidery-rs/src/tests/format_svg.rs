use tests::utils::square_pattern;


#[test]
fn empty_pattern() {
    let pattern = square_pattern(10.0);



    pattern.free().unwrap();
}
