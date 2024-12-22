use regex::Regex;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        3
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut buf = String::new();
        let mut r = BufReader::new(r);
        let _ = r.read_to_string(&mut buf);
        buf
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
        re.captures_iter(input)
            .map(|c| {
                c.extract::<2>()
                    .1
                    .into_iter()
                    .flat_map(|num| num.parse::<i64>())
                    .product::<i64>()
            })
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|do\(\)|don\'t\(\)").unwrap();
        let mut should_read = true;
        let mut sum = 0;
        for capture in re.captures_iter(input) {
            let s = capture.get(0).unwrap().as_str();
            if s == "do()" {
                should_read = true;
            } else if s == "don't()" {
                should_read = false;
            } else if should_read {
                sum += capture.get(1).unwrap().as_str().parse::<i64>().unwrap()
                    * capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 161);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 48);
    }
}
