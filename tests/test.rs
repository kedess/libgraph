use libgraph::{version, Graph, GraphKind};

#[test]
fn test_graph_1() {
    let mut graph = GraphKind::Undirected::<()>(Graph::new(10));
    assert_eq!(graph.add_edge(1, 2, ()), Ok(()));
}

#[test]
fn test_graph_2() {
    let mut graph = GraphKind::Directed::<()>(Graph::new(10));
    assert_eq!(graph.add_edge(1, 2, ()), Ok(()));
}

#[test]
#[should_panic]
fn test_graph_3() {
    let mut graph = GraphKind::Undirected::<()>(Graph::new(10));
    graph.add_edge(1, 22, ()).unwrap();
}

#[test]
#[should_panic]
fn test_graph_4() {
    let mut graph = GraphKind::Directed::<()>(Graph::new(10));
    graph.add_edge(11, 2, ()).unwrap();
}

#[test]
fn test_graph_5() {
    let mut graph = GraphKind::Undirected::<f64>(Graph::new(10));
    assert_eq!(graph.add_edge(1, 2, 10.0), Ok(()));
}

#[test]
fn test_graph_6() {
    let _ = version();
}

#[test]
fn test_graph_7() {
    let mut graph = GraphKind::Undirected::<f64>(Graph::new(10));

    graph.add_edge(1, 2, 2.0).unwrap();
    graph.add_edge(2, 3, 2.0).unwrap();
    graph.add_edge(3, 7, 3.0).unwrap();
}
