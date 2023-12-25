use core::panic;
use std::thread;
use graphrs::{algorithms::community::louvain, Edge, Graph, GraphSpecs};

fn part1(input: &str) -> usize {
    let mut graph = Graph::<&str, ()>::new(GraphSpecs::undirected_create_missing());
    for line in input.lines() {
        let mut s = line.split_whitespace();
        let name = s.next().unwrap().strip_suffix(':').unwrap();
        for n in s {
            graph.add_edge(Edge::new(name, n)).unwrap();
        }
    }
    let res = louvain::louvain_partitions(&graph, false, Some(0f64), Some(4f64), None).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].len(), 2);
    res[0].iter().map(|n| n.len()).product::<usize>()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 54);
    }
}
