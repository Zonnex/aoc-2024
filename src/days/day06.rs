use std::collections::{HashMap, HashSet};

use crate::{
    utils::vector_2d::{Vector2, N, ORIGIN},
    Solution, SolutionPair,
};

struct Map {
    data: HashMap<Vector2, char>,
    start: Vector2,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut data = input
            .trim()
            .lines()
            .rev()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| (Vector2::new_usize(x, y), c))
            })
            .collect::<HashMap<_, _>>();

        let start = data
            .iter()
            .find_map(|(k, v)| if *v == '^' { Some(*k) } else { None })
            .unwrap();

        data.entry(start).and_modify(|c| *c = '.');

        Self { data, start }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = Map::parse(input);

    let mut positions:HashMap<Vector2, HashSet<Vector2>> = HashMap::new();
    positions.entry(map.start).or_default().insert(N);

    let obstructions: HashSet<Vector2> = HashSet::new();

    let mut current = map.start;
    let mut dir = N;
    while let Some(&c) = map.data.get(&(current + dir)) {
        match c {
            '#' => dir = ORIGIN.right(dir),
            '.' => {
                current += dir;
                positions.entry(current).or_default().insert(N);
                // check if a previous path is to our right and we traversed right on it?
            }
            _ => unreachable!(),
        }
    }
    let p1 = positions.len();
    let p2 = obstructions.len();

    println!("obstructions: {:?}", obstructions);

    (Solution::from(p1), Solution::from(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::solution::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day06/test.txt");
        let (p1, p2) = super::solve(input);

        assert_eq!(p1, Solution::Usize(41));
        assert_eq!(p2, Solution::Usize(6));
    }
}
