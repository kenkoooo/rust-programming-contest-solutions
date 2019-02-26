use std::cmp;

const INF: usize = 1e16 as usize;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let d: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);

    let mut ai: Vec<(usize, usize)> = a.iter().enumerate().map(|(i, &a)| (a, i)).collect();
    ai.sort();

    let mut left_seg = SegmentTree::new(n, (INF, 0), |a, b| cmp::min(a, b));
    let mut right_seg = SegmentTree::new(n, (INF, 0), |a, b| cmp::min(a, b));

    let mut edges = vec![];
    for (_, i) in ai.into_iter() {
        if i > 0 {
            let (tmp, j) = left_seg.query(0, i);
            if tmp < INF {
                let cost = (i - j) * d + a[i] + a[j];
                edges.push((cost, i, j));
            }
        }
        if i + 1 < n {
            let (tmp, j) = right_seg.query(i + 1, n);
            if tmp < INF {
                let cost = (j - i) * d + a[i] + a[j];
                edges.push((cost, i, j));
            }
        }
        left_seg.update(i, (a[i] + (n - i) * d, i));
        right_seg.update(i, (a[i] + i * d, i));
    }
    edges.sort();

    let mut ans = 0;
    let mut uf = UnionFind::new(n);

    for (cost, i, j) in edges.into_iter() {
        if uf.find(i) == uf.find(j) {
            continue;
        }
        uf.unite(i, j);
        ans += cost;
    }

    println!("{}", ans);
}

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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
