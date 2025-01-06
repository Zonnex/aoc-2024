use hashbrown::HashMap;
use itertools::Itertools;

use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let graph = parse_graph(input);

    let p1 = p1(&graph);
    let p2 = p2(graph);

    (Solution::Usize(p1), Solution::Str(p2))
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();

        graph.entry(a).or_insert_with(Vec::new).push(b);
        graph.entry(b).or_insert_with(Vec::new).push(a);
    }
    graph
}

fn p1(graph: &HashMap<&str, Vec<&str>>) -> usize {
    let mut p1 = 0;
    for (&a, &b, &c) in graph.keys().tuple_combinations() {
        if (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            && graph[&a].contains(&b)
            && graph[&a].contains(&c)
            && graph[&b].contains(&c)
        {
            p1 += 1;
        }
    }
    p1
}

fn p2(graph: HashMap<&str, Vec<&str>>) -> String {
    let mut clique = Vec::new();
    let mut largest_clique = Vec::new();

    for (c1, connections) in graph.iter() {
        clique.clear();
        clique.push(c1);

        for c2 in connections {
            if clique.iter().all(|&c| graph[c2].contains(c)) {
                clique.push(c2);
            }
        }

        if clique.len() > largest_clique.len() {
            largest_clique.clone_from(&clique);
        }
    }
    largest_clique.sort();
    largest_clique.into_iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = include_str!("../../input/day23/test.txt");

        let (p1, p2) = super::solve(input);
        assert_eq!(p1, Solution::Usize(7));
        assert_eq!(p2, Solution::Str("co,de,ka,ta".to_string()));
    }
}
