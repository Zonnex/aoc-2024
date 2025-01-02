use hashbrown::HashSet;
use itertools::Itertools;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let edges = input
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-').unwrap();

            [(a, b), (b, a)]
        })
        .collect::<HashSet<_>>();

    let mut computers = HashSet::new();
    for (a, b) in &edges {
        computers.insert(*a);
        computers.insert(*b);
    }

    let mut sorted = computers.into_iter().collect::<Vec<_>>();
    sorted.sort();

    let mut p1 = 0_usize;
    for (a, b, c) in sorted.into_iter().tuple_combinations() {
        if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
            if edges.contains(&(a, b)) && edges.contains(&(b, c)) && edges.contains(&(a, c)) {
                p1 += 1;
            }
        }
    }

    let p2 = 0_usize;

    (Solution::from(p1), Solution::from(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day23/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(7));
        assert_eq!(p2, Solution::Usize(0));
    }
}
