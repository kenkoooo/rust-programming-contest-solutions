use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let m = sc.read::<usize>();
    let edges = (0..m).map(|_| (sc.read::<usize>() - 1, sc.read::<usize>() - 1)).collect::<Vec<_>>();
    let q = sc.read::<usize>();

    let queries = (0..q).map(|_| (sc.read::<usize>() - 1, sc.read::<usize>() - 1, sc.read::<usize>())).collect::<Vec<_>>();
    let mut ans = (0..(m + 1)).map(|_| VecDeque::new()).collect::<Vec<_>>();
    let mut ok = vec![m; q];
    let mut ng = vec![0; q];
    for i in 0..q {
        let mid = (ok[i] + ng[i]) >> 1;
        ans[mid].push_back(i);
    }

    for _ in 0..50 {
        let mut uf = UnionFind::new(n);
        for score in 0..m {
            let (from, to) = edges[score];
            uf.unite(from, to);
            let size = ans[score + 1].len();
            for _ in 0..size {
                let i = ans[score + 1].pop_front().unwrap();
                let (x, y, z) = queries[i];
                let count = if uf.same(x, y) {
                    uf.partial_size(x)
                } else {
                    uf.partial_size(x) + uf.partial_size(y)
                };
                if count >= z {
                    ok[i] = score + 1;
                } else {
                    ng[i] = score + 1;
                }
                let mid = (ok[i] + ng[i]) >> 1;
                ans[mid].push_back(i);
            }
        }
    }
    for score in &ok {
        println!("{}", score);
    }
}

struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| { i }).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
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

    fn partial_size(&mut self, x: usize) -> usize {
        let p = self.find(x);
        self.sizes[p]
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

