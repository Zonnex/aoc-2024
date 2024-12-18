use std::collections::{BinaryHeap, VecDeque};

use hashbrown::{HashMap, HashSet};

use crate::{
    utils::vector_2d::{self, Vector2},
    Solution, SolutionPair,
};

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
    start: Vector2,
    exit: Vector2,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .rev()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let start = Vector2::new_usize(1, 1);
        let exit = Vector2::new_usize(grid.len() - 2, grid[0].len() - 2);

        Self { grid, start, exit }
    }

    fn get(&self, p: Vector2) -> Option<&u8> {
        let y = p.row_index();
        let x = p.column_index();

        self.grid.get(y).and_then(|row| row.get(x))
    }

    fn print(&self) {
        for row in self.grid.iter().rev() {
            for &v in row {
                print!("{}", v as char);
            }
            println!();
        }
        println!();
    }
}

const TURNCOST: isize = 1000;

pub fn solve(input: &str) -> SolutionPair {
    let grid = Grid::new(input);

    // Your solution here...
    let rotate_left = |v| match v {
        vector_2d::N => vector_2d::W,
        vector_2d::E => vector_2d::N,
        vector_2d::S => vector_2d::E,
        vector_2d::W => vector_2d::S,
        _ => unreachable!(),
    };

    let rotate_right = |v| match v {
        vector_2d::N => vector_2d::E,
        vector_2d::E => vector_2d::S,
        vector_2d::S => vector_2d::W,
        vector_2d::W => vector_2d::N,
        _ => unreachable!(),
    };

    let mut q = BinaryHeap::new();
    let mut seen = HashMap::new();

    let mut p1 = 0;

    q.push((0_isize, grid.start, vector_2d::E));

    while let Some((score, current, dir)) = q.pop() {
        if let Some(b'E') = grid.get(current) {
            p1 = score.abs() as usize;
            continue;
        }
        
        let best = seen.get(&(current, dir)).copied().unwrap_or(isize::MAX);
        
        if best <= score {
            continue;
        }
        
        seen.insert((current, dir), score);

        let left = current.left(dir);
        if matches!(grid.get(left).unwrap(), b'.' | b'E') {
            q.push((score - TURNCOST - 1, left, rotate_left(dir)));
        }

        let right = current.right(dir);
        if matches!(grid.get(right).unwrap(), b'.' | b'E') {
            q.push((score - TURNCOST - 1, right, rotate_right(dir)));
        }

        let forward = current + dir;
        if matches!(grid.get(forward).unwrap(), b'.' | b'E') {
            q.push((score - 1, forward, dir));
        }
    }

    // traverse from the exit, find all paths until we reach the start
    let nodes: HashSet<Vector2> = HashSet::new();
    let mut q = VecDeque::new();

    for dir in [vector_2d::N, vector_2d::E, vector_2d::S, vector_2d::W].iter() {
        if seen.get(&(grid.exit, *dir)).copied().unwrap_or(isize::MAX) == p1 as isize {
            q.push_back((grid.exit, *dir, p1));
        }
    }
    let p2 = 0;

    (Solution::from(p1), Solution::from(p2))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day16/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(7036));
        assert_eq!(p2, Solution::Usize(45));
    }
}
