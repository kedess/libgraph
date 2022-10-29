use libgraph::{bfs, bfs_visitor, path_iter, BfsEvent, Graph, GraphKind};
#[test]
fn test_bfs_1() {
    let mut graph = GraphKind::Undirected(Graph::new(100));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();

    let parents = bfs(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 2, 1]);
    assert_eq!(path_iter(17, &parents).collect::<Vec<usize>>(), vec![17, 8, 4, 2, 1]);

    graph.add_edge(17, 1, 0.0).unwrap();
    let parents = bfs(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 2, 1]);
    assert_eq!(path_iter(17, &parents).collect::<Vec<usize>>(), vec![17, 1]);

    let parents = bfs(&graph, 1).unwrap();
    assert_eq!(path_iter(19, &parents).collect::<Vec<usize>>(), vec![]);
    assert_eq!(path_iter(22, &parents).collect::<Vec<usize>>(), vec![]);
}
#[test]
fn test_bfs_2() {
    let mut graph = GraphKind::Directed(Graph::new(100));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();
    let parents = bfs(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 2, 1]);
    assert_eq!(path_iter(17, &parents).collect::<Vec<usize>>(), vec![17, 8, 4, 2, 1]);

    graph.add_edge(17, 1, 0.0).unwrap();
    let parents = bfs(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 2, 1]);
    assert_eq!(path_iter(17, &parents).collect::<Vec<usize>>(), vec![17, 8, 4, 2, 1]);

    let parents = bfs(&graph, 1).unwrap();
    assert_eq!(path_iter(19, &parents).collect::<Vec<usize>>(), vec![]);
    assert_eq!(path_iter(22, &parents).collect::<Vec<usize>>(), vec![]);
}

#[test]
fn test_bfs_3() {
    let mut graph = GraphKind::Undirected(Graph::new(100));
    graph.add_edge(0, 1, 0.0).unwrap();
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();
    let parents = bfs(&graph, 0).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 2, 1, 0]);
    assert_eq!(path_iter(19, &parents).collect::<Vec<usize>>(), vec![]);
}

#[test]
#[should_panic]
fn test_bfs_4() {
    let mut graph = GraphKind::Undirected(Graph::new(10));
    graph.add_edge(0, 1, 0.0).unwrap();
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();

    let _ = bfs(&graph, 11).unwrap();
}

#[test]
fn test_bfs_5() {
    let mut graph = GraphKind::Directed(Graph::new(100));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();
    let mut vertexes = vec![];
    let _parents = bfs_visitor(&graph, 1, |event| {
        match event {
            BfsEvent::DiscoverVertex(vertex) => {
                vertexes.push(vertex);
            }
            _ => {}
        }
        false
    })
    .unwrap();
    assert_eq!(vertexes, vec![1, 2, 3, 4, 5, 8, 17]);
}

#[test]
fn test_bfs_6() {
    let mut graph = GraphKind::Directed(Graph::new(100));

    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();

    let mut vertexes = vec![];
    let _parents = bfs_visitor(&graph, 1, |event| {
        match event {
            // The event is called before you start examining a vertex (extracting it from the queue).
            BfsEvent::ExamineVertex(_usize) => {}
            // The event is called when the algorithm first encounters a vertex in a traversal.
            BfsEvent::DiscoverVertex(vertex) => {
                vertexes.push(vertex);
            }
            // The event is triggered when the edge being examined is an edge of the tree after the traversal.
            BfsEvent::IsTreeEdge(_from, _to) => {}
            // The event is triggered when the edge being examined is not an edge of the tree after the traversal.
            BfsEvent::IsNotTreeEdge(_from, _to) => {}
            // The event is called if the target vertex is grayed out at the moment of investigation.
            // Gray indicates that the vertex is currently in the queue.
            BfsEvent::GrayTarget(_usize) => {}
            // The event is called if the target node is colored black at the moment of investigation.
            // The black color means that the vertex is no longer in the queue.
            BfsEvent::BlackTarget(_usize) => {}
            // The event is called after all outgoing edges are examined and all neighboring vertices.
            BfsEvent::FinishVertex(_vertex) => {}
        }
        // If true is returned, the traversal will end after this callback is called
        false
    })
    .unwrap();

    assert_eq!(vertexes, vec![1, 2, 3, 4, 5, 8, 17]);
}

