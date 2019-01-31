use std::cmp;

const INF: i64 = std::i64::MAX / 2;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let d: i64 = sc.read();
    let p: Vec<i64> = sc.read_vec(n);
    let mut a = p
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .collect::<Vec<_>>();
    a.sort();
    let mut seg_l = SegmentTree::new(n, (INF, 0), |a, b| cmp::min(a, b));
    let mut seg_r = SegmentTree::new(n, (INF, 0), |a, b| cmp::min(a, b));
    let mut edges = vec![];
    for &(a, i) in a.iter() {
        let (left_a, left_i) = seg_l.query(0, i + 1);
        if left_a < INF {
            edges.push((left_i, i));
        }
        let (right_a, right_i) = seg_r.query(i, n);
        if right_a < INF {
            edges.push((i, right_i));
        }
        seg_l.update(i, ((n - i) as i64 * d + a, i));
        seg_r.update(i, (i as i64 * d + a, i));
    }

    let mut edges = edges
        .iter()
        .map(|&(i, j)| {
            assert!(i < j);
            let cost = (j - i) as i64 * d + p[i] + p[j];
            (cost, i, j)
        })
        .collect::<Vec<_>>();
    edges.sort();

    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for &(cost, i, j) in edges.iter() {
        if uf.find(i) != uf.find(j) {
            uf.unite(i, j);
            ans += cost;
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

/// Segment Tree for range queries
pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Copy, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f: f,
            initial_value: initial_value,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> T {
        assert!(a < b);
        self.query_range(a, b, 0, 0, self.n)
    }

    fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            self.initial_value
        } else if a <= l && r <= b {
            self.seg[k]
        } else {
            let x = self.query_range(a, b, k * 2 + 1, l, (l + r) >> 1);
            let y = self.query_range(a, b, k * 2 + 2, (l + r) >> 1, r);
            (self.f)(x, y)
        }
    }
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
