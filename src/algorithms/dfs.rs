use crate::{Color, Edge, GraphError, GraphKind, Parents};

pub fn dfs<T>(graph: &GraphKind<T>, from: usize) -> Result<Parents, GraphError> where T: Copy {
    let adj = match graph {
        GraphKind::Undirected(graph) => &graph.adj,
        GraphKind::Directed(graph) => &graph.adj,
    };
    if from >= adj.len() {
        return Err(GraphError::new(
            "Unable to start a traversal. Specified vertex number exceeds allowed graph size",
        ));
    }
    let mut parents = vec![None; adj.len()];
    let mut colors = vec![Color::White; adj.len()];
    _dfs(adj, from, &mut parents, &mut colors);
    Ok(parents)
}
fn _dfs<T>(adj: &[Vec<Edge<T>>], from: usize, parents: &mut [Option<usize>], colors: &mut [Color]) where T: Copy {
    colors[from] = Color::Grey;
    for edge in adj[from].iter() {
        if colors[edge.to] == Color::White {
            parents[edge.to] = Some(from);
            _dfs(adj, edge.to, parents, colors);
        }
    }
    colors[from] = Color::Black;
}

pub enum DfsEvent {
    ExamineVertex(usize),
    DiscoverVertex(usize, usize),
    IsTreeEdge(usize, usize),
    ReturnParent(usize, usize),
    BackEdge(usize, usize),
    ForwardOrCrossEdge(usize, usize),
    FinishVertex(usize, usize),
}

pub fn dfs_visitor<T, F: FnMut(DfsEvent) -> bool>(
    graph: &GraphKind<T>,
    from: usize,
    mut visitor: F,
) -> Result<Parents, GraphError> where T: Copy {
    let mut timer = 0;
    match graph {
        GraphKind::Undirected(graph) => {
            let adj = &graph.adj;
            let mut parents = vec![None; adj.len()];
            let mut colors = vec![Color::White; adj.len()];
            if from >= adj.len() {
                return Err(GraphError::new(
                    "Unable to start a traversal. Specified vertex number exceeds allowed graph size",
                ));
            }
            _dfs_visitor(adj, from, &mut visitor, &mut parents, &mut colors, &mut timer, false);
            Ok(parents)
        }
        GraphKind::Directed(graph) => {
            let adj = &graph.adj;
            let mut parents = vec![None; adj.len()];
            let mut colors = vec![Color::White; adj.len()];
            if from >= adj.len() {
                return Err(GraphError::new(
                    "Unable to start a traversal. Specified vertex number exceeds allowed graph size",
                ));
            }
            _dfs_visitor(adj, from, &mut visitor, &mut parents, &mut colors, &mut timer, true);
            Ok(parents)
        }
    }
}
fn _dfs_visitor<T, F: FnMut(DfsEvent) -> bool>(
    adj: &[Vec<Edge<T>>],
    from: usize,
    visitor: &mut F,
    parents: &mut [Option<usize>],
    colors: &mut [Color],
    timer: &mut usize,
    is_direct: bool,
) where T: Copy {
    colors[from] = Color::Grey;
    *timer += 1;
    if visitor(DfsEvent::DiscoverVertex(from, *timer)) {
        return;
    }
    for edge in adj[from].iter() {
        if visitor(DfsEvent::ExamineVertex(from)) {
            return;
        }
        if colors[edge.to] == Color::White {
            parents[edge.to] = Some(from);
            if visitor(DfsEvent::IsTreeEdge(from, edge.to)) {
                return;
            }
            _dfs_visitor(adj, edge.to, visitor, parents, colors, timer, is_direct);
            if visitor(DfsEvent::ReturnParent(edge.to, from)) {
                return;
            }
        }
        if colors[edge.to] == Color::Grey && parents[from].unwrap() != edge.to {
            if visitor(DfsEvent::BackEdge(from, edge.to)) {
                return;
            }
        } else if is_direct && visitor(DfsEvent::ForwardOrCrossEdge(from, edge.to)) {
            return;
        }
    }
    colors[from] = Color::Black;
    if visitor(DfsEvent::FinishVertex(from, *timer)) {}
}

