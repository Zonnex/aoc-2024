//! # Crossed Wires
//!
//! Part one is a straightforward simulation of the gates. Part two asks us to fix a broken
//! [ripple carry adder](https://en.wikipedia.org/wiki/Adder_(electronics)).
//!
//! The structure of the adder is:
//!
//! * Half adder for bits `x00` and `y00`. Outputs sum to `z00` and carry to `z01`.
//! * Full adder for bits `x01..x44` and `y01..y44`. Outputs carry to next bit in the chain
//!   "rippling" up to final bit.
//! * `z45` is the carry output from `x44` and `y44`.
//!
//! Implemented in logic gates this looks like:
//!
//! ```none
//!    Half Adder     Full Adder
//!    ┌───┐ ┌───┐    ┌───┐ ┌───┐
//!    |x00| |y00|    |x01| |y01|
//!    └───┘ └───┘    └───┘ └───┘
//!     | | ┌─┘ |      | | ┌─┘ |
//!     | └───┐ |      | └───┐ |
//!     | ┌-┘ | |      | ┌-┘ | |
//!    ┌───┐ ┌───┐    ┌───┐ ┌───┐
//!    |XOR| |AND|    |XOR| |AND|
//!    └───┘ └───┘    └───┘ └───┘
//!      |     |    ┌───┴┐     |
//!      |     └──┬────┐ |     |
//!      |   Carry| | ┌───┐    |
//!      |    out | | |AND|    |
//!      |        | | └───┘    |
//!      |        | |   └────┐ |
//!      |        | └────┐   | |
//!      |        └────┐ |   | |
//!      |            ┌───┐ ┌───┐
//!      |            |XOR| |OR |                                  Carry
//!      |            └───┘ └───┘                                   out
//!      |              |     |                                      |
//!    ┌───┐          ┌───┐   |                                    ┌───┐
//!    |z00|          |z01| Carry    ...repeat for z01 to z44...   |z45|
//!    └───┘          └───┘  out                                   └───┘
//! ```
//!
//! Then we can deduce some rules for the output of each gate type:
//!
//! 1. **XOR** If inputs are `x` and `y` then output must be another XOR gate
//!    (except for inputs `x00` and `y00`) otherwise output must be `z`.
//! 2. **AND** Output must be an OR gate (except for inputs `x00` and `y00`).
//! 3. **OR** Output must be both AND and XOR gate, except for final carry
//!    which must output to `z45`.
//!
//! We only need to find swapped outputs (not fix them) so the result is the labels of gates
//! that breaks the rules in alphabetical order.

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let mut cache = HashMap::new();
    let mut connections = HashMap::new();

    for line in s1.lines() {
        let (gate, v) = line.split_once(": ").unwrap();
        cache.insert(gate, v == "1");
    }

    for line in s2.lines() {
        let (l, gate, r, _, dest) = line.split_whitespace().collect_tuple().unwrap();
        connections.insert(dest, (l, gate, r));
    }

    for key in connections.keys() {
        compute(&connections, &mut cache, key);
    }

    let p1 = compute_result(&cache);
    let p2 = ripple_carry_adder(&connections);

    (Solution::from(p1), Solution::from(p2))
}

fn ripple_carry_adder(gates: &HashMap<&str, (&str, &str, &str)>) -> String {
    let mut output = HashSet::new();
    let mut swapped = HashSet::new();

    // Track the kind of gate that each wire label outputs to.
    for &(left, gate, right) in gates.values() {
        output.insert((left, gate));
        output.insert((right, gate));
    }

    for (&to, &(left, gate, right)) in gates {
        match gate {
            "AND" => {
                // Check that all AND gates point to an OR, except for first AND.
                if left != "x00" && right != "x00" && !output.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "OR" => {
                // Check that only XOR gates point to output, except for last carry which is OR.
                if to.starts_with('z') && to != "z45" {
                    swapped.insert(to);
                }
                // OR can never point to OR.
                if output.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "XOR" => {
                if left.starts_with('x') || right.starts_with('x') {
                    // Check that first level XOR points to second level XOR, except for first XOR.
                    if left != "x00" && right != "x00" && !output.contains(&(to, "XOR")) {
                        swapped.insert(to);
                    }
                } else {
                    // Second level XOR must point to output.
                    if !to.starts_with('z') {
                        swapped.insert(to);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort_unstable();
    result.join(",")
}

fn compute_result(cache: &HashMap<&str, bool>) -> usize {
    let mut v = 0;
    for &n in cache.keys() {
        if !n.starts_with('z') {
            continue;
        }
        let i = n[1..].parse::<usize>().unwrap();
        if cache[n] {
            v |= 1 << i;
        }
    }
    v
}

fn compute<'a>(
    connections: &HashMap<&str, (&'a str, &str, &'a str)>,
    cache: &mut HashMap<&'a str, bool>,
    key: &'a str,
) -> bool {
    if let Some(v) = cache.get(key) {
        return *v;
    }

    let (l, gate, r) = connections[key];
    let l = compute(connections, cache, l);
    let r = compute(connections, cache, r);

    let v = match gate {
        "AND" => l & r,
        "OR" => l | r,
        "XOR" => l ^ r,
        _ => unreachable!(),
    };

    cache.insert(key, v);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day24/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(2024));
        assert_eq!(p2, Solution::Str("bfw,bqk,ffh,frj,fst,hwm,kpj,kwq,mjb,nrd,rvg,tgd,tnw,vdt,wpb,z02,z03,z05,z06,z07,z08,z10,z11".into()));
    }
}
