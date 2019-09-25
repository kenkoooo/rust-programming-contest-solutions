fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let h: usize = sc.read();
    let w: usize = sc.read();

    let mut uf = UnionFind::new(h + w);
    let mut edges = vec![];
    for _ in 0..n {
        let row = sc.read::<usize>() - 1;
        let col = sc.read::<usize>() - 1;
        let value: u64 = sc.read();
        edges.push((value, row, col + h));
    }
    edges.sort();

    let mut ans = 0;
    for (value, row, col) in edges.into_iter().rev() {
        let x = uf.find(row);
        let y = uf.find(col);

        if x != y {
            if uf.sizes[x] + uf.sizes[y] >= uf.edge_count[x] + uf.edge_count[y] + 1 {
                uf.unite(x, y);
                ans += value;
            }
        } else if uf.sizes[x] > uf.edge_count[x] {
            uf.edge_count[x] += 1;
            ans += value;
        }
    }

    println!("{}", ans);
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
    edge_count: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
            edge_count: vec![0; n],
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
