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

pub fn bfs(graph: Graph, start_node: u32, end_node: u32) -> Option<Vec<Option<u32>>> {
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
    use super::*;
    #[test]
    fn queue_enqueue() {
        let mut q = Queue::new();
        q.enqueue(0);
        assert!(!q.is_empty());
    }

    #[test]
    fn queue_dequeue() {
        let mut q = Queue::new();
        q.enqueue(15);
        assert_eq!(q.dequeue(), 15);
    }

    #[test]
    fn queue_multi_dequeue() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.dequeue(), 2);
        assert_eq!(q.dequeue(), 3);
    }

    #[test]
    #[should_panic(expected = "Cannot dequeue from empty queue")]
    fn queue_dequeue_empty() {
        let mut q:Queue<i32> = Queue::new();
        q.dequeue();
    }
}
