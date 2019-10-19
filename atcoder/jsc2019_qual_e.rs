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
        let a = sc.read::<u64>();
        edges.push((a, r, c + h));
    }
    edges.sort();

    let mut ans = 0;
    let mut uf = UnionFind::new(h + w);
    for (a, r, c) in edges.into_iter().rev() {
        let r = uf.find(r);
        let c = uf.find(c);
        if r == c {
            if uf.sizes[r] > uf.edges[r] {
                uf.edges[r] += 1;
                ans += a;
            }
        } else {
            if uf.edges[r] + uf.edges[c] + 1 <= uf.sizes[r] + uf.sizes[c] {
                uf.unite(r, c);
                ans += a;
            }
        }
    }

    println!("{}", ans);
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
    edges: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
            edges: vec![0; n],
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

        self.edges[large] += self.edges[small] + 1;
        self.edges[small] = 0;

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
