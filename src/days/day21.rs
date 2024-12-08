use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(_input: &str) -> SolutionPair {
    // Your solution here...
    let p1 = 0;
    let p2 = 0;

    (Solution::from(p1), Solution::from(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day21/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(0));
        assert_eq!(p2, Solution::Usize(0));
    }
}
