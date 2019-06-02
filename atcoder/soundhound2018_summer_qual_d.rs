use std::cmp;
use std::collections::BinaryHeap;

const INF: usize = 1e16 as usize;

#[derive(Eq, PartialEq)]
struct State {
    v: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();
    let s = sc.usize_read() - 1;
    let t = sc.usize_read() - 1;

    let mut graph_a = vec![vec![]; n];
    let mut graph_b = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.usize_read() - 1;
        let v = sc.usize_read() - 1;
        let a = sc.read();
        let b = sc.read();
        graph_a[u].push((v, a));
        graph_a[v].push((u, a));
        graph_b[u].push((v, b));
        graph_b[v].push((u, b));
    }

    let dist_a = dijkstra(&graph_a, s);
    let dist_b = dijkstra(&graph_b, t);

    let mut ans = vec![0; n];
    for i in (0..n).rev() {
        ans[i] = dist_a[i] + dist_b[i];
        if i < n - 1 {
            ans[i] = cmp::min(ans[i], ans[i + 1]);
        }
    }

    for &ans in &ans {
        println!("{}", (1e15 as usize) - ans);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, from: usize) -> Vec<usize> {
    let n = graph.len();
    let mut heap = BinaryHeap::new();
    let mut dist = vec![INF; n];
    dist[from] = 0;
    heap.push(State { v: from, cost: 0 });
    while let Some(State { v, cost: _ }) = heap.pop() {
        for &(next, weight) in &graph[v] {
            if dist[next] > dist[v] + weight {
                dist[next] = dist[v] + weight;
                heap.push(State {
                    v: next,
                    cost: dist[next],
                });
            }
        }
    }
    dist
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
