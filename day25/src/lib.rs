use std::convert::Infallible;

use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graphmap::UnGraphMap};

pub fn part1(input: &str) -> String {
    let edges = input.trim().lines().flat_map(|line| {
        line.split_once(": ")
            .map(|(k, v)| v.split(' ').map(move |v| (k, v)))
            .unwrap()
    });

    let graph = UnGraphMap::<_, ()>::from_edges(edges);

    // It Christmas! #include <solution>
    let l = stoer_wagner_min_cut(&graph, |_| Result::<usize, Infallible>::Ok(1))
        .unwrap()
        .unwrap()
        .1
        .len();

    (l * (graph.node_count() - l)).to_string()
}

pub fn part2(_: &str) -> String {
    // Nothing to solve!
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
jqt: rhn xhk nvd
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
frs: qnr lhk lsr
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "54");
    }
}
