use std::cmp;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let h: Vec<usize> = sc.vec(n);
    let a: Vec<u64> = sc.vec(n);

    let mut seg = SegmentTree::new(n + 1, 0, |a, b| cmp::max(a, b));
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        dp[h[i]] = cmp::max(dp[h[i]], seg.query(0, h[i] + 1) + a[i]);
        seg.update(h[i], dp[h[i]]);
    }
    println!("{}", dp.into_iter().max().unwrap());
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
