use crate::utils::vector_2d::*;
use crate::{Solution, SolutionPair};
///////////////////////////////////////////////////////////////////////////////

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    fn get(&self, pos: Vector2) -> Option<char> {
        self.0
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize).copied())
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<char>> {
        self.0.iter()
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let grid = Grid::new(input);

    let p1 = p1(&grid);
    let p2 = p2(&grid);

    (Solution::from(p1), Solution::from(p2))
}

fn p1(grid: &Grid) -> u32 {
    let mut sum = 0;
    for (y, column) in grid.iter().enumerate() {
        for (x, letter) in column.iter().enumerate() {
            if letter == &'X' {
                let p = Vector2::new_usize(x, y);
                for dir in DIRS {
                    sum += check_dir(p, dir, grid);
                }
            }
        }
    }

    sum
}

fn check_dir(p: Vector2, dir: Vector2, grid: &Grid) -> u32 {
    let mut buffer = [None; 4];
    for (i, element) in buffer.iter_mut().enumerate() {
        let offset = dir * i;
        *element = grid.get(p + offset);
    }

    if buffer[0] == Some('X')
        && buffer[1] == Some('M')
        && buffer[2] == Some('A')
        && buffer[3] == Some('S')
    {
        return 1;
    }

    0
}

fn p2(grid: &Grid) -> u32 {
    let mut sum = 0;
    for (y, column) in grid.iter().enumerate() {
        for (x, letter) in column.iter().enumerate() {
            if letter == &'A' {
                let p = Vector2::new_usize(x, y);
                let mut corners = [None; 4];
                for i in 0..4 {
                    let diagonal = DIAGONALS[i];
                    corners[i] = grid.get(p + diagonal);
                }

                let only_ms = || corners.iter().all(|&c| matches!(c, Some('M') | Some('S')));
                let check_diagonals = || corners[0] != corners[2] && corners[1] != corners[3];

                if only_ms() && check_diagonals() {
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day04/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::U32(18));
        assert_eq!(p2, Solution::U32(9));
    }
}
