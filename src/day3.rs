use std::str::FromStr;

use crate::code_advent::Problem;

pub struct Subject();

#[derive(Debug)]
pub struct Input(Vec<Pack>);
#[derive(Debug)]
struct Pack {
    pub c1: String,
    pub c2: String,
}

impl Problem for Subject {
    type Input = Input;

    const DAY: u8 = 3;

    fn solve(input: Self::Input) -> Result<u32, Box<dyn std::error::Error>> {
        let mut acc = 0;
        for r in input.0 {
            for c in r.c1.chars() {
                if r.c2.contains(c) {
                    let var_name = if char::is_lowercase(c) {
                        c as u32 - 'a' as u32 + 1
                    } else {
                        c as u32 - 'A' as u32 + 27
                    };
                    acc += var_name;
                    break;
                }
            }
        }
        Ok(acc)
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<Vec<_>, _> = s.lines().map(|l| l.parse::<Pack>()).collect();
        Ok(Input(result?))
    }
}
impl FromStr for Pack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        assert!(len > 0);
        assert!(len % 2 == 0);
        let (l, r) = s.split_at(len / 2);
        Ok(Pack {
            c1: l.to_string(),
            c2: r.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::code_advent::Problem;

    use super::{Input, Subject};

    fn solve(input: &str) -> u32 {
        Subject::solve(input.parse::<Input>().unwrap()).unwrap()
    }
    #[test]
    fn test_p() {
        assert_eq!(16, solve("vJrwpWtwJgWrhcsFMMfFFhFp"));
    }
    #[test]
    fn test_capital_l() {
        assert_eq!(38, solve("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    }
    #[test]
    fn test_list() {
        assert_eq!(157, solve("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"));
    }
}
