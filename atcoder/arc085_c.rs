const INF: i64 = 1e15 as i64;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let a: Vec<i64> = sc.read_vec(n);

    let source = n;
    let sink = n + 1;
    let v = n + 2;
    let mut graph = dinitz::Dinitz::new(v);
    for i in 0..n {
        if a[i] < 0 {
            graph.add_edge(source, i, -a[i]);
        } else {
            graph.add_edge(i, sink, a[i]);
        }
    }

    for i in 0..n {
        let from = i + 1;
        for j in 2.. {
            let to = from * j;
            if to - 1 >= n {
                break;
            }
            graph.add_edge(from - 1, to - 1, INF);
        }
    }

    let total = a.iter().filter(|&&a| a > 0).sum::<i64>();

    println!("{}", total - graph.max_flow(source, sink));
}

pub mod dinitz {
    use std::cmp;
    use std::collections::VecDeque;
    use std::i64;
    use std::usize;

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
                    let f = self.dfs(s, t, i64::MAX);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }
}
pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
