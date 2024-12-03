use crate::{Solution, SolutionPair};

struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
            .lines()
            .map(|line| {
                let (left, right) = line
                    .split_once("   ")
                    .expect("Lines separated by triple spaces");
                (
                    left.parse::<u32>().expect("Strings to be numbers"),
                    right.parse::<u32>().expect("Strings to be numbers"),
                )
            })
            .unzip();

        left.sort();
        right.sort();

        Ok(Self { left, right })
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let input = Input::try_from(input).expect("Valid input");
    (Solution::from(p1(&input)), Solution::from(p2(&input)))
}

fn p1(input: &Input) -> u32 {
    input.left
        .iter()
        .zip(&input.right)
        .map(|(left, &right)| left.abs_diff(right))
        .sum::<u32>()
}

fn p2(input: &Input) -> u64 {
    let mut sum = 0;
    for &num in &input.left {
        let multiplier = input.right.iter().filter(|&&x| x == num).count();
        let product = num as u64 * multiplier as u64;
        sum += product;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::etc::solution::Solution;


    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day01/test.txt");
        let (p1, p2) = super::solve(input);

        assert_eq!(p1, Solution::U32(11));
        assert_eq!(p2, Solution::U64(31));
    }
}
