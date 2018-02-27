use std::collections::BinaryHeap;

const MOD: usize = 1_000_000_007;

#[derive(Copy, Clone, Eq, PartialEq)]
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

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize, t: usize) -> (Vec<usize>, Vec<usize>) {
    let mut dist = vec![std::usize::MAX; graph.len()];
    let mut pattern = vec![0; graph.len()];
    let mut heap = BinaryHeap::new();
    dist[s] = 0;
    pattern[s] = 1;
    heap.push(State { cost: 0, v: s });
    while let Some(State { v, cost }) = heap.pop() {
        if v == t { break; }
        if cost > dist[v] {
            continue;
        }
        for edge in graph.get(v).unwrap().iter() {
            let (to, d) = *edge;
            if cost + d == dist[to] {
                pattern[to] = (pattern[to] + pattern[v]) % MOD;
            } else if cost + d < dist[to] {
                pattern[to] = pattern[v];
                dist[to] = cost + d;
                heap.push(State { v: to, cost: cost + d });
            }
        }
    }

    (dist, pattern)
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let s: usize = sc.read::<usize>() - 1;
    let t: usize = sc.read::<usize>() - 1;
    let mut edges = Vec::new();
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        let d: usize = sc.read();
        graph[u].push((v, d));
        graph[v].push((u, d));
        edges.push((v, u, d));
    }

    let (s_dist, s_pattern) = dijkstra(&graph, s, t);
    let (t_dist, t_pattern) = dijkstra(&graph, t, s);

    let length = s_dist[t];

    let mut ans = (s_pattern[t] * s_pattern[t]) % MOD;
    for edge in &edges {
        let (from, to, d) = {
            let (v, u, d) = *edge;
            if s_dist[v] < s_dist[u] {
                (v, u, d)
            } else {
                (u, v, d)
            }
        };

        if s_dist[to] != s_dist[from] + d { continue; }
        if s_dist[from] + d + t_dist[to] != length { continue; }

        if s_dist[from] * 2 < length && length < s_dist[to] * 2 {
            let mut tmp = (s_pattern[from] * t_pattern[to]) % MOD;
            tmp = (tmp * tmp) % MOD;
            ans = (ans + MOD - tmp) % MOD;
        }
    }

    for v in 0..n {
        if s_dist[v] * 2 == length {
            let mut tmp = (s_pattern[v] * t_pattern[v]) % MOD;
            tmp = (tmp * tmp) % MOD;
            ans = (ans + MOD - tmp) % MOD;
        }
    }

    println!("{}", ans);
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

