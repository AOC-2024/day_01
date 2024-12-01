use day_01::find_total_distance;

#[test]
fn it_should_find_total_distance() {
    assert_eq!(find_total_distance("tests/resources/puzzle.txt"), 11);
}