use crate::{dfs_visitor, topological_sort, DfsEvent, Edge, Graph, GraphError, GraphKind};
pub fn strongly_connected_components<T>(graph: &GraphKind<T>) -> Result<Vec<Vec<usize>>, GraphError> where T: Copy {
    match graph {
        GraphKind::Undirected(_) => {
            unimplemented!()
        }
        GraphKind::Directed(_graph) => {
            let adj = &_graph.adj;
            let transpose_graph = transpose(adj)?;
            let orders = topological_sort(graph)?;
            let mut components = vec![];
            let mut visited = vec![false; adj.len()];
            for vertex in orders.iter() {
                if !visited[*vertex] {
                    let mut component = vec![];
                    dfs_visitor(&transpose_graph, *vertex, |event| {
                        if let DfsEvent::FinishVertex(vertex, _) = event {
                            if !visited[vertex] {
                                component.push(vertex);
                            }
                            visited[vertex] = true;
                        }
                        false
                    })?;
                    components.push(component);
                }
            }
            Ok(components)
        }
    }
}
fn transpose<T>(adj: &[Vec<Edge<T>>]) -> Result<GraphKind<T>, GraphError> where T: Copy {
    let mut graph_transp = GraphKind::Directed(Graph::new(adj.len()));
    for (from, edges) in adj.iter().enumerate() {
        for edge in edges {
            graph_transp.add_edge(edge.to, from, edge.weight)?;
        }
    }
    Ok(graph_transp)
}

