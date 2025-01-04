#![allow(dead_code)]
use core::panic;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

pub fn solve(input: &str) -> SolutionPair {
    let (registers, instructions) = parse(input);
    let mut output = vec![];

    let _ = run(registers, &instructions, |v| output.push(v));

    let p1 = concat(&output);
    let p2 = p2(&instructions);

    (Solution::from(p1), Solution::from(p2))
}

fn p2(instructions: &[usize]) -> usize {
    let mut a = 0;
    let mut registers = Registers { a: 0, b: 0, c: 0 };
    let mut output = vec![];

    let instruction_count = instructions.len();

    for n in 1..=instructions.len() {
        let target_slice = &instructions[(instruction_count-n)..];
        registers.a = a << 3;
        loop {
            output.clear();
            let _ = run(registers, instructions, |v| output.push(v));
            if output == target_slice {
                a = registers.a;
                break;
            } else {
                registers.a += 1;
            }
        }
    }
    registers.a
}

fn parse(input: &str) -> (Registers, Vec<usize>) {
    let (registers, instructions) = input.split_once("\n\n").unwrap();

    let registers = registers
        .lines()
        .map(|l| {
            let (_, value) = l.split_once(": ").unwrap();
            value.parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    let instructions = instructions
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (
        Registers {
            a: registers[0],
            b: registers[1],
            c: registers[2],
        },
        instructions,
    )
}

fn run<F>(mut registers: Registers, instructions: &[usize], mut output: F) -> Registers
where
    F: FnMut(usize),
{
    fn combo_operand(registers: &Registers, v: usize) -> usize{
        match v {
            0..=3 => v,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            7 => panic!("Combo operand 7 is reserved and will not appear in valid programs."),
            _ => unreachable!(),
        }
    }

    let mut i = 0;
    while i < instructions.len() {
        let opcode = instructions[i];
        let operand_literal = instructions[i + 1];

        match opcode {
            0 => {
                let numerator = registers.a;
                let combo = combo_operand(&registers, operand_literal);
                let denominator = 2_usize.pow(combo as u32);
                registers.a = numerator / denominator;
            }
            1 => {
                registers.b ^= operand_literal;
            }
            2 => {
                let combo = combo_operand(&registers, operand_literal);
                registers.b = combo % 8;
            }
            3 => {
                if registers.a != 0 {
                    i = operand_literal;
                    continue;
                }
            }
            4 => {
                registers.b ^= registers.c;
            }
            5 => {
                let combo = combo_operand(&registers, operand_literal);
                output(combo % 8);
            }
            6 => {
                let numerator = registers.a;
                let combo = combo_operand(&registers, operand_literal);
                let denominator = 2_usize.pow(combo as u32);
                registers.b = numerator / denominator;
            }
            7 => {
                let numerator = registers.a;
                let combo = combo_operand(&registers, operand_literal);
                let denominator = 2_usize.pow(combo as u32);
                registers.c = numerator / denominator;
            }
            _ => unreachable!(),
        }
        i += 2;
    }

    registers
}

fn concat(numbers: &[usize]) -> String {
    numbers.iter().map(|x| x.to_string()).join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day17/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Str("5,7,3,0".into()));
        assert_eq!(p2, Solution::Usize(117440));
    }
}
