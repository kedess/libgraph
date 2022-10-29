## libgraph
Library for working with graphs

- [bfs](#bfs)
- [bfs visitor](#bfs-visitor)
- [dfs](#dfs)
- [dfs visitor](#dfs-visitor)

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
#### <a id="bfs"/> bfs
Algorithmic complexity <b>O(V + E)</b>, where V is the number of vertices in the graph and E is the number of edges.
```rust
use libgraph::{bfs, path_iter, Graph, GraphKind};

fn main(){
    let mut graph = GraphKind::Undirected(Graph::new(100));
    graph.add_edge(0, 1, 0.0).unwrap();
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();
    let parents = bfs(&graph, 0).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 2, 1, 0]);
    assert_eq!(path_iter(19, &parents).collect::<Vec<usize>>(), vec![]);
}
```

#### <a id="bfs-visitor"/> bfs visitor
```rust
use libgraph::{GraphKind, Graph, bfs_visitor, BfsEvent};

fn main() {
    let mut graph = GraphKind::Directed(Graph::new(100));

    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(2, 4, 0.0).unwrap();
    graph.add_edge(2, 5, 0.0).unwrap();
    graph.add_edge(4, 8, 0.0).unwrap();
    graph.add_edge(8, 17, 0.0).unwrap();

    let mut vertexes = vec![];
    let _parents = bfs_visitor(&graph, 1, |event|{
        match event {
            // The event is called before you start exploring the vertex (extracting it from the queue).
            // Событие вызывается до того, как вы начнете исследовать вершину (извлекая ее из очереди).
            BfsEvent::ExamineVertex(usize) =>{

            }
            // The event is called when the algorithm first encounters a vertex in the traversal.
            // Событие вызывается, когда алгоритм впервые встречает вершину в обходе.
            BfsEvent::DiscoverVertex(vertex) => {
                vertexes.push(vertex);
            }
            // The event is triggered when the edge under investigation is an edge of the tree after traversal.
            // Событие срабатывает, когда исследуемое ребро является ребром дерева после обхода.
            BfsEvent::IsTreeEdge(from, to) => {

            }
            // The event is triggered when the edge under investigation is not an edge of the tree after traversal.
            // Событие срабатывает, когда исследуемое ребро не является ребром дерева после обхода.
            BfsEvent::IsNotTreeEdge(from, to) => {

            }
            // The event is triggered if the target vertex is grayed out at the time of the study.
            // Gray color means that the vertex is currently in the queue.
            // Событие вызывается, если в момент исследования целевая вершина выделена серым цветом
            // Серый цвет означает, что вершина в данный момент находится в очереди.
            BfsEvent::GrayTarget(usize) => {

            }
            // The event is triggered if the target node is colored black at the time of the study.
            // The black color means that the vertex is no longer in the queue.
            // Событие вызывается, если в момент исследования целевой узел окрашен в черный цвет. 
            // Черный цвет означает, что вершина больше не находится в очереди.
            BfsEvent::BlackTarget(usize) =>{

            }
            // The event is called after examining all outgoing edges and all neighboring vertices.
            // Событие вызывается после изучения всех исходящих ребер и всех соседних вершин.
            BfsEvent::FinishVertex(vertex) => {

            }
        }
        // If true is returned, the traversal will be completed after calling this callback
        // Если возвращается true, то обход будет завершен после вызова этого обратного вызова
        false 
    }).unwrap();

    assert_eq!(vertexes, vec![1, 2, 3, 4, 5, 8, 17]);
}
```
#### <a id="dfs"/> dfs
Algorithmic complexity <b>O(V + E)</b>, where V is the number of vertices in the graph and E is the number of edges.

```rust
use libgraph::{dfs, path_iter, Graph, GraphKind};

fn main(){
    let mut graph = GraphKind::Directed(Graph::new(10));
    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();

    let parents = dfs(&graph, 1).unwrap();
    assert_eq!(path_iter(5, &parents).collect::<Vec<usize>>(), vec![5, 3, 2, 1]);

    let parents = dfs(&graph, 1).unwrap();
    assert_eq!(path_iter(77, &parents).collect::<Vec<usize>>(), vec![]);
}
```

#### <a id="dfs-visitor"/> dfs visitor

```rust
use libgraph::{dfs, dfs_visitor, DfsEvent, path_iter, Graph, GraphKind};

fn main(){
    let mut graph = GraphKind::Directed(Graph::new(10));

    graph.add_edge(1, 2, 0.0).unwrap();
    graph.add_edge(2, 3, 0.0).unwrap();
    graph.add_edge(3, 5, 0.0).unwrap();

    let mut vertexes = vec![];

    let _ = dfs_visitor(&graph, 1, |event|{
        match event {
            // The event is called when the algorithm first encounters a vertex in the traversal
            // Событие вызывается, когда алгоритм впервые встречает вершину в обходе
            DfsEvent::DiscoverVertex(vertex, _time_in) => {
                vertexes.push(vertex);
            }
            // The event is called before you start exploring the vertex.
            // Событие вызывается до того, как вы начнете исследовать вершину.
            DfsEvent::ExamineVertex(_vertex) => {

            }
            // The event is triggered when the edge under investigation is an edge of the tree after traversal.
            // Событие срабатывает, когда исследуемое ребро является ребром дерева после обхода.
            DfsEvent::IsTreeEdge(_from, _to) => {

            }
            // The event is triggered when we return to the ancestor from which we explored the vertex.
            // Событие срабатывает, когда мы возвращаемся к предку, из которого мы исследовали вершину.
            DfsEvent::ReturnParent(_from,_) => {

            }
            // The event is triggered when we try to follow the reverse edge.
            // Событие срабатывает, когда мы пытаемся пойти по обратному ребру.
            DfsEvent::BackEdge(_from, _to) => {

            }
            // The event is triggered when we try to walk along a straight or transverse edge (only for a directed graph).
            // Событие срабатывает, когда мы пытаемся пройти по прямому или поперечному ребру (только для ориентированного графа).
            DfsEvent::ForwardOrCrossEdge(_, _to) => {

            }
            // The event is called after examining all outgoing edges and all neighboring vertices.
            // Событие вызывается после изучения всех исходящих ребер и всех соседних вершин.
            DfsEvent::FinishVertex(_vertex, _time_out) => {
            }
            _ =>{

            }
        }
        // If true is returned, the traversal will be completed after calling this callback
        // Если возвращается true, то обход будет завершен после вызова этого обратного вызова
        false
    }).unwrap();
    assert_eq!(vertexes, vec![1, 2, 3, 5]);
}
```


### Cargo.toml
```bash
[dependencies]
libgraph = {git = "https://github.com/mingendo/libgraph.git", branch="main"}
```
