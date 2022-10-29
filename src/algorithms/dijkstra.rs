use crate::{GraphError, GraphKind, Parents};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Add;
pub type Distances<T> = Vec<Option<T>>;
struct Pair<T> where T: PartialOrd + PartialEq + Copy + Default {
    vertex: usize,
    dist: T,
}
pub fn dijkstra<T>(graph: &GraphKind<T>, from: usize) -> Result<(Parents, Distances<T>), GraphError> 
where T: Default + Copy + PartialOrd + PartialEq + Add<Output = T> {
    let adj = match graph {
        GraphKind::Undirected(_graph) => &_graph.adj,
        GraphKind::Directed(_graph) => &_graph.adj,
    };
    let mut parents = vec![None; adj.len()];
    let mut visited = vec![false; adj.len()];
    let mut distances = vec![None; adj.len()];
    let mut heap = BinaryHeap::<Pair<T>>::new();
    distances[from] = Some(T::default());
    heap.push(Pair {
        vertex: from,
        dist: T::default(),
    });
    while !heap.is_empty() {
        let pair = heap.pop().unwrap();
        visited[pair.vertex] = true;
        for edge in &adj[pair.vertex] {
            if !visited[edge.to]
                && (distances[edge.to].is_none() || edge.weight + pair.dist < distances[edge.to].unwrap())
            {
                parents[edge.to] = Some(pair.vertex);
                distances[edge.to] = Some(edge.weight + pair.dist);
                heap.push(Pair {
                    vertex: edge.to,
                    dist: distances[edge.to].unwrap(),
                });
            }
        }
    }
    Ok((parents, distances))
}
impl<T> PartialEq for Pair<T> where T: PartialOrd + PartialEq + Copy + Default {
    fn eq(&self, other: &Pair<T>) -> bool {
        self.dist == other.dist
    }
}
impl<T> Eq for Pair<T> where T: PartialOrd + PartialEq + Copy + Default {}
impl<T> Ord for Pair<T> where T: PartialOrd + PartialEq + Copy + Default {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.partial_cmp(&self.dist).unwrap()
    }
}
impl<T> PartialOrd for Pair<T> where T: PartialOrd + PartialEq + Copy + Default {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.partial_cmp(&self.dist).unwrap())
    }
}

