fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let h: usize = sc.read();
    let w: usize = sc.read();
    let mut edges = vec![];
    for _ in 0..n {
        let r = sc.read::<usize>() - 1;
        let c = sc.read::<usize>() - 1;
        let a: usize = sc.read();
        edges.push((a, r, c + h));
    }

    edges.sort();
    edges.reverse();
    let mut ans = 0;
    let mut uf = UnionFind::new(h + w);
    for (v, r, c) in edges.into_iter() {
        let r = uf.find(r);
        let c = uf.find(c);
        if r == c {
            let edges = uf.edge_count[r];
            let size = uf.sizes[r];
            if edges + 1 <= size {
                uf.edge_count[r] += 1;
                ans += v;
            }
        } else {
            let edges = uf.edge_count[r] + uf.edge_count[c];
            let size = uf.sizes[r] + uf.sizes[c];
            if edges + 1 <= size {
                uf.unite(r, c);
                ans += v;
            }
        }
    }

    println!("{}", ans);
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    edge_count: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            edge_count: vec![0; n],
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

        self.edge_count[large] += self.edge_count[small] + 1;
        self.edge_count[small] = 0;

        self.size -= 1;
        return true;
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
