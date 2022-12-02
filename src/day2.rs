use anyhow::{anyhow, Error, Result};
use aoc_runner_derive::*;

/// Either a Win, Tie, or Loss.
#[derive(Clone, Debug)]
pub enum PlayResult {
    Win,
    Tie,
    Loss,
}

/// A choice between Rock, Paper, and Scissors
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum Play {
    Rock = 1,
    Paper,
    Scissors,
}

impl Play {
    /// Given a separate Play, determine whether or not a game between self and the other play
    /// would result in a Tie, Win, or Loss.
    pub fn wins_against(self: &Play, against: &Play) -> PlayResult {
        match (against, self) {
            (Play::Rock, Play::Rock) => PlayResult::Tie,
            (Play::Rock, Play::Paper) => PlayResult::Win,
            (Play::Rock, Play::Scissors) => PlayResult::Loss,
            (Play::Paper, Play::Rock) => PlayResult::Loss,
            (Play::Paper, Play::Paper) => PlayResult::Tie,
            (Play::Paper, Play::Scissors) => PlayResult::Win,
            (Play::Scissors, Play::Rock) => PlayResult::Win,
            (Play::Scissors, Play::Paper) => PlayResult::Loss,
            (Play::Scissors, Play::Scissors) => PlayResult::Tie,
        }
    }

    /// Given a strategy of "X", return a losing play; a strategy of "Y", a tying play; a
    /// strategy of "Z", a winning play.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aoc::day2::*;
    /// let given_plays = "A Y\nB X\nC Z";
    /// let games = input_generator(given_plays);
    /// let points: u32 = games
    ///     .iter()
    ///     .map(|(play_left, play_right)|
    ///          Game(*play_left, Play::from_strategy(play_left, play_right)
    ///                           .expect("Unable to devise strategy")))
    ///     .map(|g| g.points())
    ///     .sum();
    /// assert_eq!(points, 12);
    /// ```
    pub fn from_strategy(from: &Play, strategy: &str) -> Result<Play> {
        match strategy {
            "X" => match from {
                Play::Rock => Ok(Play::Scissors),
                Play::Paper => Ok(Play::Rock),
                Play::Scissors => Ok(Play::Paper),
            }
            "Y" => Ok(*from),
            "Z" => match from {
                Play::Rock => Ok(Play::Paper),
                Play::Paper => Ok(Play::Scissors),
                Play::Scissors => Ok(Play::Rock),
            }
            p => Err(anyhow!("Was given an invalid play: {p}"))
        }
    }
}

impl std::str::FromStr for Play {
    type Err = Error;

    /// Determine a Play from an input. Note that both Plays and strategies to determine Plays can
    /// be represented by "X", "Y", and "Z".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            p => Err(anyhow!("Was given an invalid play: {p}")),
        }
    }
}

/// Two Plays pit against each other
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game(pub Play, pub Play);

impl Game {
    /// Points for the shape selected (rock: 1, paper: 2, scissors: 3) and the score for the
    /// outcome of the round, with 0 for a loss, 3 for a tie, and 6 for a win. The points are
    /// calculated from the right hand perspective of the game.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aoc::day2::*;
    /// let given_plays = "A Y\nB X\nC Z";
    /// let games = input_generator(given_plays);
    /// let points: u32 = games
    ///     .iter()
    ///     .map(|(play_left, play_right)|
    ///          Game(*play_left, play_right.parse().expect("invalid play")))
    ///     .map(|g| g.points())
    ///     .sum();
    /// assert_eq!(points, 15);
    /// ```
    pub fn points(&self) -> u32 {
        self.1 as u32
            + match self.1.wins_against(&self.0) {
                PlayResult::Win => 6,
                PlayResult::Tie => 3,
                PlayResult::Loss => 0,
            }
    }
}

/// Given an input in the form of Plays, where A, B, and C are the first player's Rock, Paper, or
/// Scissor, generate a list of Plays with an additional strategy on the right. The strategy can
/// either be parsed to a Play or used as a strategy to pick a winning/losing/drawing piece.
///
/// # Example
///
/// ```rust
/// # use aoc::day2::*;
/// let given_plays = "A Y\nB X\nC Z";
/// let games = input_generator(given_plays)
///     .iter()
///     .map(|(play_left, play_right)|
///          Game(*play_left, play_right.parse().expect("invalid play")))
///     .collect::<Vec<_>>();
/// assert_eq!(games, vec![
///     Game(Play::Rock, Play::Paper),
///     Game(Play::Paper, Play::Rock),
///     Game(Play::Scissors, Play::Scissors)
/// ]);
/// ```
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Play, String)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let play_left: Play = split
                .next()
                .expect("No split data was available")
                .parse()
                .expect("Was not given a valid play");
            let play_right = split
                .next()
                .expect("No split data was available")
                .to_string();
            (play_left, play_right)
        })
        .collect()
}

#[doc(hidden)]
#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Play, String)]) -> String {
    let total: u32 = input
        .iter()
        .map(|(play_left, play_right)| Game(*play_left, play_right.parse().expect("invalid play")))
        .map(|g| g.points())
        .sum();
    total.to_string()
}

#[doc(hidden)]
#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Play, String)]) -> String {
    let points: u32 = input
        .iter()
        .map(|(play_left, play_right)|
             Game(*play_left, Play::from_strategy(play_left, play_right)
                              .expect("Unable to devise strategy")))
        .map(|g| g.points())
        .sum();
    points.to_string()
}
