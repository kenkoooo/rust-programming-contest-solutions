use std::cmp;

const INF: u64 = std::u64::MAX;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let q: usize = sc.read();

    let mut edges = vec![];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c: u64 = sc.read();
        edges.push((c, a, b));
    }
    edges.sort();

    let mut min_size = 1;
    let mut uf = UnionFind::new(n);
    let mut head = 0;
    let mut size_cost = vec![INF; n + 1];
    size_cost[0] = 0;
    size_cost[1] = 0;

    while head < m {
        let (c, _, _) = edges[head];
        while head < m && edges[head].0 == c {
            let (_, a, b) = edges[head];
            uf.unite(a, b);
            head += 1;
        }

        while min_size <= n && uf.size_dist[min_size] == 0 {
            min_size += 1;
        }
        size_cost[min_size] = cmp::min(size_cost[min_size], c);
    }

    for i in (1..(n + 1)).rev() {
        if size_cost[i - 1] == INF {
            size_cost[i - 1] = size_cost[i];
        }
    }

    for _ in 0..q {
        let q: usize = sc.read();
        if q > n || size_cost[q] == INF {
            println!("trumpet");
        } else {
            println!("{}", size_cost[q]);
        }
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size_dist: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut size_dist = vec![0; n + 1];
        size_dist[1] = n;
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
            size_dist: size_dist,
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

        let small_size = self.sizes[small];
        let large_size = self.sizes[large];
        self.size_dist[small_size] -= 1;
        self.size_dist[large_size] -= 1;
        self.size_dist[small_size + large_size] += 1;

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
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
