use libgraph::{bellman_ford, path_iter, Graph, GraphKind};

#[test]
fn test_bellman_ford_1() {
    let mut graph = GraphKind::Directed(Graph::new(8));

    graph.add_edge(1, 2, 2.0).unwrap();
    graph.add_edge(2, 3, 5.0).unwrap();
    graph.add_edge(3, 5, 7.0).unwrap();
    graph.add_edge(1, 5, 19.0).unwrap();

    let (parents, distances) = bellman_ford(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 3, 2, 1]);
    assert_eq!(distances[5].unwrap(), 14.0);
}

#[test]
#[should_panic]
fn test_bellman_ford_2() {
    let mut graph = GraphKind::Directed(Graph::new(8));

    graph.add_edge(1, 2, 2.0).unwrap();
    graph.add_edge(2, 3, -2.0).unwrap();
    graph.add_edge(3, 4, -2.0).unwrap();
    graph.add_edge(4, 2, -2.0).unwrap();

    bellman_ford(&graph, 1).unwrap();
}

