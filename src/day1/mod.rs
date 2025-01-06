use std::{collections::HashMap, fs::read_to_string, iter::zip};

pub fn total_distance(input: &str) -> u32 {
    let file = read_to_string(input).expect("error reading file");
    let lines = file.split("\n");
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in lines {
        let line_str = line.to_string();
        let mut line_columns = line_str.split_whitespace();
        let left_num = line_columns
            .next()
            .map(|left| left.parse::<u32>())
            .expect("should have found a left number string");
        let right_num = line_columns
            .next()
            .map(|right| right.parse::<u32>())
            .expect("should have found a right number string");
        left.push(left_num.expect("should have parsed the left number"));
        right.push(right_num.expect("should have parsed the right number"));
    }

    left.sort();
    right.sort();

    zip(left.iter(), right.iter()).fold(0, |sum, pair| sum + pair.0.abs_diff(*pair.1))
}

pub fn similarity_score(input: &str) -> u32 {
    let file = read_to_string(input).expect("error reading file");
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    let lines = file.split("\n");
    for line in lines {
        let line_str = line.to_string();
        let mut line_columns = line_str.split_whitespace();
        let left_num = line_columns
            .next()
            .map(|left| left.parse::<u32>())
            .expect("should have found a left number string");
        left.push(left_num.unwrap());

        let right_num = line_columns
            .next()
            .map(|right| right.parse::<u32>())
            .expect("should have found a right number string")
            .unwrap();
        right
            .entry(right_num)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    left.iter().fold(0, |sum, val| {
        let count = match right.get(val) {
            Some(val) => val,
            None => &0,
        };
        sum + (val * count)
    })
}

#[cfg(test)]
pub mod tests {
    use crate::day1::{similarity_score, total_distance};

    #[test]
    pub fn test_day1_test_input() {
        assert_eq!(total_distance("src/day1/test-input.txt"), 11);
    }

    #[test]
    pub fn test_day1_real_input() {
        assert_eq!(total_distance("src/day1/real-input.txt"), 2378066);
    }

    #[test]
    pub fn test_day1_similarity_score_test_input() {
      assert_eq!(similarity_score("src/day1/test-input.txt"), 31);
    }

    #[test]
    pub fn test_day1_similarity_score_real() {
      assert_eq!(similarity_score("src/day1/real-input.txt"), 18934359);
    }
}
