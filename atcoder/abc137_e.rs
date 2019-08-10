use std::cmp;
use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let p: i64 = sc.read();
    let mut edges = vec![];
    let mut inverse = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c: i64 = sc.read();
        edges.push((a, b, c));
        inverse[b].push(a);
    }

    let mut reach = vec![false; n];
    reach[n - 1] = true;
    let mut q = VecDeque::new();
    q.push_back(n - 1);
    while let Some(v) = q.pop_front() {
        for &prev in inverse[v].iter() {
            if !reach[prev] {
                reach[prev] = true;
                q.push_back(prev);
            }
        }
    }

    let mut graph = vec![vec![]; n];
    for (a, b, c) in edges.into_iter() {
        if reach[a] && reach[b] {
            graph[a].push((b, p - c));
        }
    }

    let inf = std::i64::MAX;
    let (dist, negative) = bellman_ford::shortest_path(&graph, 0, inf);
    let mut neg = false;
    for b in negative.into_iter() {
        neg = neg || b;
    }
    if neg {
        println!("-1");
    } else {
        println!("{}", cmp::max(-dist[n - 1], 0));
    }
}

pub mod bellman_ford {
    pub fn shortest_path(
        graph: &Vec<Vec<(usize, i64)>>,
        start: usize,
        inf: i64,
    ) -> (Vec<i64>, Vec<bool>) {
        let n = graph.len();
        let mut dist = vec![inf; n];
        dist[start] = 0;
        for _ in 0..n {
            for v in 0..n {
                for &(to, cost) in &graph[v] {
                    if dist[v] == inf || dist[to] <= dist[v] + cost {
                        continue;
                    }
                    dist[to] = dist[v] + cost;
                }
            }
        }

        let mut negative = vec![false; n];
        for _ in 0..n {
            for v in 0..n {
                for &(to, cost) in &graph[v] {
                    if dist[v] == inf {
                        continue;
                    }
                    if dist[to] > dist[v] + cost {
                        dist[to] = dist[v] + cost;
                        negative[to] = true;
                    }
                    if negative[v] {
                        negative[to] = true;
                    }
                }
            }
        }

        return (dist, negative);
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
