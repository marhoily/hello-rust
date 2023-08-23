use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Round {
    opponents_move: Move,
    desired_outcome: Outcome,
}
#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 1,
    Loss = 2,
    Draw = 0,
}
impl Outcome {
    fn score(self: Outcome) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
    fn parse(c: char) -> Option<Self> {
        match c {
            'X' => Some(Outcome::Loss),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }
}
impl Round {
    fn score(&self) -> i32 {
        let your_move = self.opponents_move.find_move_for(self.desired_outcome);
        println!(
            "Opponent's move: {:?}, Desired Outcome: {:?}, Your move: {:?}",
            self.opponents_move, self.desired_outcome, your_move
        );
        your_move.score() + self.desired_outcome.score()
    }
}
#[derive(Debug, Clone, Copy, FromPrimitive)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Move {
    fn score(self: Move) -> i32 {
        self as i32
    }
    fn find_move_for(self: Move, desired_outcome: Outcome) -> Move {
        let diff = self as i32 - 1 + desired_outcome as i32;
        let reminder = diff.rem_euclid(3) + 1;
        Move::from_i32(reminder)
            .expect(format!("ERR: diff: {:?}; reminder: {:?}", diff, reminder).as_str())
    }
    fn parse(c: char) -> Option<Self> {
        match c {
            'A' => Some(Move::Rock),
            'B' => Some(Move::Paper),
            'C' => Some(Move::Scissors),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct Input(Vec<Round>);
impl Input {
    pub fn solve(&self) -> Result<u32, Box<dyn Error>> {
        Ok(self.0.iter().map(|r| r.score() as u32).sum())
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let result = text
            .lines()
            .map(|l| l.parse::<Round>().unwrap())
            .collect::<Vec<_>>();
        Ok(Input(result))
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let x = line
            .chars()
            .nth(0)
            .map_or(None, Move::parse)
            .ok_or("first value in line is not found")?;
        let y = line
            .chars()
            .nth(2)
            .map_or(None, Outcome::parse)
            .ok_or("second value in line is not found")?;
        Ok(Round {
            opponents_move: x,
            desired_outcome: y,
        })
    }
}
