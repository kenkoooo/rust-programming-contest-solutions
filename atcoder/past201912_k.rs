fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let mut root = None;
    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        let p = sc.read::<i64>();
        if p == -1 {
            root = Some(i);
        } else {
            let p = p as usize - 1;
            graph[p].push(i);
            graph[i].push(p);
        }
    }

    let root = root.unwrap();
    let lca = lca::LowestCommonAncestor::new(&graph, root);

    let q: usize = sc.read();
    for _ in 0..q {
        let a = sc.read::<usize>() - 1;
        let boss = sc.read::<usize>() - 1;
        if lca.get_lca(a, boss) == boss {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

mod lca {
    use std::collections::VecDeque;

    const MAX_PARENT: usize = 1 << 50;
    pub struct LowestCommonAncestor {
        parent: Vec<Vec<usize>>,
        depth: Vec<usize>,
        log_v: usize,
    }

    impl LowestCommonAncestor {
        pub fn new(graph: &Vec<Vec<usize>>, root: usize) -> Self {
            let num_v = graph.len();
            let mut depth = vec![0; num_v];

            let mut log_v = 1;
            let mut i = 1;
            while i <= num_v {
                i *= 2;
                log_v += 1;
            }
            let mut parent: Vec<Vec<usize>> = vec![vec![0; num_v]; log_v];

            let mut depth_vis = vec![false; num_v];
            let mut stack = VecDeque::new();
            stack.push_front(root);
            parent[0][root] = MAX_PARENT;
            depth[root] = 0;
            depth_vis[root] = true;
            while !stack.is_empty() {
                let v = stack.pop_front().unwrap();
                stack.push_front(v);
                for &u in &graph[v] {
                    if depth_vis[u] {
                        continue;
                    }
                    parent[0][u] = v;
                    depth[u] = depth[v] + 1;
                    depth_vis[u] = true;
                    stack.push_front(u);
                }

                let head = stack.pop_front().unwrap();
                if head != v {
                    stack.push_front(head);
                }
            }

            for k in 0..(log_v - 1) {
                for u in 0..num_v {
                    parent[k + 1][u] = if parent[k][u] == MAX_PARENT {
                        MAX_PARENT
                    } else {
                        parent[k][parent[k][u]]
                    };
                }
            }

            LowestCommonAncestor {
                parent: parent,
                depth: depth,
                log_v: log_v,
            }
        }

        pub fn get_lca(&self, u: usize, v: usize) -> usize {
            let (mut u, mut v) = if self.depth[u] <= self.depth[v] {
                (u, v)
            } else {
                (v, u)
            };
            for k in 0..self.log_v {
                if ((self.depth[v] - self.depth[u]) & (1 << k)) != 0 {
                    v = self.parent[k][v];
                }
            }
            if u == v {
                return u;
            }

            for k in (0..self.log_v).rev() {
                if self.parent[k][u] != self.parent[k][v] {
                    u = self.parent[k][u];
                    v = self.parent[k][v];
                }
            }
            return self.parent[0][u];
        }

        pub fn get_dist(&self, u: usize, v: usize) -> usize {
            let lca = self.get_lca(u, v);
            self.depth[u] + self.depth[v] - self.depth[lca] * 2
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
