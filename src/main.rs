use reqwest::header::COOKIE;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Round {
    you: Move,
    opponent: Move,
}
impl Round {
    fn score(&self) -> u8 {
        let diff = self.you as i32 - self.opponent as i32;
        let outcome = diff.rem_euclid(3);
     
        let outcome_score = match outcome {
            0 => 3,
            1 => 6,
            _ => 0,
        };
        println!("{:?} {:?}", outcome_score, self.you as u8);
        outcome_score + self.you as u8
    }
}
#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
#[derive(Debug)]
struct Script(Vec<Round>);
impl Script {
    fn solve(&self) -> Result<u32, Box<dyn Error>> {
        Ok(self.0.iter().map(|r| r.score() as u32).sum())
    }
}

impl FromStr for Script {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let result = text
            .lines()
            .map(|l| l.parse::<Round>().unwrap())
            .collect::<Vec<_>>();
        Ok(Script(result))
    }
}
impl FromStr for Round {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let x = match line.chars().nth(0) {
            Some('A') => Some(Move::Rock),
            Some('B') => Some(Move::Paper),
            Some('C') => Some(Move::Scissors),
            _ => None,
        }
        .ok_or("first value in line is not found")?;
        let y = match line.chars().nth(2) {
            Some('X') => Some(Move::Rock),
            Some('Y') => Some(Move::Paper),
            Some('Z') => Some(Move::Scissors),
            _ => None,
        }
        .ok_or("second value in line is not found")?;
        Ok(Round {
            you: y,
            opponent: x,
        })
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?
        .get("https://adventofcode.com/2022/day/2/input")
        .header(COOKIE, "_ga=GA1.2.19743620.1692601480; _gid=GA1.2.1285583899.1692601480; _ga_MHSNPJKWC7=GS1.2.1692601480.1.1.1692601713.0.0.0; session=53616c7465645f5f006f509444acba3e035c1eeaa9454bae98c37ad96581fa02443b287ce297ad47457335dd716b287153637ae1250715adbac4f02fa963ce35")
        .send()?;

    let result = if resp.status().is_success() {
        resp.text()?.parse::<Script>()?.solve()
    } else {
        let err = resp.text()?;
        Err(err.into())
    };
    print!("{:?}", result);
    Ok(())
}
