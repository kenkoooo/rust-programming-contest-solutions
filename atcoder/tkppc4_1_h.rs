use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INF: u64 = std::u64::MAX / 2;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let k: u64 = sc.read();
    let mut t: Vec<u64> = vec![0; n];
    for i in 1..(n - 1) {
        t[i] = sc.read();
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c: u64 = sc.read();
        let d: u64 = sc.read();
        graph[a].push((b, c, d));
        graph[b].push((a, c, d));
    }

    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(State {
        v: 0,
        cost: dist[0],
    });
    while let Some(State { v, cost }) = heap.pop() {
        if cost > k {
            break;
        }
        if v == n - 1 {
            println!("{}", cost);
            return;
        }
        for &(next, c, d) in graph[v].iter() {
            let next_cost = (cost + t[v] + d - 1) / d * d + c;
            if dist[next] > next_cost {
                dist[next] = next_cost;
                heap.push(State {
                    v: next,
                    cost: next_cost,
                });
            }
        }
    }
    println!("-1");
}

#[derive(Eq, PartialEq)]
struct State {
    v: usize,
    cost: u64,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
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
