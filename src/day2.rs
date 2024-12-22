use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

fn is_safe(nums: &[i64]) -> bool {
    let increasing = nums[1] > nums[0];
    nums.windows(2).all(|chunk| {
        let diff = chunk[1] - chunk[0];
        if increasing {
            (1..=3).contains(&diff)
        } else {
            (-3..=-1).contains(&diff)
        }
    })
}

impl Solver for Problem {
    type Input = Vec<Vec<i64>>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        2
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .map(|line| {
                line.unwrap()
                    .split_whitespace()
                    .flat_map(|s| s.parse())
                    .collect()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|nums| is_safe(nums)).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|nums| {
                if is_safe(nums) {
                    return true;
                }

                (0..nums.len()).any(|i| is_safe(&[&nums[..i], &nums[i + 1..]].concat()))
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 2);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 4);
    }
}
