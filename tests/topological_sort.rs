use libgraph::{topological_sort, Empty, Graph, GraphKind};
#[test]
fn test_topology_sort() {
    let mut graph = GraphKind::Directed::<f32>(Graph::new(10));
    graph.add_edge(0, 1, 0.0).unwrap();
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(1, 3, 0.0).unwrap();
    graph.add_edge(1, 5, 0.0).unwrap();
    graph.add_edge(1, 4, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(3, 4, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();
    assert_eq!(topological_sort(&graph).unwrap(), vec![9, 8, 7, 6, 0, 1, 3, 5, 2, 4]);

    let mut graph = GraphKind::Directed::<f32>(Graph::new(3));
    graph.add_edge(0, 1, 0.0).unwrap();
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 0, 0.0).unwrap();

    let mut graph = GraphKind::Directed::<Empty>(Graph::new(6));
    graph.add_edge(1, 2, Empty).unwrap();
    graph.add_edge(2, 4, Empty).unwrap();
    graph.add_edge(1, 3, Empty).unwrap();
    graph.add_edge(2, 3, Empty).unwrap();
    graph.add_edge(3, 4, Empty).unwrap();
    graph.add_edge(3, 5, Empty).unwrap();
    assert_eq!(topological_sort(&graph).unwrap(), vec![1, 2, 3, 5, 4, 0]);
}

