use itertools::Itertools;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
const P2_MULTIPLIER: isize = 10_000_000_000_000;

pub fn solve(input: &str) -> SolutionPair {
    let xs = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse().unwrap())
        .tuples();

    let (mut p1, mut p2) = (0, 0);
    for (ax, ay, bx, by, tx, ty) in xs {
        p1 += solve_single(ax, ay, bx, by, tx, ty);
        p2 += solve_single(ax, ay, bx, by, tx + P2_MULTIPLIER, ty + P2_MULTIPLIER);
    }

    (Solution::from(p1), Solution::from(p2))
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn solve_single(ax: isize, ay: isize, bx: isize, by: isize, tx: isize, ty: isize) -> isize {
    let b = (ty * ax - tx * ay) / (by * ax - bx * ay);
    let a = (tx - b * bx) / ax;
    
    let x = ax * a + bx * b;
    let y = ay * a + by * b;

    if (x, y) != (tx, ty) {
        return 0;
    }
    a * 3 + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        println!("{}", 8400 / 22);
        let rem = 8400_usize.rem_euclid(22);

        println!("{}", rem)
    }

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day13/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Isize(480));
        assert_eq!(p2, Solution::Isize(875318608908));
    }
}
