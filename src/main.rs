use bfs::*;
fn main() {
    let v0:Vertex = vec![1, 2, 3, 4];
    let v1:Vertex = vec![4];
    let v2:Vertex = vec![0, 6];
    let v3:Vertex = vec![0, 4, 5];
    let v4:Vertex = vec![0, 1, 3, 6];
    let v5:Vertex = vec![3, 6, 7];
    let v6:Vertex = vec![2, 4, 5, 7];
    let v7:Vertex = vec![5, 6];

    let graph:Graph = vec![v0, v1, v2, v3, v4, v5, v6, v7];

    match bfs(graph, 0, 7) {
        Some(path) => {
            for node in path.iter() {
                println!("Move to node {}", node.unwrap());
            }
        }
        None => {
            println!("No path found.");
        }
    }
}