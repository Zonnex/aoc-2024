use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let p1 = compute(input);

    // match regex, replace with nothing, parse, sum
    let replacer = regex::Regex::new(r"don't\(\)[\s\S]*?do\(\)").unwrap();

    let input = replacer.replace_all(input, "").to_string();

    let p2 = match input.contains("don't()") {
        true => {
            let (input, _) = input.split_once("don't()").unwrap();
            compute(input)
        },
        false => compute(&input),
    };

    (Solution::from(p1), Solution::from(p2))
}

fn compute(input: &str) -> u64 {
    let matches = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();

    // iter matches, split on comma, parse, multiply, sum
    matches
        .find_iter(input)
        .map(|m| {
            let mut nums = m.as_str().split(['(', ',', ')']);
            let a = nums.nth(1).unwrap().parse::<u64>().unwrap();
            let b = nums.next().unwrap().parse::<u64>().unwrap();
            a * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::etc::solution::Solution;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day03/test.txt");
        let (part1, part2) = super::solve(input);
        assert_eq!(part1, Solution::U64(161));
        assert_eq!(part2, Solution::U64(48));
    }
}
