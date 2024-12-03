use crate::{Solution, SolutionPair};
use itertools::Itertools;

pub fn solve(input: &str) -> SolutionPair {
    let (mut p1, mut p2) = (0, 0);
    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(str::parse::<i32>)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        if check_report_safe(&values) {
            p1 += 1;
        }

        if brute_force(&values) {
            p2 += 1;
        }
    }

    (Solution::from(p1), Solution::from(p2))
}

fn brute_force(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut test = report.to_vec();
        test.remove(i);
        if check_report_safe(&test) {
            return true;
        }
    }
    false
}

fn check_report_safe(report: &[i32]) -> bool {
    let mut ok = false;

    // check if ascending
    ok |= report.iter().tuple_windows().all(|(a, b)| a < b);

    // check if descending
    ok |= report.iter().tuple_windows().all(|(a, b)| a > b);

    // check if difference is 3 or less
    ok && report.iter().tuple_windows().all(|(a, b)| {
        let diff = a - b;
        diff.abs() <= 3
    })
}

#[cfg(test)]
mod tests {
    use crate::etc::solution::Solution;


    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day02/test.txt");
        let (part1, part2) = super::solve(input);
        assert_eq!(part1, Solution::I32(2));
        assert_eq!(part2, Solution::I32(4));
    }
}
