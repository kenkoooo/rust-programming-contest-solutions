use std::collections::BTreeMap;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut x_map = BTreeMap::new();
    let mut y_map = BTreeMap::new();
    let mut vertices = Vec::new();
    for i in 0..n {
        let x: usize = sc.read();
        let y: usize = sc.read();
        vertices.push((x, y));
        if !x_map.contains_key(&x) {
            x_map.insert(x, Vec::new());
        }
        if !y_map.contains_key(&y) {
            y_map.insert(y, Vec::new());
        }
        x_map.get_mut(&x).unwrap().push(i);
        y_map.get_mut(&y).unwrap().push(i);
    }

    let mut uf = UnionFind::new(n);
    for v in x_map.values() {
        for i in 1..v.len() {
            uf.unite(v[0], v[i]);
        }
    }
    for v in y_map.values() {
        for i in 1..v.len() {
            uf.unite(v[0], v[i]);
        }
    }

    let mut edges = Vec::new();
    let x_keys: Vec<usize> = x_map.keys().map(|&i| i).collect();
    for i in 1..x_keys.len() {
        let len = x_keys[i] - x_keys[i - 1];
        edges.push((len, x_keys[i - 1], x_keys[i], true));
    }
    let y_keys: Vec<usize> = y_map.keys().map(|&i| i).collect();
    for i in 1..y_keys.len() {
        let len = y_keys[i] - y_keys[i - 1];
        edges.push((len, y_keys[i - 1], y_keys[i], false));
    }
    edges.sort();
    let mut ans = 0;
    for &(length, from, to, is_x) in &edges {
        let (from, to) = if is_x {
            let from = x_map[&from][0];
            let to = x_map[&to][0];
            (from, to)
        } else {
            let from = y_map[&from][0];
            let to = y_map[&to][0];
            (from, to)
        };
        if uf.find(from) != uf.find(to) {
            ans += length;
            uf.unite(from, to);
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
