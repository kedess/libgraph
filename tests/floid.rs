use libgraph::{floid, Graph, GraphKind};

#[test]
fn test_floid_1() {
    let mut graph = GraphKind::Directed::<i32>(Graph::new(5));
    graph.add_edge(1, 2, 1).unwrap();
    graph.add_edge(1, 3, 6).unwrap();
    graph.add_edge(2, 3, 4).unwrap();
    graph.add_edge(2, 4, 1).unwrap();
    graph.add_edge(4, 3, 1).unwrap();

    let dist = floid(&graph).unwrap();
    assert_eq!(dist.get_dist(1, 2).unwrap(), 1);
    assert_eq!(dist.get_dist(1, 4).unwrap(), 2);
}

#[test]
#[should_panic]
fn test_floid_2() {
    let mut graph = GraphKind::Directed::<i32>(Graph::new(4));
    graph.add_edge(1, 2, 1).unwrap();
    graph.add_edge(2, 3, -5).unwrap();
    graph.add_edge(3, 1, -7).unwrap();

    let _ = floid(&graph).unwrap();
}

