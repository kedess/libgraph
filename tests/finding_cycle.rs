use libgraph::{finding_cycle, Graph, GraphKind};

#[test]
fn test_1() {
    let mut graph = GraphKind::Directed(Graph::new(10));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    assert_eq!(None, finding_cycle(&graph).unwrap());

    graph.add_edge(3, 1, 0.0).unwrap();
    assert_eq!(vec![3, 2, 1], finding_cycle(&graph).unwrap().unwrap());
}

#[test]
fn test_2() {
    let mut graph = GraphKind::Undirected(Graph::new(10));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    assert_eq!(None, finding_cycle(&graph).unwrap());

    graph.add_edge(3, 1, 0.0).unwrap();
    assert_eq!(vec![3, 2, 1], finding_cycle(&graph).unwrap().unwrap());
}

