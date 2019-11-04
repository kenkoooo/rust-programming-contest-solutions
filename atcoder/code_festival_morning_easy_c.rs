use std::cmp;
use std::collections::HashMap;

const INF: u64 = std::u64::MAX / 2;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let s = sc.read::<usize>() - 1;
    let t = sc.read::<usize>() - 1;

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read::<u64>();
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let dist1 = dijkstra(s, &graph);
    let dist2 = dijkstra(t, &graph);
    for i in 0..n {
        if dist1[i] == dist2[i] && dist1[i] != INF {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}

fn dijkstra(s: usize, graph: &Vec<Vec<(usize, u64)>>) -> Vec<u64> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    dist[s] = 0;
    let mut heap = FibonacciHeap::new(cmp::min);
    heap.push((0, s));
    while let Some((_, v)) = heap.pop() {
        for &(next, cost) in graph[v].iter() {
            if dist[next] > dist[v] + cost {
                dist[next] = dist[v] + cost;
                heap.push((dist[next], next));
            }
        }
    }
    dist
}

fn store_to_map<T, F>(map: &mut HashMap<usize, Node<T>>, mut node: Node<T>, ordering: F)
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T,
{
    let d = node.children.len();
    match map.remove(&d) {
        Some(mut other) => {
            let node = if (ordering)(node.key, other.key) == node.key {
                node.children.push(other);
                node
            } else {
                other.children.push(node);
                other
            };
            store_to_map(map, node, ordering);
        }
        None => {
            map.insert(d, node);
        }
    }
}

pub struct FibonacciHeap<T, F> {
    pub nodes: Vec<Node<T>>,
    min_index: usize,
    ordering: F,
}

impl<T, F> FibonacciHeap<T, F>
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T + Copy,
{
    pub fn new(ordering: F) -> FibonacciHeap<T, F> {
        FibonacciHeap {
            nodes: Vec::new(),
            min_index: 0,
            ordering: ordering,
        }
    }
    pub fn push(&mut self, x: T) {
        let node = Node::new(x);
        self.nodes.push(node);
        let cur_min = self.nodes[self.min_index].key;
        if (self.ordering)(cur_min, x) == x {
            self.min_index = self.nodes.len() - 1;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        let mut map: HashMap<usize, Node<T>> = HashMap::new();
        let mut popped = None;

        let mut nodes = Vec::new();
        std::mem::swap(&mut self.nodes, &mut nodes);
        for (i, node) in nodes.into_iter().enumerate() {
            if i == self.min_index {
                popped = Some(node.key);
                for node in node.children.into_iter() {
                    store_to_map(&mut map, node, self.ordering);
                }
            } else {
                store_to_map(&mut map, node, self.ordering);
            }
        }

        self.nodes = map.into_iter().map(|(_, node)| node).collect();
        if !self.nodes.is_empty() {
            let mut min = self.nodes[0].key;
            for i in 0..self.nodes.len() {
                if (self.ordering)(min, self.nodes[i].key) == self.nodes[i].key {
                    min = self.nodes[i].key;
                }
            }

            self.min_index = self
                .nodes
                .iter()
                .enumerate()
                .find(|&(_, node)| node.key == min)
                .unwrap()
                .0;
        } else {
            self.min_index = 0;
        }
        popped
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub key: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(x: T) -> Node<T> {
        Node {
            key: x,
            children: Vec::new(),
        }
    }
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
