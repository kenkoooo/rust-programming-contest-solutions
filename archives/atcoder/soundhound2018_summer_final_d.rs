use std::collections::BTreeSet;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let q = sc.read();

    let mut uf = UnionFind::new(n);
    let mut edges = vec![BTreeSet::new(); n];
    let mut lazy_unite = vec![Vec::new(); n];

    for _ in 0..q {
        let t = sc.usize_read();
        if t == 1 {
            let u = sc.usize_read() - 1;
            let v = sc.usize_read() - 1;
            edges[u].insert(v);
            edges[v].insert(u);
            if uf.find(u) != uf.find(v) {
                lazy_unite[uf.find(u)].push(uf.find(v));
                lazy_unite[uf.find(v)].push(uf.find(u));
            }
        } else if t == 2 {
            let u = sc.usize_read() - 1;
            let _ = sc.usize_read();
            complete_dfs(uf.find(u), &mut uf, &mut lazy_unite);
        } else {
            let u = sc.usize_read() - 1;
            let v = sc.usize_read() - 1;
            if uf.find(u) == uf.find(v) || edges[u].contains(&v) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn complete_dfs(v: usize, uf: &mut UnionFind, graph: &mut Vec<Vec<usize>>) {
    let mut tmp = vec![];
    for &u in graph[v].iter() {
        tmp.push(u);
        uf.unite(v, u);
    }
    graph[v].clear();
    for &u in tmp.iter() {
        complete_dfs(u, uf, graph);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        return true;
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
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

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
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
