use hashbrown::HashSet;

use crate::{Solution, SolutionPair};

type Pair = (u32, u32);
enum Instruction {
    Keep,
    Swap,
}

struct Sorter {
    instructions: HashSet<Pair>,
}

impl Sorter {
    fn check(&self, pair: Pair) -> Instruction {
        let swapped = (pair.1, pair.0);
        match self.instructions.contains(&swapped) {
            true => Instruction::Swap,
            false => Instruction::Keep,
        }
    }

    fn sort(&self, mut values: Vec<u32>) -> Vec<u32> {
        while self.check_line(values.as_slice()).is_none() {
            for i in 0..values.len()-1 {
                let pair = (values[i], values[i + 1]);
                match self.check(pair) {
                    Instruction::Swap => {
                        values.swap(i, i+1);
                    }
                    Instruction::Keep => {}
                }
            }
        }
        values
    }

    fn check_line(&self, values: &[u32]) -> Option<u32> {
        for i in 0..values.len()-1 {
            let pair = (values[i], values[i + 1]);
            match self.check(pair) {
                Instruction::Swap => {
                    return None;
                }
                Instruction::Keep => {}
            }
        }
        let middle = values.len() / 2;
        Some(values[middle])
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let (sorting_input, rows) = input.split_once("\n\n").unwrap();
    let sorting = sorting_input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once('|').unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect::<HashSet<_>>();

    let sorter = Sorter {
        instructions: sorting,
    };

    let mut p1 = Vec::new();
    let mut p2 = Vec::new();
    for line in rows.lines() {
        let nums = line
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        match sorter.check_line(&nums) {
            Some(num) => {
                p1.push(num);
            }
            None => {
                let nums = sorter.sort(nums);
                let answer = nums[nums.len() / 2];
                p2.push(answer);
            }
        }
    }
    let p1 = p1.iter().sum::<u32>();
    let p2 = p2.iter().sum::<u32>();

    (Solution::from(p1), Solution::from(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day05/test.txt");
        let (part1, part2) = super::solve(input);
        assert_eq!(part1, Solution::U32(143));
        assert_eq!(part2, Solution::U32(123));
    }

    #[test]
    fn test_lines() {
        let input = include_str!("../../input/day05/test.txt");
        let (sorting_input, _) = input.split_once("\n\n").unwrap();
        let sorting = sorting_input
            .lines()
            .map(|l| {
                let (left, right) = l.split_once('|').unwrap();
                (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
            })
            .collect::<HashSet<_>>();

        let sorter = Sorter {
            instructions: sorting,
        };

        assert_eq!(sorter.check_line(&[75, 47, 61, 53, 29]), Some(61));
        assert_eq!(sorter.check_line(&[97, 61, 53, 29, 13]), Some(53));
        assert_eq!(sorter.check_line(&[75, 29, 13]), Some(29));
        assert_eq!(sorter.check_line(&[75, 97, 47, 61, 53]), None);
        assert_eq!(sorter.check_line(&[61, 13, 29]), None);
        assert_eq!(sorter.check_line(&[97, 13, 75, 29, 47]), None);
    }

    #[test]
    fn test_sort_line() {
        let input = include_str!("../../input/day05/test.txt");
        let (sorting_input, _) = input.split_once("\n\n").unwrap();
        let sorting = sorting_input
            .lines()
            .map(|l| {
                let (left, right) = l.split_once('|').unwrap();
                (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
            })
            .collect::<HashSet<_>>();

        let sorter = Sorter {
            instructions: sorting,
        };

        assert_eq!(sorter.sort(vec![61, 13, 29]), vec![61, 29, 13]);
        assert_eq!(
            sorter.sort(vec![75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            sorter.sort(vec![97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }
}
