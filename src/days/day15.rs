use crate::{Solution, SolutionPair};

const BOX: u8 = b'O';
const WIDE_BOX_LEFT: u8 = b'[';
const WIDE_BOX_RIGHT: u8 = b']';
const EMPTY: u8 = b'.';
const WALL: u8 = b'#';
const ROBOT: u8 = b'@';

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
    robot: (usize, usize),
}
impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        let robot = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &v)| {
                        if v == ROBOT {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        Self { grid, robot }
    }

    fn print(&self) {
        for row in &self.grid {
            for &v in row {
                print!("{}", v as char);
            }
            println!();
        }
        println!();
    }

    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.grid.get(y).and_then(|row| row.get(x))
    }

    fn set(&mut self, x: usize, y: usize, v: u8) {
        self.grid[y][x] = v;
    }

    fn move_robot(&mut self, instruction: u8) {
        let (x, y) = self.robot;
        let (x2, y2) = match instruction {
            b'^' => (x, y - 1),
            b'v' => (x, y + 1),
            b'>' => (x + 1, y),
            b'<' => (x - 1, y),
            _ => return,
        };

        match self.get(x2, y2) {
            Some(first) => {
                match *first {
                    EMPTY => {
                        // swap robot and empty
                        self.set(x, y, EMPTY);
                        self.set(x2, y2, ROBOT);
                        self.robot = (x2, y2);
                    }
                    BOX => {
                        let dir = (x2 as isize - x as isize, y2 as isize - y as isize);

                        let mut next = (x2 + dir.0 as usize, y2 + dir.1 as usize);
                        while let Some(last) =
                            self.get(next.0, next.1)
                        {
                            match *last {
                                BOX => {
                                    next = (next.0 + dir.0 as usize, next.1 + dir.1 as usize);
                                },
                                EMPTY => {
                                    self.set(x, y, EMPTY);
                                    self.set(x2, y2, ROBOT);
                                    self.set(next.0, next.1, BOX);
                                    self.robot = (x2, y2);
                                    break;
                                }
                                WALL => break,
                                _ => panic!("out of bounds"),
                            }
                        }
                    }
                    WALL => return,
                    _ => panic!("out of bounds"),
                }
            }
            _ => panic!("out of bounds")
        }
    }

    fn sum_gps(&self) -> usize {
        // find all O
        // value is 100* distance to top wall + distance to left wall

        self.grid.iter().enumerate().fold(0, |acc, (y, row)| {
            acc + row.iter().enumerate().fold(
                0,
                |acc, (x, &v)| {
                    if v == BOX {
                        acc + 100 * y + x
                    } else {
                        acc
                    }
                },
            )
        })
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let grid = map
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut grid = Grid::new(grid);
    // grid.print();

    for instruction in instructions.trim().bytes() {
        grid.move_robot(instruction);
        // grid.print();
    }

    let p1 = grid.sum_gps();
    let p2 = 0_usize;

    (Solution::from(p1), Solution::from(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day15/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(10092));
        assert_eq!(p2, Solution::Usize(0));
    }

    #[test]
    fn test_small_input() {
        let input = include_str!("../../input/day15/test_small.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(2028));
        assert_eq!(p2, Solution::Usize(0));
    }
}
