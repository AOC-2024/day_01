use std::fs::read_to_string;

pub fn find_total_distance(input_path: &str) -> u32 {
    let puzzle = extract_puzzle(input_path);

    return find_distance(puzzle);
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Puzzle {
    left: Vec<u32>,
    right: Vec<u32>
}

impl Puzzle {
    fn new() -> Puzzle {
        return Puzzle {
            left: vec![],
            right: vec![]
        };
    }

    fn add_line(&mut self, line: &str) {
        let numbers: Vec<u32> = line
        .split_whitespace() 
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
        
        self.left.push(numbers[0]);
        self.right.push(numbers[1]);
    }
}


fn extract_puzzle(input_path: &str) -> Puzzle {
    let mut puzzle = Puzzle::new();
    read_to_string(input_path)
    .unwrap()
    .lines()
    .for_each(|line| puzzle.add_line(line));

    puzzle
}

fn find_distance(puzzle: Puzzle) -> u32 {
    let mut right_vec_values_computed: Vec<u32> = Vec::new();
    let mut left_vec: Vec<u32> = puzzle.left.clone();
    left_vec.sort();

    left_vec.iter().fold(0, |acc, left_value| {
        let min_value = match puzzle
            .right
            .iter()
            .filter(|right_value| {
                let used_count = right_vec_values_computed.iter().filter(|&&v| v == **right_value).count();
                let available_count = puzzle.right.iter().filter(|&&v| v == **right_value).count();
                used_count < available_count
            })
            .min()
        {
            Some(min) => min,
            None => panic!("Could not find minimum value in the right puzzle inputs for the left value {left_value}"),
        };

        right_vec_values_computed.push(*min_value);

        acc + left_value.abs_diff(*min_value)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_puzzle_and_extract_two_vectors() {
        assert_eq!(extract_puzzle("tests/resources/puzzle.txt"), Puzzle {
            left: vec![3, 4, 2, 1, 3, 3],
            right: vec![4, 3, 5, 3, 9, 3]
        })
    }

    #[test]
    fn should_find_distance_with_one_line() {
        assert_eq!(find_distance(Puzzle {
            left: vec![1],
            right: vec![2]
        }), 1)
    }

    #[test]
    fn should_find_distance_with_two_lines() {
        assert_eq!(find_distance(Puzzle {
            left: vec![1, 1],
            right: vec![2, 2]
        }), 2)
    }

    #[test]
    fn should_find_distance_with_two_lines_absolute() {
        assert_eq!(find_distance(Puzzle {
            left: vec![2, 2],
            right: vec![1, 1]
        }), 2)
    }

    #[test]
    fn should_find_distance_right_not_sorted() {
        assert_eq!(find_distance(Puzzle {
            left: vec![2, 3, 5],
            right: vec![3, 5, 4]
        }), 2)
    }

    #[test]
    fn should_find_distance_left_and_right_not_sorted() {
        assert_eq!(find_distance(Puzzle {
            left: vec![5, 3, 2],
            right: vec![3, 5, 4]
        }), 2)
    }
}