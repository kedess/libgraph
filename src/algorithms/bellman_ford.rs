use crate::{GraphError, GraphKind, Parents};
use std::ops::Add;

pub type Distances<T> = Vec<Option<T>>;

pub fn bellman_ford<T>(graph: &GraphKind<T>, from: usize) -> Result<(Parents, Distances<T>), GraphError> where T: Default + Copy + PartialOrd + PartialEq + Add<Output = T> {
    let adj = match graph {
        GraphKind::Undirected(_graph) => unimplemented!(),
        GraphKind::Directed(_graph) => &_graph.adj,
    };

    let mut parents = vec![None; adj.len()];
    let mut distances = vec![None; adj.len()];
    distances[from] = Some(Default::default());
    for _ in 0..adj.len() {
        let mut any = false;
        for idx in 0..adj.len() {
            if distances[idx].is_some() {
                for edge in &adj[idx] {
                    if distances[edge.to].is_none() {
                        parents[edge.to] = Some(idx);
                        distances[edge.to] = Some(edge.weight + distances[idx].unwrap());
                        any = true;
                    } else if edge.weight + distances[idx].unwrap() < distances[edge.to].unwrap() {
                        parents[edge.to] = Some(idx);
                        distances[edge.to] = Some(edge.weight + distances[idx].unwrap());
                        any = true
                    }
                }
            }
        }
        if !any {
            return Ok((parents, distances));
        }
    }
    Err(GraphError::new("There is a negative weight cycle"))
}

