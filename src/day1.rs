use aoc_runner_derive::*;

/// A box storing all meals, snacks, etc. and the position the box is in within the elves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalorieBox {
    position: usize,
    calories: Vec<u32>,
}

impl CalorieBox {
    #[allow(dead_code)]
    pub fn new(position: usize, calories: &[u32]) -> CalorieBox {
        CalorieBox { position, calories: Vec::from(calories) }
    }

    /// Generate a total of all calories within the box
    pub fn total(&self) -> u32 {
        self.calories.iter().sum()
    }
}

/// Given an input in the form of numbers across lines, transform that input into a series of
/// CalorieBoxes.
///
/// # Example
///
/// ```rust
/// # use aoc::day1::*;
/// let given_calories = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
/// let boxes = input_generator(given_calories);
/// let manual = vec![
///     CalorieBox::new(0, &[1000, 2000, 3000]),
///     CalorieBox::new(1, &[4000]),
///     CalorieBox::new(2, &[5000, 6000]),
///     CalorieBox::new(3, &[7000, 8000, 9000]),
///     CalorieBox::new(4, &[10000]),
/// ];
/// assert_eq!(boxes, manual);
/// ```
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

/// Get the largest box from a slice of CalorieBox.
///
/// # Panics
///
/// If the input contains no boxes, the function will panic.
///
/// # Example
///
/// ```rust
/// # use aoc::day1::*;
/// let given_calories = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
/// let boxes = input_generator(given_calories);
/// assert_eq!(get_largest_box(&boxes).total(), 24000);
/// ```
pub fn get_largest_box(input: &[CalorieBox]) -> &CalorieBox {
    input.iter().max_by(|a, b| a.total().partial_cmp(&b.total()).unwrap()).expect("empty input")
}

/// Get the largest set of boxes from a slice of CalorieBox.
///
/// # Example
///
/// ```rust
/// # use aoc::day1::*;
/// let given_calories = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
/// let boxes = input_generator(given_calories);
/// let total = sum_boxes(&get_largest_boxes(&boxes, 3));
/// assert_eq!(total, 45000);
/// ```
pub fn get_largest_boxes(input: &[CalorieBox], count: usize) -> Vec<CalorieBox> {
    let mut boxes = Vec::from(input);
    boxes.sort_by(|a, b| b.total().partial_cmp(&a.total()).unwrap());
    boxes.truncate(count);
    boxes
}

/// Get the sum of all calories in the given boxes.
///
/// # Example
///
/// ```rust
/// # use aoc::day1::*;
/// let given_calories = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
/// let boxes = input_generator(given_calories);
/// let total = sum_boxes(&get_largest_boxes(&boxes, 3));
/// assert_eq!(total, 45000);
/// ```
pub fn sum_boxes(input: &[CalorieBox]) -> u32 {
    input.iter().map(|b| b.total()).sum::<u32>()
}

#[doc(hidden)]
#[aoc(day1, part1)]
pub fn solve_part1(input: &[CalorieBox]) -> String {
    get_largest_box(input).total().to_string()
}

#[doc(hidden)]
#[aoc(day1, part2)]
pub fn solve_part2(input: &[CalorieBox]) -> String {
    sum_boxes(&get_largest_boxes(input, 3)).to_string()
}
