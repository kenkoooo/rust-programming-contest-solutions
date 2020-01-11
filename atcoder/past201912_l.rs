fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut pos = vec![];
    for _ in 0..(n + m) {
        let x: i64 = sc.read();
        let y: i64 = sc.read();
        let c: usize = sc.read();
        pos.push((x, y, c));
    }

    let mut edges = vec![];
    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            let (x1, y1, c1) = pos[i];
            let (x2, y2, c2) = pos[j];
            let dx = x1 - x2;
            let dy = y1 - y2;
            let cost = (dx * dx + dy * dy) * if c1 == c2 { 1 } else { 100 };
            edges.push((cost, i, j));
        }
    }
    edges.sort();

    let mut ans = std::f64::MAX;
    for mask in 0..(1 << m) {
        let mut sum = 0.0;
        let mut uf = UnionFind::new(n + m);
        for &(cost, i, j) in edges.iter() {
            if i >= n && mask & (1 << (i - n)) == 0 {
                continue;
            }
            if j >= n && mask & (1 << (j - n)) == 0 {
                continue;
            }
            if uf.find(i) != uf.find(j) {
                uf.unite(i, j);
                sum += (cost as f64).sqrt();
            }
        }

        if ans > sum {
            ans = sum;
        }
    }

    println!("{}", ans);
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
