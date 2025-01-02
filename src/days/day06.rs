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
    let mut map = Map::parse(input);

    let p1 = sim(&map).unwrap();

    let mut p2: usize = 0;
    for &p in &p1 {
        map.data.entry(p).and_modify(|c| *c = '#');

        if sim(&map).is_none() {
            p2 += 1;
        }

        map.data.entry(p).and_modify(|c| *c = '.');
    }

    (Solution::from(p1.len()), Solution::from(p2))
}

// here we let guard go, see if we get out or not
fn sim(map: &Map) -> Option<Vec<Vector2>> {
    let mut positions: HashMap<Vector2, HashSet<Vector2>> = HashMap::new();
    positions.entry(map.start).or_default().insert(N);

    let mut current = map.start;
    let mut dir = N;
    while let Some(&c) = map.data.get(&(current + dir)) {
        match c {
            '#' => dir = ORIGIN.right(dir),
            '.' => {
                current += dir;
                if !positions.entry(current).or_default().insert(dir) {
                    return None;
                }
            }
            _ => unreachable!(),
        }
    }

    Some(positions.keys().copied().collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day06/test.txt");
        let (p1, p2) = super::solve(input);

        assert_eq!(p1, Solution::Usize(41));
        assert_eq!(p2, Solution::Usize(6));
    }
}
