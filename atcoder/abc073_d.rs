use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

const MAX_COST: usize = 1 << 50;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    mask: usize,
    vertex: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let r: usize = sc.read();
    let towns: Vec<usize> = (0..r).map(|_| sc.read::<usize>() - 1).collect();

    let mut graph = vec![vec![MAX_COST; n]; n];
    for i in 0..n {
        graph[i][i] = 0;
    }
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read::<usize>();
        graph[a][b] = c;
        graph[b][a] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] = cmp::min(graph[i][j], graph[i][k] + graph[k][j]);
            }
        }
    }

    let mut bit_dp = vec![vec![MAX_COST; r]; 1 << r];
    let mut heap = BinaryHeap::new();
    for i in 0..r {
        let mask = 1 << i;
        bit_dp[mask][i] = 0;
        heap.push(State { cost: 0, mask: mask, vertex: i })
    }

    while let Some(State { cost, mask, vertex }) = heap.pop() {
        if mask == (1 << r) - 1 {
            println!("{}", cost);
            return;
        }

        for to in 0..r {
            if ((1 << to) & mask) != 0 { continue; }
            let e = graph[towns[vertex]][towns[to]];
            let next = mask | (1 << to);
            if bit_dp[next][to] <= bit_dp[mask][vertex] + e { continue; }
            bit_dp[next][to] = bit_dp[mask][vertex] + e;
            heap.push(State { cost: bit_dp[next][to], mask: next, vertex: to });
        }
    }
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

