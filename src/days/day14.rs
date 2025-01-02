#![allow(dead_code)]
use itertools::Itertools;

use crate::utils::iter::*;
use crate::utils::parse::*;

use std::cmp::Ordering::*;
use crate::{Solution, SolutionPair};

type Robot = [usize; 4];

pub fn solve(input: &str) -> SolutionPair {
    solve_inner::<101, 103>(input)
}

fn solve_inner<const WIDTH: usize, const HEIGHT: usize>(input: &str) -> SolutionPair {
    let robots = input
        .iter_signed::<i32>()
        .chunk::<4>()
        .map(|[x, y, dx, dy]| {
            [x as usize, y as usize, dx.rem_euclid(WIDTH as i32) as usize, dy.rem_euclid(HEIGHT as i32) as usize]
        })
        .collect::<Vec<_>>();

    let p1 = p1::<WIDTH, HEIGHT>(&robots, 100);
    let p2 = match p2::<WIDTH, HEIGHT>(robots) {
        Some(p2) => Solution::Usize(p2),
        None => Solution::None,
    };

    (Solution::from(p1), p2)
}

fn p1<const WIDTH: usize, const HEIGHT: usize>(robots: &[Robot], seconds: usize) -> usize {
    let half_width = WIDTH / 2;
    let half_height = HEIGHT / 2;

    let mut quadrants = [0; 4];

    for [x, y, dx, dy] in robots {
        let x = (x + dx * seconds) % WIDTH;
        let y = (y + dy * seconds) % HEIGHT;

        match (x.cmp(&half_width), y.cmp(&half_height)) {
            (Less, Less) => quadrants[0] += 1,
            (Less, Greater) => quadrants[1] += 1,
            (Greater, Less) => quadrants[2] += 1,
            (Greater, Greater) => quadrants[3] += 1,
            _ => (),
        }
    }

    quadrants.iter().product()
}

fn p2<const WIDTH: usize, const HEIGHT: usize>(mut robots: Vec<Robot>) -> Option<usize> {
    for i in 1.. {
        for [x, y, dx, dy] in robots.iter_mut() {
            *x = (*x + *dx).rem_euclid(WIDTH);
            *y = (*y + *dy).rem_euclid(HEIGHT);
        }
        if robots.iter().map(|&[x, y, _, _]| (x, y)).all_unique() {
            // print_grid::<WIDTH, HEIGHT>(&robots);

            return Some(i);
        }
        // pattern should have been found within WIDTH * HEIGHT iterations
        if i > WIDTH * HEIGHT { 
            return None;
        }
    }

    unreachable!()
}

fn print_grid<const WIDTH: usize, const HEIGHT: usize>(robots: &[Robot]) {
    let mut grid = [['.'; WIDTH]; HEIGHT];
    for [x, y, _, _] in robots.iter() {
        grid[*y][*x] = '#';
    }
    for row in grid.iter() {
        println!("{}", row.iter().join(""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day14/test.txt");

        let (p1, p2) = super::solve_inner::<11, 7>(input);
        assert_eq!(p1, Solution::Usize(12));
        assert_eq!(p2, Solution::Usize(1));
    }
}
