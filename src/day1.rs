use aoc_runner_derive::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalorieBox {
    pub position: usize,
    pub calories: Vec<u32>,
}

impl CalorieBox {
    pub fn total(&self) -> u32 {
        self.calories.iter().sum()
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<CalorieBox> {
    let mut boxes = vec![];
    let mut calories = vec![];
    for line in input.lines() {
        if line.is_empty() {
            boxes.push(CalorieBox { position: boxes.len(), calories });
            calories = vec![];
        } else {
            calories.push(line.parse().expect("couldn't parse"));
        }
    }
    boxes.push(CalorieBox { position: boxes.len(), calories });
    boxes
}

pub fn get_largest_box(input: &[CalorieBox]) -> &CalorieBox {
    input.iter().max_by(|a, b| a.total().partial_cmp(&b.total()).unwrap()).expect("empty input")
}

pub fn get_largest_boxes(input: &[CalorieBox], count: usize) -> Vec<CalorieBox> {
    let mut boxes = Vec::from(input);
    boxes.sort_by(|a, b| b.total().partial_cmp(&a.total()).unwrap());
    boxes.truncate(count);
    boxes
}

pub fn sum_boxes(input: &[CalorieBox]) -> u32 {
    input.iter().map(|b| b.total()).sum::<u32>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[CalorieBox]) -> String {
    get_largest_box(input).total().to_string()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[CalorieBox]) -> String {
    sum_boxes(&get_largest_boxes(input, 3)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

        const GIVEN_CALORIES: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn parse_given_input() {
        // Take input from Advent of Code
        let boxes = input_generator(GIVEN_CALORIES);
        let manual = vec![
            CalorieBox { position: 0, calories: vec![1000, 2000, 3000] },
            CalorieBox { position: 1, calories: vec![4000] },
            CalorieBox { position: 2, calories: vec![5000, 6000] },
            CalorieBox { position: 3, calories: vec![7000, 8000, 9000] },
            CalorieBox { position: 4, calories: vec![10000] },
        ];
        assert_eq!(boxes, manual);
    }

    #[test]
    fn get_given_max_calories() {
        let boxes = input_generator(GIVEN_CALORIES);
        assert_eq!(get_largest_box(&boxes).total(), 24000);
    }

    #[test]
    fn get_given_multiple_max_calories() {
        let boxes = input_generator(GIVEN_CALORIES);
        let total = sum_boxes(&get_largest_boxes(&boxes, 3));
        assert_eq!(total, 45000);
    }
}
