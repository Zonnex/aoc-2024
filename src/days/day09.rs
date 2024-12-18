use std::collections::VecDeque;

use crate::{Solution, SolutionPair};
// use itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Id(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Count(u32);

struct HardDrive {
    files: VecDeque<(Id, Count)>,
    front_index: usize,
    back_index: usize,
}

impl HardDrive {
    fn new(data: &[u32]) -> Self {
        let files = data
            .iter()
            .step_by(2)
            .enumerate()
            .map(|(index, &count)| (Id(index), Count(count)))
            .collect::<VecDeque<_>>();

        let front_index = 0;
        let back_index = files.len() - 1;

        Self {
            files,
            front_index,
            back_index,
        }
    }
    fn pop_block_front(&mut self) -> Option<Id> {
        let (index, Count(count)) = self.files.get_mut(self.front_index).unwrap();
        match count {
            0 => return None,
            _ => {
                *count -= 1;
                if *count == 0 {
                    self.front_index += 1;
                }
                Some(*index)
            }
        }
    }

    fn pop_block_back(&mut self) -> Option<Id> {
        let (index, Count(count)) = self.files.get_mut(self.back_index).unwrap();
        match count {
            0 => return None,
            _ => {
                *count -= 1;
                if *count == 0 {
                    self.back_index -= 1;
                }
                Some(*index)
            }
        }
    }

    fn pop_file_front(&mut self) -> Option<(Id, Count)> {
        return self.files.pop_front();
    }

    fn pop_file_back(&mut self, size: u32) -> Option<(Id, Count)> {
        if let Some(x) =
            self.files
                .iter()
                .enumerate()
                .rev()
                .find_map(|(index, (Id(_), Count(count)))| match *count <= size {
                    true => Some(index),
                    false => None,
                })
        {
            self.files.remove(x)
        } else {
            None
        }
    }

    fn p1(&mut self, input: &[u32]) -> u32 {
        let mut checksum = 0;
        let mut i = 0_u32;
        'outer: for (index, digit) in input.into_iter().enumerate() {
            match index % 2 {
                0 => {
                    if let Some(value) = self.pop_file_front() {
                        let (Id(value), Count(count)) = value;
                        for j in 0..count {
                            checksum += (i + j) * value as u32;
                        }
                    } else {
                        break 'outer;
                    }
                    i += *digit;
                }
                1 => {
                    let mut spaces = *digit;
                    while spaces > 0 {
                        if let Some(value) = self.pop_file_back(spaces) {
                            let (Id(value), Count(count)) = value;
                            spaces -= count;
                            for j in 0..count {
                                checksum += (i + j) * value as u32;
                            }
                            i += spaces;
                        } else {
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        checksum
    }

    fn p2(&mut self, input: &[u32]) -> u32 {
        let mut compacted: Vec<Option<(usize, u32)>> = vec![];
        compacted.resize_with(input.len(), || { None });

        // we need to move each file from back to as far in front as we can.
        // Check:

        /*
        00...111...2...333.44.5555.6666.777.888899
        0099.111...2...333.44.5555.6666.777.8888.. // move 9
        0099.1117772...333.44.5555.6666.....8888.. // move 7
        0099.111777244.333....5555.6666.....8888.. // move 4
        00992111777.44.333....5555.6666.....8888.. // move 2

        */
        'file_loop: for (fi, &fc) in input.iter().enumerate().step_by(2).rev() {
            'space_loop: for (si, &sc) in input.iter().enumerate().skip(1).step_by(2) {
                // if file fits in space, insert it and track remaining spaces. In example, 9 and 2 will fit in first space.
                if si > fi {
                    break 'space_loop;
                }
                if sc <= fc {

                }
            }
            compacted[fi] = Some((fi, fc));
        }

        0
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let digits = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let (p1, p2) = (p1(&digits), p2(&digits));

    (Solution::from(p1), Solution::from(p2))
}

fn p1(input: &[u32]) -> u32 {
    let mut harddisk = HardDrive::new(input);
    harddisk.p1(input)
}

fn p2(input: &[u32]) -> u32 {
    let mut harddisk = HardDrive::new(input);
    harddisk.p2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day09/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(1928));
        assert_eq!(p2, Solution::U32(2858));
    }
}
