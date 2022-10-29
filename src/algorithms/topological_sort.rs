use crate::{dfs_visitor, DfsEvent, GraphError, GraphKind};
pub fn topological_sort<T>(graph: &GraphKind<T>) -> Result<Vec<usize>, GraphError> where T: Copy {
    match graph {
        GraphKind::Undirected(_) => unimplemented!(),
        GraphKind::Directed(g) => {
            let mut vertexes = vec![];
            let mut visited = vec![false; g.adj.len()];
            for vertex in 0..g.adj.len() {
                if !visited[vertex] {
                    dfs_visitor(graph, vertex, |event| {
                        if let DfsEvent::FinishVertex(vertex, _) = event {
                            if !visited[vertex] {
                                vertexes.push(vertex);
                                visited[vertex] = true;
                            }
                        }
                        false
                    })?;
                }
            }
            vertexes.reverse();
            Ok(vertexes)
        }
    }
}

