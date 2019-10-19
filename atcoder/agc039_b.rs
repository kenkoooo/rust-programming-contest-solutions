use std::cmp;
use std::collections::{BTreeSet, VecDeque};

const INF: usize = 1000000;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let edge: Vec<Vec<bool>> = (0..n)
        .map(|_| sc.chars().into_iter().map(|c| c == '1').collect())
        .collect();

    let mut ans: Option<usize> = None;
    for start in 0..n {
        if let Some(t) = bfs(start, &edge) {
            match ans {
                Some(cur) => ans = Some(cmp::max(cur, t)),
                None => ans = Some(t),
            }
        }
    }

    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn bfs(start: usize, edge: &Vec<Vec<bool>>) -> Option<usize> {
    let n = edge.len();
    let mut distances = vec![vec![]; n];
    distances[start].push(0);
    let mut dist = vec![INF; n];
    dist[start] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(v) = q.pop_front() {
        for next in 0..n {
            if !edge[v][next] {
                continue;
            }
            let next_dist = dist[v] + 1;
            if dist[next] > next_dist {
                dist[next] = next_dist;
                distances[next].push(next_dist);
                q.push_back(next);
            } else if dist[next] < next_dist {
                distances[next].push(next_dist);
            }
        }
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        let min = distances[i][0];
        if distances[i].iter().any(|&x| (x - min) % 2 != 0) {
            return None;
        }
        set.insert(min);
    }
    Some(set.len())
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
