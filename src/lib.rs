use std::collections::VecDeque;

struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { items: VecDeque::new() }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push_back(v);
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from empty queue")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

type Vertex = Vec<u32>;
type Graph = Vec<Vertex>;

fn bfs(graph: Graph, start_node: u32, end_node: u32) -> Option<Vec<Option<u32>>> {
    let mut queue = Queue::new();
    queue.enqueue(start_node);

    let mut visited_vertices = vec![false; graph.len()];
    visited_vertices[0] = true;

    let mut prev: Vec<Option<u32>> = vec![None; graph.len()];

    'outer_loop: while !queue.is_empty() {
        let curr_node = queue.dequeue();
        for v in graph[curr_node as usize].iter() {
            if *v == end_node {
                prev[*v as usize] = Some(curr_node);
                break 'outer_loop;
            }

            if !visited_vertices[*v as usize] {
                queue.enqueue(*v);
                visited_vertices[*v as usize] = true;
                prev[*v as usize] = Some(curr_node);
            }
        }
    }

    let mut path = Vec::new();
    let mut at = Some(end_node);
    while at != None {
        path.push(at);
        at = prev[at.unwrap_or(0) as usize];
    }

    path.reverse();

    match path[0] {
        Some(x) if x == start_node => Some(path),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
