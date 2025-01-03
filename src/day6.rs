use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        6
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .map_while(Result::ok)
            .map(|l| l.chars().collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut position: (isize, isize) = (0, 0);
        let mut dir_index = 0;
        let mut all_pos = HashSet::new();

        for x in 0..input.len() {
            for y in 0..input[x].len() {
                if input[x][y] == '^' {
                    position = (x as isize, y as isize);
                }
            }
        }
        loop {
            let new_x = position.0 + directions[dir_index].0;
            let new_y = position.1 + directions[dir_index].1;
            if new_x < 0
                || new_x as usize >= input.len()
                || new_y < 0
                || new_y as usize >= input.len()
            {
                return all_pos.len();
            }
            if input[new_x as usize][new_y as usize] == '#' {
                dir_index = (dir_index + 1) % 4;
            } else {
                position = (new_x, new_y);
                all_pos.insert(position);
            }
        }
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 41);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 0);
    }
}
