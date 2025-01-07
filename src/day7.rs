use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(i64, Vec<i64>)>;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        7
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .map_while(Result::ok)
            .map(|line| {
                let (target, nums) = line.split_once(':').unwrap();
                (
                    target.parse::<i64>().unwrap(),
                    nums.split_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect(),
                )
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        fn match_target(target: i64, nums: &[i64]) -> bool {
            let len = nums.len();
            if target < 0 {
                false
            } else if target == 0 {
                nums.is_empty()
            } else if nums.is_empty() {
                false
            } else if match_target(target - nums[len - 1], &nums[0..(len - 1)]) {
                true
            } else if nums[len - 1] == 0 || target % nums[len - 1] != 0 {
                false
            } else {
                match_target(target / nums[len - 1], &nums[0..len - 1])
            }
        }

        input
            .iter()
            .filter(|(target, nums)| match_target(*target, nums))
            .map(|(target, _)| target)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        fn match_target(target: i64, nums: &[i64]) -> bool {
            let len = nums.len();
            if target < 0 {
                false
            } else if target == 0 {
                nums.is_empty()
            } else if nums.is_empty() {
                false
            } else if match_target(target - nums[len - 1], &nums[0..(len - 1)]) {
                true
            } else if match_target(
                target / 10i64.pow(nums[len - 1].ilog10() + 1),
                &nums[0..(len - 1)],
            ) && target % 10i64.pow(nums[len - 1].ilog10() + 1) == nums[len - 1]
            {
                true
            } else if nums[len - 1] == 0 || target % nums[len - 1] != 0 {
                false
            } else {
                match_target(target / nums[len - 1], &nums[0..len - 1])
            }
        }

        input
            .iter()
            .filter(|(target, nums)| match_target(*target, nums))
            .map(|(target, _)| target)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 3749);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 11387);
    }
}
