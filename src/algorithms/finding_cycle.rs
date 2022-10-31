use crate::{dfs_visitor, DfsEvent, GraphError, GraphKind};

pub fn finding_cycle<T>(graph: &GraphKind<T>) -> Result<Option<Vec<usize>>, GraphError> where T: Default + Copy + std::fmt::Debug {
    let adj = match graph {
        GraphKind::Undirected(_graph) => &_graph.adj,
        GraphKind::Directed(_graph) => &_graph.adj,
    };
    let mut visited = vec![false; adj.len()];
    let mut cycle = None;
    for vertex in 0..adj.len() {
        if !visited[vertex] {
            let mut stack = Stack::<usize>::new();
            dfs_visitor(graph, vertex, |event| {
                match event {
                    DfsEvent::DiscoverVertex(vertex, _) => {
                        stack.push(vertex);
                    }
                    DfsEvent::FinishVertex(vertex, _) => {
                        stack.pop();
                        visited[vertex] = true;
                    }
                    DfsEvent::BackEdge(_, to) => {
                        let mut start = stack.pop().unwrap();
                        let mut vertexes = vec![start];
                        while start != to {
                            start = stack.pop().unwrap();
                            vertexes.push(start);
                        }
                        cycle = Some(vertexes);
                    }
                    _ => {}
                }
                false
            })?;
            if cycle.is_some() {
                return Ok(cycle);
            }
        }
    }
    Ok(None)
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    #[allow(unused)]
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    #[allow(unused)]
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    #[allow(unused)]
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_stack() {
    let mut st = Stack::new();
    st.push(1);
    st.push(2);
    st.push(3);
    assert_eq!(st.pop().unwrap(), 3);
    assert_eq!(st.pop().unwrap(), 2);
    assert_eq!(st.pop().unwrap(), 1);
    assert_eq!(st.pop(), None);
    assert_eq!(st.pop(), None);
}

