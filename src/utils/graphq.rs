//! # Graph utilities
//!
//! If you want to laugh loudly: <https://github.com/petgraph/petgraph/issues/199>
//!
//! > Can you describe what the equivalence criteria are for two graphs?
//!
//! lol.

pub fn graph_eq<N, E, Ty, Ix>(
    a: &petgraph::Graph<N, E, Ty, Ix>,
    b: &petgraph::Graph<N, E, Ty, Ix>,
) -> bool
where
    N: PartialEq,
    E: PartialEq,
    Ty: petgraph::EdgeType,
    Ix: petgraph::graph::IndexType + PartialEq,
{
    let a_ns = a.raw_nodes().iter().map(|n| &n.weight);
    let b_ns = b.raw_nodes().iter().map(|n| &n.weight);
    let a_es = a
        .raw_edges()
        .iter()
        .map(|e| (e.source(), e.target(), &e.weight));
    let b_es = b
        .raw_edges()
        .iter()
        .map(|e| (e.source(), e.target(), &e.weight));
    a_ns.eq(b_ns) && a_es.eq(b_es)
}

#[cfg(test)]
mod test {
    use super::*;

    use petgraph::graph::UnGraph;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_two_graphs_are_equal() {
        let a = UnGraph::<u32, u32>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);
        let b = UnGraph::<u32, u32>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);
        assert_eq!(graph_eq(&a, &b), true);
        let c = UnGraph::<u32, u32>::from_edges(&[(1, 2), (2, 3), (1, 4)]);
        assert_eq!(graph_eq(&a, &c), false);
    }
}
