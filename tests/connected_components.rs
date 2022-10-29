use libgraph::{connected_components, Empty, Graph, GraphKind};
#[test]
fn test_connected_components() {
    let mut graph = GraphKind::Undirected::<f32>(Graph::new(20));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 4, 0.0).unwrap();
    graph.add_edge(5, 6, 0.0).unwrap();
    graph.add_edge(6, 7, 0.0).unwrap();
    graph.add_edge(8, 9, 0.0).unwrap();
    graph.add_edge(9, 10, 0.0).unwrap();
    graph.add_edge(10, 11, 0.0).unwrap();

    let components = connected_components(&graph).unwrap();
    assert_eq!(components[0], [0]);
    assert_eq!(components[1], [1, 2, 3, 4]);
    assert_eq!(components[2], [5, 6, 7]);
    assert_eq!(components[3], [8, 9, 10, 11]);
    assert_eq!(components.len(), 12);

    let graph = GraphKind::Undirected::<f32>(Graph::new(100));

    let components = connected_components(&graph).unwrap();
    assert_eq!(components.len(), 100);

    let mut graph = GraphKind::Undirected::<Empty>(Graph::new(6));
    graph.add_edge(0, 2, Empty).unwrap();
    graph.add_edge(2, 5, Empty).unwrap();
    graph.add_edge(2, 3, Empty).unwrap();
    graph.add_edge(5, 3, Empty).unwrap();
    graph.add_edge(1, 4, Empty).unwrap();

    let components = connected_components(&graph).unwrap();
    assert_eq!(components[0], [0, 2, 5, 3]);
    assert_eq!(components[1], [1, 4]);
    assert_eq!(components.len(), 2);
}

