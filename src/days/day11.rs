use hashbrown::HashMap;

use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let mut stones = input
        .split(' ')
        .map(|w| (w.parse().unwrap(), 1))
        .collect::<HashMap<_, _>>();

    let mut p1 = 0;
    for i in 0..75 {
        if i == 25 {
            p1 = stones.values().sum();
        }
        stones = update(&stones);
    }
    let p2 = stones.values().sum::<usize>();
    (Solution::from(p1), Solution::from(p2))
}

fn update(old_state: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut state = HashMap::with_capacity(old_state.len());
    for (&value, &count) in old_state {
        match value {
            0 => *state.entry(1).or_default() += count,
            _ => {
                let digits = value.ilog10() + 1;
                if digits % 2 == 0 {
                    let (left, right) = split(value, digits);
                    *state.entry(left).or_default() += count;
                    *state.entry(right).or_default() += count;
                } else {
                    *state.entry(value * 2024).or_default() += count
                }
            }
        }
    }
    state
}

#[inline]
fn split(value: u64, digits: u32) -> (u64, u64) {
    let divisor = 10u64.pow(digits / 2);
    let left = value / divisor;
    let right = value % divisor;
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day11/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(55312));
        assert_eq!(p2, Solution::Usize(65601038650482));
    }
}
