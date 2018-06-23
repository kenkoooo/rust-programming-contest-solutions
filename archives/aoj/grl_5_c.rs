use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let mut graph = (0..n).map(|_| Vec::new()).collect::<Vec<_>>();
    for i in 0..n {
        let k = sc.read::<usize>();
        for _ in 0..k {
            let c: usize = sc.read();
            graph[i].push(c);
            graph[c].push(i);
        }
    }

    let lca = LowestCommonAncestor::new(&graph);

    let q: usize = sc.read();
    for _ in 0..q {
        let u: usize = sc.read();
        let v: usize = sc.read();
        println!("{}", lca.get_lca(u, v));
    }
}

struct LowestCommonAncestor {
    graph: Vec<Vec<usize>>,
    parent: Vec<Vec<i32>>,
    depth: Vec<usize>,
    root: usize,
    log_v: usize,
}

impl LowestCommonAncestor {
    fn new(graph: &Vec<Vec<usize>>) -> LowestCommonAncestor {
        let v = graph.len();
        let root = 0;
        let graph = graph.clone();
        let mut depth = vec![0; v];

        let mut log_v = 1;
        let mut i = 1;
        while i <= v {
            i *= 2;
            log_v += 1;
        }
        let mut parent: Vec<Vec<i32>> = vec![vec![0; v]; log_v];

        let mut depth_vis = vec![false; v];
        let mut stack = VecDeque::new();
        stack.push_front(root);
        parent[0][root] = -1;
        depth[root] = 0;
        depth_vis[root] = true;
        while !stack.is_empty() {
            let v = stack.pop_front().unwrap();
            stack.push_front(v);
            for u in &graph[v] {
                let u = *u;
                if depth_vis[u] {
                    continue;
                }
                parent[0][u] = v as i32;
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
            for u in 0..v {
                parent[k + 1][u] = if parent[k][u] < 0 {
                    -1
                } else {
                    parent[k][parent[k][u] as usize]
                };
            }
        }

        LowestCommonAncestor {
            graph: graph,
            parent: parent,
            depth: depth,
            root: root,
            log_v: log_v,
        }
    }

    fn get_lca(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] <= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        for k in 0..self.log_v {
            if ((self.depth[v] - self.depth[u]) & (1 << k)) != 0 {
                v = self.parent[k][v] as usize;
            }
        }
        if u == v {
            return u;
        }

        for k in (0..self.log_v).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u] as usize;
                v = self.parent[k][v] as usize;
            }
        }
        return self.parent[0][u] as usize;
    }

    fn get_dist(&self, u: usize, v: usize) -> usize {
        let lca = self.get_lca(u, v);
        self.depth[u] + self.depth[v] - self.depth[lca] * 2
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
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
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

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
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
