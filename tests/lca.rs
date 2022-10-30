use libgraph::{Graph, GraphKind, Lca};
#[test]
fn test_lca_1() {
    let mut graph = GraphKind::Undirected(Graph::new(9));
    graph.add_edge(0, 1, 0).unwrap();
    graph.add_edge(1, 2, 0).unwrap();
    graph.add_edge(1, 3, 0).unwrap();
    graph.add_edge(2, 4, 0).unwrap();
    graph.add_edge(2, 5, 0).unwrap();
    graph.add_edge(3, 6, 0).unwrap();
    graph.add_edge(3, 7, 0).unwrap();
    graph.add_edge(7, 8, 0).unwrap();
    let lca = Lca::build(&graph, 0).unwrap();
    assert_eq!(2, lca.query(4, 5).unwrap());
    assert_eq!(1, lca.query(4, 8).unwrap());
    assert_eq!(2, lca.query(4, 2).unwrap());
    assert_eq!(1, lca.query(2, 8).unwrap());
    assert_eq!(3, lca.query(6, 8).unwrap());
}
#[test]
fn test_lca_2() {
    let mut graph = GraphKind::Undirected(Graph::new(12));
    graph.add_edge(0, 1, 0).unwrap();
    graph.add_edge(1, 2, 0).unwrap();
    graph.add_edge(1, 3, 0).unwrap();
    graph.add_edge(2, 4, 0).unwrap();
    graph.add_edge(2, 5, 0).unwrap();
    graph.add_edge(3, 6, 0).unwrap();
    graph.add_edge(3, 7, 0).unwrap();
    graph.add_edge(7, 8, 0).unwrap();
    graph.add_edge(3, 9, 0).unwrap();
    graph.add_edge(9, 10, 0).unwrap();
    graph.add_edge(9, 11, 0).unwrap();
    let lca = Lca::build(&graph, 0).unwrap();
    assert_eq!(3, lca.query(11, 7).unwrap());
}

