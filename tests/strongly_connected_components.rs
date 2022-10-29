use libgraph::{strongly_connected_components, Empty, Graph, GraphKind};
#[test]
fn test_strongly_connected_components() {
    let mut graph = GraphKind::Directed::<Empty>(Graph::new(10));
    graph.add_edge(1, 4, Empty).unwrap();
    graph.add_edge(4, 7, Empty).unwrap();
    graph.add_edge(7, 1, Empty).unwrap();
    graph.add_edge(9, 7, Empty).unwrap();
    graph.add_edge(6, 9, Empty).unwrap();
    graph.add_edge(9, 3, Empty).unwrap();
    graph.add_edge(3, 6, Empty).unwrap();
    graph.add_edge(8, 6, Empty).unwrap();
    graph.add_edge(2, 8, Empty).unwrap();
    graph.add_edge(8, 5, Empty).unwrap();
    graph.add_edge(5, 2, Empty).unwrap();
    let strongly_connected_components = strongly_connected_components(&graph).unwrap();
    assert_eq!(strongly_connected_components[0], vec![8, 5, 2]);
    assert_eq!(strongly_connected_components[1], vec![9, 3, 6]);
    assert_eq!(strongly_connected_components[2], vec![4, 7, 1]);
    assert_eq!(strongly_connected_components[3], vec![0]);
}
