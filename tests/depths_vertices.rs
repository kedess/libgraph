use libgraph::{depths_vertices, Graph, GraphKind};
#[test]
fn depths_vertices_test_1() {
    let mut graph = GraphKind::Undirected(Graph::new(13));
    graph.add_edge(1, 4, 0).unwrap();
    graph.add_edge(1, 2, 0).unwrap();
    graph.add_edge(4, 11, 0).unwrap();
    graph.add_edge(4, 12, 0).unwrap();
    graph.add_edge(12, 3, 0).unwrap();
    graph.add_edge(2, 5, 0).unwrap();
    graph.add_edge(2, 6, 0).unwrap();
    graph.add_edge(5, 9, 0).unwrap();
    graph.add_edge(5, 10, 0).unwrap();
    graph.add_edge(6, 7, 0).unwrap();
    graph.add_edge(7, 8, 0).unwrap();

    let depths = depths_vertices(&graph, 1).unwrap();
    assert_eq!(depths[3], Some(3));
    assert_eq!(depths[8], Some(4));
    assert_eq!(depths[11], Some(2));
}

#[test]
#[should_panic]
fn depths_vertices_test_2() {
    let graph = GraphKind::Undirected::<f32>(Graph::new(13));
    let _ = depths_vertices(&graph, 15).unwrap();
}

