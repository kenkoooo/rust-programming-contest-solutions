use std::collections::BTreeMap;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let p: Vec<usize> = (0..n).map(|_| sc.read::<usize>() - 1).collect();
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        let x = sc.read::<usize>() - 1;
        let y = sc.read::<usize>() - 1;
        uf.unite(x, y);
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        let x = uf.find(i);
        if !map.contains_key(&x) {
            map.insert(x, Vec::new());
        }
        map.get_mut(&x).unwrap().push(i);
    }

    let mut ans = 0;
    for v in map.values() {
        let mut check = Vec::new();
        for &i in v {
            let p = p[i];
            check.push(p);
            check.push(i);
        }
        check.sort();

        let mut pos = 0;
        while pos + 1 < check.len() {
            if check[pos] == check[pos + 1] {
                ans += 1;
                pos += 2;
            } else {
                pos += 1;
            }
        }
    }

    println!("{}", ans);
}

struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
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
