use std::usize;
use std::cmp;
use std::collections::vec_deque::VecDeque;
use std::i64::MAX;

fn main() {
    let (r, c) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };

    let dot = ".".as_bytes()[0];
    let map = (0..r).map(|_| {
        read_line()
            .trim()
            .bytes()
            .map(|b| b == dot)
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut ok = 0;

    let mut dinitz = Dinitz::new(r * c + 2);
    let source = r * c;
    let sink = source + 1;
    for i in 0..r {
        for j in 0..c {
            if !map[i][j] { continue; }

            ok += 1;

            let v = i * c + j;
            if i % 2 == j % 2 {
                dinitz.add_edge(source, v, 1);
            } else {
                dinitz.add_edge(v, sink, 1);
                continue;
            }

            let x = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            for t in &x {
                let (di, dj) = *t;
                let ni = (i as i32) + di;
                let nj = (j as i32) + dj;
                if ni < 0 || ni >= r as i32 || nj < 0 || nj >= c as i32 {
                    continue;
                }
                let w = ni * (c as i32) + nj;
                dinitz.add_edge(v, w as usize, 1);
            }
        }
    }

    let f = dinitz.max_flow(source, sink);
    println!("{}", ok - f as usize);
}

struct Edge {
    pub to: usize,
    pub rev: usize,
    pub cap: i64,
}

pub struct Dinitz {
    g: Vec<Vec<Edge>>,
    level: Vec<i32>,
    iter: Vec<usize>,
}

impl Dinitz {
    pub fn new(v: usize) -> Dinitz {
        let mut g: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..v {
            g.push(Vec::new());
        }
        Dinitz {
            g: g,
            level: vec![0; v],
            iter: vec![0; v],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let to_len = self.g[to].len();
        let from_len = self.g[from].len();
        self.g[from].push(Edge {
            to: to,
            rev: to_len,
            cap: cap,
        });
        self.g[to].push(Edge {
            to: from,
            rev: from_len,
            cap: 0,
        });
    }

    fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
        if v == t {
            return f;
        }
        while self.iter[v] < self.g[v].len() {
            let (e_cap, e_to, e_rev);
            {
                let ref e = self.g[v][self.iter[v]];
                e_cap = e.cap;
                e_to = e.to;
                e_rev = e.rev;
            }
            if e_cap > 0 && self.level[v] < self.level[e_to] {
                let d = self.dfs(e_to, t, cmp::min(f, e_cap));
                if d > 0 {
                    {
                        let ref mut e = self.g[v][self.iter[v]];
                        e.cap -= d;
                    }
                    {
                        let ref mut rev_edge = self.g[e_to][e_rev];
                        rev_edge.cap += d;
                    }
                    return d;
                }
            }
            self.iter[v] += 1;
        }

        return 0;
    }

    fn bfs(&mut self, s: usize) {
        let v = self.level.len();
        self.level = vec![-1; v];
        self.level[s] = 0;
        let mut deque = VecDeque::new();
        deque.push_back(s);
        while !deque.is_empty() {
            let v = deque.pop_front().unwrap();
            for e in &self.g[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    deque.push_back(e.to);
                }
            }
        }
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let v = self.level.len();
        let mut flow: i64 = 0;
        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                return flow;
            }
            self.iter = vec![0; v];
            loop {
                let f = self.dfs(s, t, MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
    }
}

fn read_line() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug, {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}