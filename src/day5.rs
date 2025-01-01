use crate::solver::Solver;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
};

pub struct Problem;

pub struct Input {
    rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

impl Solver for Problem {
    type Input = Input;
    type Output1 = i64;
    type Output2 = i64;

    fn get_day(&self) -> i32 {
        5
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut read_rules = true;
        let mut rules = vec![];
        let mut updates = vec![];

        let r = BufReader::new(r);
        for line in r.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                read_rules = false;
            } else if read_rules {
                let mut nums = line.split('|').map(|s| s.parse::<i64>().unwrap());
                rules.push((nums.next().unwrap(), nums.next().unwrap()));
            } else {
                let nums: Vec<_> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
                updates.push(nums);
            }
        }

        Self::Input { rules, updates }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut successor_map = HashMap::new();
        for (fst, snd) in &input.rules {
            successor_map
                .entry(fst)
                .or_insert_with(HashSet::new)
                .insert(snd);
        }

        input
            .updates
            .iter()
            .filter(|update| {
                for i in (0..update.len()).rev() {
                    for j in 0..i {
                        if successor_map.contains_key(&update[i])
                            && successor_map[&update[i]].contains(&update[j])
                        {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut successor_map: HashMap<i64, HashSet<i64>> = HashMap::new();
        for (fst, snd) in &input.rules {
            let snd_successor_list = if successor_map.contains_key(snd) {
                successor_map[snd].clone()
            } else {
                HashSet::new()
            };
            let fst_successor_list = successor_map.entry(*fst).or_default();

            fst_successor_list.insert(*snd);
            for i in snd_successor_list.iter() {
                fst_successor_list.insert(*i);
            }
        }

        input
            .updates
            .iter()
            .filter(|update| {
                for i in (0..update.len()).rev() {
                    for j in 0..i {
                        if successor_map.contains_key(&update[i])
                            && successor_map[&update[i]].contains(&update[j])
                        {
                            return true;
                        }
                    }
                }
                false
            })
            .fold(0, |acc, update| {
                let mut update = update.clone();
                update.sort_by(|a, b| {
                    if successor_map.contains_key(a) && successor_map[a].contains(b) {
                        Ordering::Less
                    } else if successor_map.contains_key(b) && successor_map[b].contains(a) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                acc + update[update.len() / 2]
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_first() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_first(&input), 143);
    }

    #[test]
    fn test_second() {
        let problem = Problem {};
        let input = problem.parse_input(INPUT.as_bytes());
        assert_eq!(problem.solve_second(&input), 123);
    }
}
