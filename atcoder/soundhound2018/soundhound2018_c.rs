use std::usize;
use std::cmp;
use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let r = sc.read::<usize>();
    let c = sc.read::<usize>();

    let map: Vec<Vec<u8>> = (0..r).map(|_| sc.read::<String>().bytes().collect()).collect();

    let source = r * c;
    let sink = source + 1;
    let v = sink + 1;

    let mut ok = 0;
    let mut dinitz = Dinitz::new(v);
    for i in 0..r {
        for j in 0..c {
            if map[i][j] == b'*' {
                continue;
            }
            ok += 1;

            let u = i * c + j;
            if i % 2 == j % 2 {
                dinitz.add_edge(source, u, 1);
            } else {
                dinitz.add_edge(u, sink, 1);
                continue;
            }


            let mut add = |x: usize, y: usize| {
                dinitz.add_edge(u, x * c + y, 1);
            };

            if i > 0 {
                add(i - 1, j);
            }
            if i < r - 1 {
                add(i + 1, j);
            }
            if j > 0 {
                add(i, j - 1);
            }
            if j < c - 1 {
                add(i, j + 1);
            }
        }
    }

    println!("{}", ok - dinitz.max_flow(source, sink));
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
                let f = self.dfs(s, t, std::i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
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
