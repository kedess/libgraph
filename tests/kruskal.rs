use libgraph::{kruskal, Graph, GraphKind, SpanningTreeEdge};
#[test]
fn test_kruskal() {
    let mut graph = GraphKind::Undirected(Graph::new(20));
    graph.add_edge(0, 1, 3.0).unwrap();
    graph.add_edge(1, 2, 7.0).unwrap();
    graph.add_edge(1, 4, 5.0).unwrap();
    graph.add_edge(2, 3, 8.0).unwrap();
    graph.add_edge(2, 5, 7.0).unwrap();
    graph.add_edge(2, 4, 9.0).unwrap();
    graph.add_edge(3, 5, 5.0).unwrap();
    graph.add_edge(5, 7, 9.0).unwrap();
    graph.add_edge(5, 6, 8.0).unwrap();
    graph.add_edge(5, 4, 15.0).unwrap();
    graph.add_edge(6, 7, 11.0).unwrap();
    graph.add_edge(6, 4, 6.0).unwrap();
    let edges: Vec<SpanningTreeEdge<f64>> = kruskal(&graph).unwrap();
    let summary_weight: f64 = edges.iter().map(|edge| edge.weight).sum();
    assert_eq!(42.0, summary_weight);
    let res = edges
        .iter()
        .map(|edge| (edge.from, edge.to))
        .collect::<Vec<(usize, usize)>>();
    assert_eq!(res, vec![(0, 1), (3, 5), (4, 1), (4, 6), (1, 2), (5, 2), (7, 5)]);
}

