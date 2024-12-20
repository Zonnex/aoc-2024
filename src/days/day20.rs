use std::collections::VecDeque;

use hashbrown::HashMap;
use itertools::Itertools;

use crate::{Solution, SolutionPair};

const WALL: u8 = b'#';
const START: u8 = b'S';
const END: u8 = b'E';

type Vector2 = (usize, usize);

pub fn solve(input: &str) -> SolutionPair {
    solve_inner(input, 100)
}

fn solve_inner(input: &str, threshold: usize) -> SolutionPair {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            match grid[y][x] {
                START => start = (x, y),
                END => end = (x, y),
                _ => {}
            }
        }
    }

    let (_steps, distances) = bfs(&grid, start, end);

    let (mut p1, mut p2) = (0_usize, 0_usize);
    for ((&a, &d1), (&b, &d2)) in distances.iter().tuple_combinations() {
        let d = manhattan(a, b);

        if d <= 20 && threshold <= time_save(d2, d1, d) {
            if d <= 2 {
                p1 += 1;
            }
            p2 += 1;
        }
    }

    (Solution::from(p1), Solution::from(p2))
}

fn time_save(d2: usize, d1: usize, d: usize) -> usize {
    d2.abs_diff(d1) - d
}

fn manhattan(a: Vector2, b: Vector2) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;

    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn bfs(map: &[&[u8]], start: Vector2, end: Vector2) -> (usize, HashMap<Vector2, usize>) {
    let mut q = VecDeque::from([(0, start)]);

    let mut distances = HashMap::new();

    while let Some((steps, (x, y))) = q.pop_front() {
        if distances.contains_key(&(x, y)) {
            continue;
        }
        distances.insert((x, y), steps);

        if (x, y) == end {
            return (steps, distances);
        }

        for (x2, y2) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
            match map.get(y2).and_then(|row| row.get(x2)) {
                Some(&WALL) => continue,
                Some(_) => q.push_back((steps + 1, (x2, y2))),
                _ => continue,
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day20/test.txt");

        let (p1, p2) = super::solve_inner(input, 2);
        assert_eq!(p1, Solution::Usize(44));
        assert_eq!(p2, Solution::Usize(3081));
    }
}
