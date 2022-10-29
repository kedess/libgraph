use crate::{bfs_visitor, BfsEvent, GraphError, GraphKind};
pub fn connected_components<T>(graph: &GraphKind<T>) -> Result<Vec<Vec<usize>>, GraphError> where T: Copy {
    match graph {
        GraphKind::Undirected(_graph) => {
            let adj = &_graph.adj;
            let mut components = vec![];
            let mut visited = vec![false; adj.len()];
            for vertex in 0..adj.len() {
                if !visited[vertex] {
                    let mut vec = vec![];
                    bfs_visitor(graph, vertex, |event| {
                        if let BfsEvent::DiscoverVertex(vertex) = event {
                            vec.push(vertex);
                            visited[vertex] = true;
                        }
                        false
                    })?;
                    components.push(vec);
                }
            }
            Ok(components)
        }
        GraphKind::Directed(_) => {
            unimplemented!()
        }
    }
}

