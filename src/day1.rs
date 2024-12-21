use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<i64>, Vec<i64>);
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        1
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .map_while(|line| {
                let binding = line.unwrap();
                let vec: Vec<i64> = binding
                    .split_whitespace()
                    .take(2)
                    .flat_map(|s| s.parse())
                    .collect();
                Some((vec[0], vec[1]))
            })
            .unzip()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (mut vec1, mut vec2) = input.clone();
        vec1.sort();
        vec2.sort();
        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (vec1, vec2) = input;
        let mut snd_counter: HashMap<&i64, i64> = HashMap::new();
        for i in vec2 {
            snd_counter.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }

        vec1.iter()
            .map(|i| *i * *snd_counter.get(i).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 11);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 31);
    }
}
