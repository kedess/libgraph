use std::cmp::min;
use std::collections::HashMap;
use std::ops::Add;

use crate::{GraphError, GraphKind};

pub struct Distance<T> {
    map: Vec<HashMap<usize, T>>,
}

impl<T> Distance<T> where T: Copy {
    pub fn get_dist(&self, from: usize, to: usize) -> Option<T> {
        self.map[from].get(&to).copied()
    }
}

pub fn floid<T>(graph: &GraphKind<T>) -> Result<Distance<T>, GraphError>
where T: Default + Copy + Ord + PartialEq + Add<Output = T> + std::fmt::Debug {
    let adj = match graph {
        GraphKind::Undirected(_) => unimplemented!(),
        GraphKind::Directed(_graph) => &_graph.adj,
    };
    let mut distance = Distance {
        map: vec![HashMap::new(); adj.len()],
    };
    for (idx, edges) in adj.iter().enumerate() {
        for edge in edges {
            distance.map[idx].insert(edge.to, edge.weight);
        }
    }
    for i in 0..adj.len() {
        for j in 0..adj.len() {
            for k in 0..adj.len() {
                if let (Some(&first), Some(&second)) = (distance.map[j].get(&i), distance.map[i].get(&k)) {
                    let weight = distance.map[j].entry(k).or_insert(first + second);
                    *weight = min(*weight, first + second);
                    if j == k && *weight < T::default() {
                        return Err(GraphError::new("A negative cycle is detected in the graph"));
                    }
                };
            }
        }
    }
    Ok(distance)
}
