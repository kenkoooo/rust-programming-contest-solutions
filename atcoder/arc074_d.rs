fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let h: usize = sc.read();
    let w: usize = sc.read();

    let board = (0..h).map(|_| sc.chars()).collect::<Vec<_>>();
    let mut dinitz = dinitz::Dinitz::new(h + w + 2);
    let source = h + w;
    let sink = source + 1;
    let inf = 1 << 50;
    for i in 0..h {
        for j in 0..w {
            match board[i][j] {
                'S' => {
                    dinitz.add_edge(source, i, inf);
                    dinitz.add_edge(source, j + h, inf);
                }
                'T' => {
                    dinitz.add_edge(i, sink, inf);
                    dinitz.add_edge(j + h, sink, inf);
                }
                'o' => {
                    dinitz.add_edge(i, j + h, 1);
                    dinitz.add_edge(j + h, i, 1);
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }

    let flow = dinitz.max_flow(source, sink);
    if flow >= inf {
        println!("-1");
    } else {
        println!("{}", flow);
    }
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

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
