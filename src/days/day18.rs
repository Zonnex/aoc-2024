use std::{collections::VecDeque, vec};

use hashbrown::HashSet;
use itertools::Itertools;

use crate::{Solution, SolutionPair};

type Vector2 = (usize, usize);

#[derive(Debug, Default, Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn of_size(size: usize) -> Self {
        let grid = vec![vec![b'.'; size]; size];

        Self { grid }
    }

    fn get(&self, (x, y): Vector2) -> Option<u8> {
        self.grid.get(y).and_then(|row| row.get(x).copied())
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.grid[y][x] = value;
    }
}

pub fn solve(input: &str) -> SolutionPair {
    // Your solution here...
    let mut grid = Grid::of_size(71);

    let mut coords = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().unwrap())
        .tuples();

    for _ in 0..1024 {
        let (x, y) = coords.next().unwrap();
        grid.set(x, y, b'#');
    }

    let p1 = bfs(&grid, (70, 70), (0, 0)).unwrap();
    
    /*
        p2 could be massively improved by recording the shortest path from p1, and then recalculate the path only when the instructions overlap with the shortest path.
    */
    let mut grid = grid.clone();
    let mut p2 = String::from("(0,0)");
    for (x, y) in coords {
        grid.set(x, y, b'#');
        if dfs(&grid, (70, 70), (0, 0)).is_none() {
            p2 = format!("{},{}", x, y);
            break;
        }
    }

    (Solution::from(p1), Solution::Str(p2))
}

fn bfs(grid: &Grid, start: Vector2, end: Vector2) -> Option<usize> {
    let mut q = VecDeque::from([(0, start)]);
    let mut visited = HashSet::new();

    while let Some((steps, p)) = q.pop_front() {
        if p == end {
            return Some(steps);
        }

        if !visited.insert(p) {
            continue;
        }

        let (x, y) = p;
        for next in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
            if let Some(b'.') = grid.get(next) {
                q.push_back((steps + 1, next));
            }
        }
    }

    None
}

fn dfs(grid: &Grid, start: Vector2, end: Vector2) -> Option<usize> {
    let mut q = vec![(0, start)];
    let mut visited = HashSet::new();

    while let Some((steps, p)) = q.pop() {
        if p == end {
            return Some(steps);
        }

        if !visited.insert(p) {
            continue;
        }

        let (x, y) = p;
        for next in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
            if let Some(b'.') = grid.get(next) {
                q.push((steps + 1, next));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day18/test.txt");

        let start = (0, 0);
        let end = (6, 6);
        let instructions = input
            .lines()
            .map(|l| {
                l.split_once(',')
                    .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let mut grid = Grid::of_size(7);
        for instruction in instructions.iter().take(12) {
            grid.set(instruction.0, instruction.1, b'#');
        }
        let steps = super::bfs(&grid, start, end);

        assert_eq!(steps, Some(22));

        let mut grid = grid.clone();

        let mut p2 = String::from("(0,0)");
        for &(x, y) in instructions.iter().skip(12) {
            grid.set(x, y, b'#');
            if let None = bfs(&grid, start, end) {
                p2 = format!("{},{}", x, y);
                break;
            }
        }

        assert_eq!(p2, "6,1");
    }
}
