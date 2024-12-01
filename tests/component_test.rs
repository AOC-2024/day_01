use day_01::{find_total_distance, find_similarity_score};

#[test]
fn it_should_find_total_distance() {
    assert_eq!(find_total_distance("tests/resources/puzzle.txt"), 11);
}

#[test]
fn it_should_find_similarity_score() {
    assert_eq!(find_similarity_score("tests/resources/puzzle.txt"), 31);
}