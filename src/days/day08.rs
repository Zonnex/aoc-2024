use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{utils::vector_2d::Vector2, Solution, SolutionPair};

struct Antennas {
    antennas: HashMap<char, Vec<Vector2>>,
    boundary: Vector2,
}

impl Antennas {
    fn parse(input: &str) -> Self {
        let mut antennas = Antennas {
            antennas: HashMap::new(),
            boundary: Vector2::default(),
        };

        for (y, line) in input.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.add(c, Vector2::new_usize(x, y));
                }
                antennas.boundary = antennas.boundary.max(Vector2::new_usize(x, y));
            }
        }

        antennas
    }

    fn add(&mut self, c: char, v: Vector2) {
        self.antennas.entry(c).or_default().push(v);
    }

    fn is_inside(&self, antinode: Vector2) -> bool {
        0 <= antinode.x
            && 0 <= antinode.y
            && antinode.x <= self.boundary.x
            && antinode.y <= self.boundary.y
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let antennas = Antennas::parse(input);

    let p1 = p1(&antennas);
    let p2 = p2(&antennas);

    (Solution::from(p1), Solution::from(p2))
}

fn p1(input: &Antennas) -> usize {
    let mut antinodes = HashSet::new();
    for (&_c, antennas) in &input.antennas {
        for (&a, &b) in antennas.iter().tuple_combinations() {
            let x_diff = b.x - a.x;
            let y_diff = b.y - a.y;
            let distance = Vector2::new(x_diff, y_diff);

            let candidates = [a - distance, b + distance];
            for &antinode in &candidates {
                if input.is_inside(antinode) {
                    // println!("antinode: {:?}", antinode);
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn p2(input: &Antennas) -> usize {
    let mut antinodes = HashSet::new();
    for (&_c, antennas) in &input.antennas {
        for (&a, &b) in antennas.iter().tuple_combinations() {
            let x_diff = b.x - a.x;
            let y_diff = b.y - a.y;
            let distance = Vector2::new(x_diff, y_diff);

            let mut i = 0;
            loop {
                let candidate = a + (distance * i);
                if !input.is_inside(candidate) {
                    break;
                }
                antinodes.insert(candidate);
                i += 1;
            }

            let mut i = 0;
            loop {
                let candidate = b - (distance * i);
                if !input.is_inside(candidate) {
                    break;
                }
                antinodes.insert(candidate);
                i += 1;
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day08/test.txt");
        let (p1, p2) = solve(input);

        assert_eq!(p1, Solution::Usize(14));
        assert_eq!(p2, Solution::Usize(34));
    }
}
