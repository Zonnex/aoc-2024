use hashbrown::HashSet;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let grid = input
        .lines()
        .rev()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();

    let regions = regions(&grid);
    // Your solution here...
    let p1 = regions.iter().map(price_p1).sum::<usize>();
    let p2 = 0_usize;

    (Solution::from(p1), Solution::from(p2))
}

fn regions(grid: &[&[u8]]) -> Vec<HashSet<(usize, usize)>> {
    let mut regions = Vec::new();
    let mut taken: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !taken.contains(&(x, y)) {
                let region = map_region(grid, (x, y));
                taken.extend(region.iter());
                regions.push(region);
            }
        }
    }

    regions
}

fn map_region(grid: &[&[u8]], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let (x, y) = start;
    let mut region: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = Vec::new();
    queue.push(start);

    let v = grid[y][x];

    while let Some((x, y)) = queue.pop() {
        region.insert((x,y));

        for (x2, y2) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if let Some(v2) = grid.get(y2).and_then(|row| row.get(x2)) {
                if *v2 == v && !region.contains(&(x2, y2)) {
                    queue.push((x2, y2));
                }
            }
        }
    }

    region
}

fn price_p1(region: &HashSet<(usize, usize)>) -> usize {
    let area = region.len();
    let borders = region
        .iter()
        .map(|(x, y)| {
            let borders = [(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)];

            borders
                .iter()
                .filter(|(x, y)| !region.contains(&(*x, *y)))
                .count()
        })
        .sum::<usize>();

    area * borders
}

fn _price_p2(region: &HashSet<(usize, usize)>) -> usize {
    let area = region.len();
    let mut _sides = 0;

    /*
    for the sides, to capture the outer sides we can find lowest x and y, traverse clockwise.
    for inner sides, I have no idea yet
    */


    area * _sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day12/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(1930));
        assert_eq!(p2, Solution::Usize(0));
    }

    #[test]
    fn test_price_p1() {
        /*
        AAAA
        BBCD
        BBCC
        EEEC
        */

        // AAAA
        assert_eq!(price_p1(&HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)])), 40);

        // BB
        // BB
        assert_eq!(price_p1(&HashSet::from([(0, 1), (1, 1), (0, 2), (1, 2)])), 32);

        // C
        // CC
        //  C
        assert_eq!(price_p1(&HashSet::from([(2, 1), (2, 2), (3, 2), (3, 3)])), 40);

        // D
        assert_eq!(price_p1(&HashSet::from([(3, 1)])), 4);

        // EEE
        assert_eq!(price_p1(&HashSet::from([(0, 3), (1, 3), (2, 3)])), 24);
    }

    #[test]
    fn test_price_p2() {
        /*
        AAAA
        BBCD
        BBCC
        EEEC
        */

        // AAAA
        assert_eq!(_price_p2(&HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0)])), 40);

        // BB
        // BB
        assert_eq!(_price_p2(&HashSet::from([(0, 1), (1, 1), (0, 2), (1, 2)])), 32);

        // C
        // CC
        //  C
        assert_eq!(_price_p2(&HashSet::from([(2, 1), (2, 2), (3, 2), (3, 3)])), 40);

        // D
        assert_eq!(_price_p2(&HashSet::from([(3, 1)])), 4);

        // EEE
        assert_eq!(_price_p2(&HashSet::from([(0, 3), (1, 3), (2, 3)])), 24);
    }
}
