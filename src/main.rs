use day_01::{find_total_distance, find_similarity_score};

fn main() {
    let total_distances = find_total_distance("src/resources/puzzle.txt");
    //Result : 2264607
    println!("Total distances : {total_distances}");

    let similarity_score = find_similarity_score("src/resources/puzzle.txt");
    //Result : 19457120
    println!("Total distances : {similarity_score}");
}
