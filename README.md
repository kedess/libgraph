## libgraph
Library for working with graphs

```rust
use libgraph::{GraphKind, Graph, version};
fn main() {

    let ver = version();
    
    /*
    * Creating a graph of 10 vertices. Vertex numbering starts from 0.
    * 10 vertices will be created that are not connected to each other
    */
    let mut graph = GraphKind::Undirected::<f64>(Graph::new(10));

    /*
    * Connecting the vertices
    */
    graph.add_edge(1, 2, 2.0).unwrap();
    graph.add_edge(2, 3, 2.0).unwrap();
    graph.add_edge(3, 7, 3.0).unwrap();
}
```

### Cargo.toml
```bash
[dependencies]
libgraph = {git = "https://github.com/mingendo/libgraph.git", branch="main"}
```
