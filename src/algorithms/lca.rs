use crate::{depths_vertices, dfs_visitor, DfsEvent, GraphError, GraphKind};
use std::cmp::{max, min};
use std::fmt;

pub struct Lca {
    rmq: RMinQ,
    borders: Vec<usize>,
}
impl Lca {
    pub fn build<T>(graph: &GraphKind<T>, root: usize) -> Result<Self, GraphError> where T: Copy {
        match graph {
            GraphKind::Undirected(_graph) => {
                let adj = &_graph.adj;
                let depths = depths_vertices(graph, root)?;
                if depths.iter().any(|value| value.is_none()) {
                    return Err(GraphError::new("Graph is not a tree"));
                };
                let mut vertexes = vec![];
                let _ = dfs_visitor(graph, root, |event| {
                    match event {
                        DfsEvent::DiscoverVertex(from, _) => {
                            vertexes.push(from);
                        }
                        DfsEvent::ReturnParent(_, to) => {
                            vertexes.push(to);
                        }
                        _ => {}
                    }
                    false
                })?;
                let depths = depths.iter().map(|value| value.unwrap()).collect::<Vec<usize>>();
                let mut borders = vec![0; adj.len()];
                for (idx, vertex) in vertexes.iter().enumerate().rev() {
                    borders[*vertex] = idx;
                }
                let rmq = RMinQ::build(vertexes, depths).map_err(|err| GraphError::new(&err.to_string()))?;
                Ok(Lca { rmq, borders })
            }
            GraphKind::Directed(_) => {
                unimplemented!()
            }
        }
    }
    pub fn query(&self, first: usize, second: usize) -> Result<usize, GraphError> {
        let l = min(self.borders[first], self.borders[second]);
        let r = max(self.borders[first], self.borders[second]);
        self.rmq.min(l, r).map_err(|err| GraphError::new(&err.to_string()))
    }
}
struct SegmentTreeError {
    msg: String,
}
impl SegmentTreeError {
    fn new(msg: &str) -> Self {
        SegmentTreeError { msg: msg.to_string() }
    }
}
impl fmt::Display for SegmentTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}
struct RMinQ {
    values: Vec<usize>,
    depths: Vec<usize>,
    sz: usize,
    bound: usize,
}
impl RMinQ {
    fn build(values: Vec<usize>, depths: Vec<usize>) -> Result<Self, SegmentTreeError> {
        if values.is_empty() {
            return Err(SegmentTreeError::new("The value list cannot be empty"));
        }
        let mut n = 1usize;
        while n < values.len() {
            n <<= 1;
        }
        let mut dst: Vec<usize> = vec![usize::MAX; n << 1];
        build_inner_rminq(&mut dst, &values, &depths, 0, 0, n - 1);
        Ok(RMinQ {
            bound: values.len(),
            values: dst,
            depths,
            sz: n,
        })
    }
    pub fn min(&self, l: usize, r: usize) -> Result<usize, SegmentTreeError> {
        if l < self.bound && r < self.bound {
            return Ok(self.min_inner(0, 0, self.sz - 1, l, r));
        }
        Err(SegmentTreeError::new("Not valid range"))
    }
    fn min_inner(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> usize {
        if l > r {
            return usize::MAX;
        }
        if tl == l && tr == r {
            return self.values[v];
        }
        let m = (tl + tr) >> 1;
        if r <= m {
            return self.min_inner((v << 1) + 1, tl, m, l, min(r, m));
        }
        if l > m {
            return self.min_inner((v << 1) + 2, m + 1, tr, max(l, m + 1), r);
        }
        let left_value = self.min_inner((v << 1) + 1, tl, m, l, min(r, m));
        let right_value = self.min_inner((v << 1) + 2, m + 1, tr, max(l, m + 1), r);
        let left = {
            if left_value < usize::MAX {
                self.depths[left_value]
            } else {
                usize::MAX
            }
        };
        let right = {
            if right_value < usize::MAX {
                self.depths[right_value]
            } else {
                usize::MAX
            }
        };
        if left < right {
            left_value
        } else {
            right_value
        }
    }
}
fn build_inner_rminq(dst: &mut [usize], src: &[usize], depths: &[usize], v: usize, tl: usize, tr: usize) {
    if tl == tr {
        if tl < src.len() {
            dst[v] = src[tl];
        }
    } else {
        let m = (tl + tr) >> 1;
        build_inner_rminq(dst, src, depths, (v << 1) + 1, tl, m);
        build_inner_rminq(dst, src, depths, (v << 1) + 2, m + 1, tr);
        let left = {
            if dst[(v << 1) + 1] < usize::MAX {
                depths[dst[(v << 1) + 1]]
            } else {
                usize::MAX
            }
        };
        let right = {
            if dst[(v << 1) + 2] < usize::MAX {
                depths[dst[(v << 1) + 2]]
            } else {
                usize::MAX
            }
        };
        if left < right {
            dst[v] = dst[(v << 1) + 1];
        } else {
            dst[v] = dst[(v << 1) + 2];
        }
    }
}

