use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let equations = input
        .lines()
        .map(|l| {
            let (target, values) = l.split_once(": ").unwrap();
            let values = values
                .split(' ')
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (target.parse::<u64>().unwrap(), values)
        })
        .collect::<Vec<_>>();

    let p1 = check(&equations, false);
    let p2 = check(&equations, true);

    (Solution::U64(p1), Solution::U64(p2))
}

fn check(input: &[(u64, Vec<u64>)], allow_concat: bool) -> u64 {
    input
        .iter()
        .filter(|(target, values)| check_line(*target, values[0], &values[1..], allow_concat))
        .map(|(target, _)| *target)
        .sum::<u64>()
}

fn check_line(target: u64, current: u64, rest: &[u64], allow_concat: bool) -> bool {
    if current > target {
        return false;
    }
    return match rest {
        [] if current == target => true,
        [] => false,
        [next, rest @ ..] => {
            (allow_concat && check_line(target, concat(current, *next), rest, allow_concat))
            || check_line(target, current * next, rest, allow_concat)
            || check_line(target, current + next, rest, allow_concat)
        }
    };
    
    fn concat(current: u64, next: u64) -> u64 {
        current * 10u64.pow(next.ilog10() + 1) + next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day07/test.txt");
        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::U64(3749));
        assert_eq!(p2, Solution::U64(11387));
    }

    #[test]
    fn test_individual_p1() {
        let tests = vec![
            (true, (190, vec![10, 19])),
            (true, (3267, vec![81, 40, 27])),
            (false, (83, vec![17, 5])),
            (false, (156, vec![15, 6])),
            (false, (7290, vec![6, 8, 6, 15])),
            (false, (161011, vec![16, 10, 13])),
            (false, (192, vec![17, 8, 14])),
            (false, (21037, vec![9, 7, 18, 13])),
            (true, (292, vec![11, 6, 16, 20])),
        ];

        for (expected, (target, numbers)) in tests {
            assert_eq!(
                expected,
                super::check_line(target, numbers[0], &numbers[1..], false)
            );
        }
    }

    #[test]
    fn test_individual_p2() {
        let tests = vec![
            (true, (190, vec![10, 19])),
            (true, (3267, vec![81, 40, 27])),
            (false, (83, vec![17, 5])),
            (true, (156, vec![15, 6])),
            (true, (7290, vec![6, 8, 6, 15])),
            (false, (161011, vec![16, 10, 13])),
            (true, (192, vec![17, 8, 14])),
            (false, (21037, vec![9, 7, 18, 13])),
            (true, (292, vec![11, 6, 16, 20])),
        ];

        for (expected, (target, numbers)) in tests {
            assert_eq!(
                expected,
                super::check_line(target, numbers[0], &numbers[1..], true)
            );
        }
    }
}
