use crate::{dfs_visitor, DfsEvent, GraphError, GraphKind};

use std::cmp::min;

pub fn find_bridges<T>(graph: &GraphKind<T>) -> Result<Vec<(usize, usize)>, GraphError> where T: Copy {
    let adj = match graph {
        GraphKind::Undirected(_graph) => &_graph.adj,
        GraphKind::Directed(_graph) => unimplemented!(),
    };
    let mut bridges = vec![];
    let mut visited = vec![false; adj.len()];
    let mut tin = vec![0usize; adj.len()];
    let mut d = vec![0usize; adj.len()];
    for vertex in 0..adj.len() {
        if !visited[vertex] {
            let mut vertexes = vec![];
            let parents = dfs_visitor(graph, vertex, |event| {
                match event {
                    DfsEvent::DiscoverVertex(vertex, timer) => {
                        tin[vertex] = timer;
                        d[vertex] = timer;
                        visited[vertex] = true;
                    }
                    DfsEvent::BackEdge(from, to) => {
                        d[from] = min(d[from], tin[to]);
                    }
                    DfsEvent::ReturnParent(from, to) => {
                        d[to] = min(d[from], d[to]);
                    }
                    DfsEvent::FinishVertex(vertex, _) => {
                        if d[vertex] == tin[vertex] {
                            vertexes.push(vertex);
                        }
                    }
                    _ => {}
                }
                false
            })?;
            for to in vertexes {
                if let Some(from) = parents[to] {
                    bridges.push((from, to));
                }
            }
        }
    }
    Ok(bridges)
}

