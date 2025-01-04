use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let values = input
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let (mut p1, p2) = (0_usize, 0_usize);
    for value in values {
        let mut value = value;
        for _ in 0..2000 {
            value = prune(mix(value, value * 64));
            value = prune(mix(value, value / 32));
            value = prune(mix(value, value * 2048));
        }
        p1 += value;
    }

    (Solution::from(p1), Solution::from(p2))
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(a: usize) -> usize {
    a % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day22/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(37327623));
        assert_eq!(p2, Solution::Usize(23));
    }
}
