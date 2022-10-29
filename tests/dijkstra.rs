use libgraph::{dijkstra, path_iter, Graph, GraphKind};
#[test]
fn test_deikstra_1() {
    let mut graph = GraphKind::Undirected::<f32>(Graph::new(10));
    graph.add_edge(1, 2, 2.0).unwrap();
    graph.add_edge(2, 3, 5.0).unwrap();
    graph.add_edge(3, 5, 7.0).unwrap();
    graph.add_edge(1, 5, 19.0).unwrap();
    let (parents, distances) = dijkstra(&graph, 1).unwrap();
    assert_eq!(distances[5].unwrap(), 14.0);
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 3, 2, 1]);
}
#[test]
fn test_deikstra_2() {
    let mut graph = GraphKind::Undirected::<f32>(Graph::new(10));
    graph.add_edge(1, 2, 7.0).unwrap();
    graph.add_edge(1, 6, 14.0).unwrap();
    graph.add_edge(1, 3, 9.0).unwrap();
    graph.add_edge(6, 3, 2.0).unwrap();
    graph.add_edge(6, 5, 9.0).unwrap();
    graph.add_edge(3, 2, 10.0).unwrap();
    graph.add_edge(3, 4, 11.0).unwrap();
    graph.add_edge(2, 4, 15.0).unwrap();
    graph.add_edge(4, 5, 6.0).unwrap();
    let (parents, distances) = dijkstra(&graph, 1).unwrap();
    assert_eq!(distances[4].unwrap(), 20.0);
    assert_eq!(path_iter(4, &parents).collect::<Vec<usize>>(), vec![4, 3, 1]);
}

