use crate::{Color, GraphError, GraphKind, Parents};
use std::collections::VecDeque;
pub fn bfs<T>(graph: &GraphKind<T>, from: usize) -> Result<Parents, GraphError> where T: Copy {
    let adj = match graph {
        GraphKind::Undirected(graph) => &graph.adj,
        GraphKind::Directed(graph) => &graph.adj,
    };
    if from >= adj.len() {
        return Err(GraphError::new(
            "Unable to start a traversal. Specified vertex number exceeds allowed graph size",
        ));
    }
    let mut queue = VecDeque::new();
    let mut parents = vec![None; adj.len()];
    let mut colors = vec![Color::White; adj.len()];
    queue.push_back(from);
    colors[from] = Color::Grey;
    parents[from] = None;
    while let Some(vertex) = queue.pop_front() {
        for edge in adj[vertex].iter() {
            if colors[edge.to] == Color::White {
                parents[edge.to] = Some(vertex);
                queue.push_back(edge.to);
                colors[edge.to] = Color::Grey;
            }
        }
        colors[vertex] = Color::Black;
    }
    Ok(parents)
}

pub enum BfsEvent {
    ExamineVertex(usize),
    DiscoverVertex(usize),
    IsTreeEdge(usize, usize),
    IsNotTreeEdge(usize, usize),
    GrayTarget(usize),
    BlackTarget(usize),
    FinishVertex(usize),
}

pub fn bfs_visitor<T, F: FnMut(BfsEvent) -> bool>(graph: &GraphKind<T>, from: usize, mut visitor: F, ) -> Result<Parents, GraphError>
where T: Copy {
    let adj = match graph {
        GraphKind::Undirected(graph) => &graph.adj,
        GraphKind::Directed(graph) => &graph.adj,
    };
    if from >= adj.len() {
        return Err(GraphError::new(
            "Unable to start a traversal. Specified vertex number exceeds allowed graph size",
        ));
    }
    let mut queue = VecDeque::new();
    let mut parents = vec![None; adj.len()];
    let mut colors = vec![Color::White; adj.len()];
    queue.push_back(from);
    if visitor(BfsEvent::DiscoverVertex(from)) {
        return Ok(parents);
    }
    colors[from] = Color::Grey;
    parents[from] = None;
    while let Some(vertex) = queue.pop_front() {
        if visitor(BfsEvent::ExamineVertex(vertex)) {
            return Ok(parents);
        }
        for edge in adj[vertex].iter() {
            if colors[edge.to] == Color::White {
                if visitor(BfsEvent::IsTreeEdge(from, edge.to)) {
                    return Ok(parents);
                }
                if visitor(BfsEvent::DiscoverVertex(edge.to)) {
                    return Ok(parents);
                }
                parents[edge.to] = Some(vertex);
                queue.push_back(edge.to);
                colors[edge.to] = Color::Grey;
            }
            if visitor(BfsEvent::IsNotTreeEdge(from, edge.to)) {
                return Ok(parents);
            }
            if colors[vertex] == Color::Grey {
                if visitor(BfsEvent::GrayTarget(vertex)) {
                    return Ok(parents);
                }
            } else if visitor(BfsEvent::BlackTarget(vertex)) {
                return Ok(parents);
            }
        }
        colors[vertex] = Color::Black;
        if visitor(BfsEvent::FinishVertex(vertex)) {
            return Ok(parents);
        }
    }
    Ok(parents)
}

