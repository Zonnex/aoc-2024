use std::collections::HashSet;

use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let map = input
        .lines()
        .rev()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();

    let (mut p1, mut p2) = (0, 0_usize);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'0' {
                let trailheads = traverse(&map, (x, y));
                p1 += trailheads.iter().collect::<HashSet<_>>().len();
                p2 += trailheads.len();
            }
        }
    }

    (Solution::from(p1), Solution::from(p2))
}

fn traverse(map: &[&[u8]], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut stack = vec![start];
    let mut seen = Vec::new();
    while let Some((x, y)) = stack.pop() {
        if map[y][x] == b'9' {
            seen.push((x, y));
            continue;
        }

        let current_height = map[y][x];

        for (x2, y2) in [(x, y+1), (x, y-1), (x+1, y), (x-1, y)] {
            if let Some(tile) = map.get(y2).and_then(|row| row.get(x2)) {
                if *tile - current_height == 1 {
                    stack.push((x2, y2));
                }
            }
        }
    }

    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day10/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(36));
        assert_eq!(p2, Solution::Usize(81));
    }
}
