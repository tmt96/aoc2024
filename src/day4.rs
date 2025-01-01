use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    fn get_day(&self) -> i32 {
        4
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .map_while(Result::ok)
            .map(|l| l.chars().collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        // horizontal
        let horizontal_count: usize = input
            .iter()
            .map(|line| {
                line.windows(4)
                    .filter(|&window| {
                        window == &['X', 'M', 'A', 'S'] || window == &['S', 'A', 'M', 'X']
                    })
                    .count()
            })
            .sum();

        let vertical_count: usize = input
            .windows(4)
            .map(|lines| {
                (0..lines[0].len())
                    .filter(|&i| {
                        (lines[0][i] == 'X'
                            && lines[1][i] == 'M'
                            && lines[2][i] == 'A'
                            && lines[3][i] == 'S')
                            || (lines[0][i] == 'S'
                                && lines[1][i] == 'A'
                                && lines[2][i] == 'M'
                                && lines[3][i] == 'X')
                    })
                    .count()
            })
            .sum();

        let downward_diagonal_count: usize = input
            .windows(4)
            .map(|lines| {
                (0..lines[0].len() - 3)
                    .filter(|&i| {
                        (lines[0][i] == 'X'
                            && lines[1][i + 1] == 'M'
                            && lines[2][i + 2] == 'A'
                            && lines[3][i + 3] == 'S')
                            || (lines[0][i] == 'S'
                                && lines[1][i + 1] == 'A'
                                && lines[2][i + 2] == 'M'
                                && lines[3][i + 3] == 'X')
                    })
                    .count()
            })
            .sum();

        let upward_diagonal_count: usize = input
            .windows(4)
            .map(|lines| {
                (3..lines[0].len())
                    .filter(|&i| {
                        (lines[0][i] == 'X'
                            && lines[1][i - 1] == 'M'
                            && lines[2][i - 2] == 'A'
                            && lines[3][i - 3] == 'S')
                            || (lines[0][i] == 'S'
                                && lines[1][i - 1] == 'A'
                                && lines[2][i - 2] == 'M'
                                && lines[3][i - 3] == 'X')
                    })
                    .count()
            })
            .sum();

        horizontal_count + vertical_count + downward_diagonal_count + upward_diagonal_count
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .windows(3)
            .map(|lines| {
                (0..lines[0].len() - 2)
                    .filter(|&i| {
                        lines[1][i + 1] == 'A'
                            && ((lines[0][i] == 'M' && lines[2][i + 2] == 'S')
                                || (lines[0][i] == 'S' && lines[2][i + 2] == 'M'))
                            && ((lines[0][i + 2] == 'M' && lines[2][i] == 'S')
                                || (lines[0][i + 2] == 'S' && lines[2][i] == 'M'))
                    })
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 18);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 9);
    }
}
