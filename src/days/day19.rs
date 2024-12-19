#![allow(unused_mut)]
use hashbrown::HashMap;

use crate::{Solution, SolutionPair};

type Cache<'a> = HashMap<&'a [u8], usize>;

pub fn solve(input: &str) -> SolutionPair {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(str::as_bytes).collect::<Vec<_>>();

    let (mut p1, mut p2) = (0, 0);

    for pattern in patterns.lines() {
        let mut cache = HashMap::new();
        let paths = dfs(pattern.as_bytes(), &towels, &mut cache);
        if paths > 0 {
            p1 += 1;
            p2 += paths;
        }
    }

    (Solution::from(p1), Solution::from(p2))
}

fn dfs<'a>(remaining: &'a [u8], towels: &[&[u8]], cache: &mut Cache<'a>) -> usize {
    if remaining.is_empty() {
        return 1;
    }
    if let Some(&paths) = cache.get(&remaining) {
        return paths;
    }
    let paths = towels
        .iter()
        .filter(|towel| remaining.starts_with(towel))
        .map(|towel| {
            let remaining = &remaining[towel.len()..];
            dfs(remaining, towels, cache)
        })
        .sum();

    cache.insert(remaining, paths);

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day19/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(6));
        assert_eq!(p2, Solution::Usize(16));
    }
}
