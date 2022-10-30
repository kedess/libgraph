use crate::{dfs_visitor, DfsEvent, GraphError, GraphKind};
pub fn depths_vertices<T>(graph: &GraphKind<T>, root: usize) -> Result<Vec<Option<usize>>, GraphError> where T: Copy {
    match graph {
        GraphKind::Undirected(_graph) => {
            let adj = &_graph.adj;
            let mut depths = vec![None; adj.len()];
            if root >= adj.len() {
                return Err(GraphError::new("The vertex number exceeds the size of the graph"));
            }
            depths[root] = Some(0);
            let _ = dfs_visitor(graph, root, |event| {
                if let DfsEvent::IsTreeEdge(from, to) = event {
                    depths[to] = Some(depths[from].unwrap() + 1);
                }
                false
            })?;
            Ok(depths)
        }
        GraphKind::Directed(_) => {
            unimplemented!()
        }
    }
}

