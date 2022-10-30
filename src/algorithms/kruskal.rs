use crate::{GraphError, GraphKind};
use dsu::Dsu;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
pub struct SpanningTreeEdge<T> {
    pub from: usize,
    pub to: usize,
    pub weight: T,
}
struct KruskalEdge<U> where U: PartialOrd + Copy {
    from: usize,
    to: usize,
    dist: U,
}
impl<U> std::cmp::PartialEq for KruskalEdge<U> where U: PartialOrd + Copy {
    fn eq(&self, other: &KruskalEdge<U>) -> bool {
        self.dist == other.dist
    }
}
impl<U> Eq for KruskalEdge<U> where U: PartialOrd + Copy {}
impl<U> std::cmp::Ord for KruskalEdge<U> where U: PartialOrd + Copy {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.partial_cmp(&self.dist).unwrap()
    }
}
impl<U> std::cmp::PartialOrd for KruskalEdge<U> where U: PartialOrd + Copy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.partial_cmp(&self.dist).unwrap())
    }
}
pub fn kruskal<T>(graph: &GraphKind<T>) -> Result<Vec<SpanningTreeEdge<T>>, GraphError> where T: PartialOrd + Copy {
    let adj = match graph {
        GraphKind::Undirected(_graph) => &_graph.adj,
        GraphKind::Directed(_) => unimplemented!(),
    };
    let mut spanning_tree = vec![];
    let mut heap = BinaryHeap::new();
    let mut dsu = Dsu::new(adj.len());
    for (from, edges) in adj.iter().enumerate() {
        for edge in edges.iter() {
            heap.push(KruskalEdge {
                from,
                to: edge.to,
                dist: edge.weight,
            });
        }
    }
    while let Some(kruskal_edge) = heap.pop() {
        if dsu.lookup(kruskal_edge.from).unwrap() != dsu.lookup(kruskal_edge.to).unwrap() {
            dsu.union(kruskal_edge.from, kruskal_edge.to);
            spanning_tree.push(SpanningTreeEdge {
                from: kruskal_edge.from,
                to: kruskal_edge.to,
                weight: kruskal_edge.dist,
            });
        }
    }
    Ok(spanning_tree)
}

