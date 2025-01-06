use crate::{Solution, SolutionPair};

type Key = [u32; 5];
type Lock = [u32; 5];

pub fn solve(input: &str) -> SolutionPair {
    let (keys, locks) = parse_input(input);

    let (mut p1, p2) = (0, Solution::None);
    for key in &keys {
        for lock in &locks {
            let mut valid = true;
            for column in 0..5 {
                if key[column] + lock[column] > 5 {
                    valid = false;
                }
            }
            if valid {
                p1 += 1;
            }
        }
    }

    (Solution::Usize(p1), p2)
}

fn parse_input(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for schematic in input.split("\n\n") {
        match schematic.as_bytes()[0] {
            b'#' => {
                let mut i = 0;
                let mut key = [0; 5];
                for &b in schematic.as_bytes().iter().skip(5) {
                    if b == b'\n' {
                        continue;
                    }
                    let column = i % 5;
                    if b == b'#' {
                        key[column] += 1;
                    }
                    i += 1;
                }
                keys.push(key);
            }
            b'.' => {
                let mut i = 0;
                let mut lock = [5; 5];
                for &b in schematic.as_bytes().iter().skip(5) {
                    if b == b'\n' {
                        continue;
                    }
                    let column = i % 5;
                    if b == b'.' {
                        lock[column] -= 1;
                    }
                    i += 1;
                }
                locks.push(lock);
            }
            _ => panic!("Invalid input"),
        }
    }

    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day25/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(3));
        assert_eq!(p2, Solution::None);
    }
}
