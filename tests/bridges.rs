use libgraph::{find_bridges, Graph, GraphKind};
#[test]
fn test_bridges_1() {
    let mut graph = GraphKind::Undirected(Graph::new(8));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(1, 3, 0.0).unwrap();
    graph.add_edge(3, 4, 0.0).unwrap();
    graph.add_edge(4, 5, 0.0).unwrap();
    graph.add_edge(4, 6, 0.0).unwrap();
    graph.add_edge(5, 6, 0.0).unwrap();
    graph.add_edge(5, 7, 0.0).unwrap();
    let bridges = find_bridges(&graph).unwrap();
    assert_eq!(bridges, vec![(5, 7), (3, 4)]);
}

#[test]
fn test_bridges_2() {
    let mut graph = GraphKind::Undirected(Graph::new(8));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(1, 3, 0.0).unwrap();
    let bridges = find_bridges(&graph).unwrap();
    assert_eq!(bridges, vec![]);
}

#[test]
fn test_bridges_3() {
    let mut graph = GraphKind::Undirected(Graph::new(8));
    graph.add_edge(0, 1, 0.0).unwrap();
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(1, 3, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 4, 0.0).unwrap();
    graph.add_edge(7, 6, 0.0).unwrap();
    let bridges = find_bridges(&graph).unwrap();
    assert_eq!(bridges, vec![(3, 4), (0, 1), (6, 7)]);
}

