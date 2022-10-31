mod algorithms;
pub use algorithms::bfs::{bfs, bfs_visitor, BfsEvent};
pub use algorithms::dfs::{dfs, dfs_visitor, DfsEvent};
pub use algorithms::connected_components::connected_components;
pub use algorithms::topological_sort::topological_sort;
pub use algorithms::strongly_connected_components::strongly_connected_components;
pub use algorithms::dijkstra::dijkstra;
pub use algorithms::kruskal::{kruskal, SpanningTreeEdge};
pub use algorithms::depths_vertices::depths_vertices;
pub use algorithms::lca::Lca;
pub use algorithms::floid::floid;
pub use algorithms::finding_cycle::finding_cycle;
pub use algorithms::bridges::find_bridges;
pub use algorithms::bellman_ford::bellman_ford;

pub mod error;
use error::GraphError;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    VERSION
}

pub type Parents = Vec<Option<usize>>;

#[derive(Clone, Copy)]
pub struct Empty;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Color {
    White = 0,
    Grey = 1,
    Black = 2,
}

#[derive(Copy, Clone)]
struct Edge<T> where T: Copy {
    pub to: usize,
    pub weight: T,
}

pub struct Graph<T> where T: Copy {
    adj: Vec<Vec<Edge<T>>>,
}

impl<T> Graph<T> where T: Copy {
    pub fn new(size: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); size],
        }
    }
}

pub enum GraphKind<T> where T: Copy {
    Undirected(Graph<T>),
    Directed(Graph<T>),
}

impl<T> GraphKind<T> where T: Copy {
    pub fn add_edge(&mut self, from: usize, to: usize, weight: T) -> Result<(), GraphError> {
        match self {
            GraphKind::Undirected(graph) => {
                if from >= graph.adj.len() || to >= graph.adj.len() {
                    return Err(GraphError::new(
                        "It is unable to add an edge. Specified vertex numbers exceed the allowed size of the graph",
                    ));
                }
                graph.adj[from].push(Edge { to, weight });
                graph.adj[to].push(Edge { to: from, weight });
                Ok(())
            }
            GraphKind::Directed(graph) => {
                if from >= graph.adj.len() || to >= graph.adj.len() {
                    return Err(GraphError::new(
                        "It is unable to add an edge. Specified vertex numbers exceed the allowed size of the graph",
                    ));
                }
                graph.adj[from].push(Edge { to, weight });
                Ok(())
            }
        }
    }
}

pub fn path_iter(target: usize, parents: &Parents) -> PathIter<'_> {
    PathIter {
        target,
        parents,
        used: false,
    }
}

pub struct PathIter<'a> {
    target: usize,
    parents: &'a Parents,
    used: bool,
}

impl<'a> Iterator for PathIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.target >= self.parents.len() {
            return None;
        }
        self.parents[self.target]?;
        if !self.used {
            self.used = true;
            return Some(self.target);
        }
        #[allow(clippy::never_loop)]
        while let Some(next) = self.parents[self.target] {
            let old = next;
            self.target = next;
            return Some(old);
        }
        None
    }
}
