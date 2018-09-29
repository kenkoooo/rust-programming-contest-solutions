use std::cmp;
use std::collections::BinaryHeap;

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

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut graph: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; n];
    let mut dist = vec![vec![std::usize::MAX / 2; n]; n];
    for i in 0..n { dist[i][i] = 0; }
    for i in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read();
        graph[a].push((b, c, i));
        graph[b].push((a, c, i));
        dist[a][b] = c;
        dist[b][a] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut used = vec![false; m];
    for from in 0..n {
        let mut heap = BinaryHeap::new();
        heap.push(State { v: from, cost: 0 });
        let mut vis = vec![false; n];
        vis[0] = true;
        while let Some(State { v, cost }) = heap.pop() {
            for e in &graph[v] {
                let (to, c, i) = *e;
                if dist[from][v] + c != dist[from][to] { continue; }
                used[i] = true;
                if !vis[to] {
                    heap.push(State { v: to, cost: dist[from][to] });
                    vis[to] = true;
                }
            }
        }
    }

    println!("{}", used.iter().map(|b| if *b { 0 } else { 1 }).sum::<usize>());
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

