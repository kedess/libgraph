use libgraph::{dfs, dfs_visitor, path_iter, DfsEvent, Graph, GraphKind};
#[test]
fn test_dfs_1() {
    let mut graph = GraphKind::Directed(Graph::new(10));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();

    let parents = dfs(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 3, 2, 1]);

    let parents = dfs(&graph, 1).unwrap();
    assert_eq!(path_iter(77, &parents).collect::<Vec<usize>>(), vec![]);
}

#[test]
#[should_panic]
fn test_dfs_2() {
    let mut graph = GraphKind::Directed(Graph::new(10));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();

    let _ = dfs(&graph, 12).unwrap();
}

#[test]
fn test_dfs_3() {
    let mut graph = GraphKind::Directed(Graph::new(10));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();
    let mut vertexes = vec![];
    let _ = dfs_visitor(&graph, 1, |event| {
        match event {
            DfsEvent::DiscoverVertex(vertex, _time) => {
                vertexes.push(vertex);
            }
            _ => {}
        }
        false
    })
    .unwrap();
    assert_eq!(vertexes, vec![1, 2, 3, 5]);
}

#[test]
fn test_dfs_4() {
    let mut graph = GraphKind::Directed(Graph::new(10));

    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();

    let mut vertexes = vec![];

    let _ = dfs_visitor(&graph, 1, |event| {
        match event {
            // The event is called when the algorithm first encounters a vertex in a traversal.
            DfsEvent::DiscoverVertex(vertex, _time_in) => {
                vertexes.push(vertex);
            }
            // The event is called before you start exploring a vertex.
            DfsEvent::ExamineVertex(_vertex) => {}
            // The event is triggered when the edge being examined is an edge of the tree after the traversal.
            DfsEvent::IsTreeEdge(_from, _to) => {}
            // The event is triggered when we return to the ancestor from which we explored the vertex.
            DfsEvent::ReturnParent(_from, _) => {}
            // The event is triggered when we try to traverse an inverse edge.
            DfsEvent::BackEdge(_from, _to) => {}
            // The event is triggered when we try to traverse a straight or transversal edge (only for an oriented graph).
            DfsEvent::ForwardOrCrossEdge(_, _to) => {}
            // The event is called after all outgoing edges are examined and all neighboring vertices.
            DfsEvent::FinishVertex(_vertex, _time_out) => {}
        }
        // If true is returned, the traversal will end after this callback is called
        false
    })
    .unwrap();
    assert_eq!(vertexes, vec![1, 2, 3, 5]);
}

