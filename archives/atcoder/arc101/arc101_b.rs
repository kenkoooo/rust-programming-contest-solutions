use std::collections::{BTreeMap, BTreeSet};
use std::ops::{AddAssign, Sub};

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);

    let mut ng = 1e9 as usize;
    let mut ok = 0;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let a = a
            .iter()
            .map(|&a| if a >= mid { 1 } else { -1 })
            .collect::<Vec<i32>>();
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + a[i];
        }

        let sum_set = sum.iter().map(|&s| s).collect::<BTreeSet<_>>();
        let sum_map = sum_set
            .iter()
            .enumerate()
            .map(|(i, &s)| (s, i))
            .collect::<BTreeMap<i32, usize>>();
        let converted = sum.iter().map(|s| sum_map[s]).collect::<Vec<_>>();

        let mut bit = FenwickTree::new(sum_map.len(), 0);
        let mut ans = 0;
        for &c in converted.iter() {
            ans += bit.sum_one(c + 1);
            bit.add(c, 1);
        }

        let total = n * (n - 1) / 2 + n;
        let half = (total + 1) / 2;

        if ans >= half {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
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

/// `FenwickTree` is a data structure that can efficiently update elements
/// and calculate prefix sums in a table of numbers.
/// [https://en.wikipedia.org/wiki/Fenwick_tree](https://en.wikipedia.org/wiki/Fenwick_tree)
pub struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    init: T,
}

impl<T: Copy + AddAssign + Sub<Output = T>> FenwickTree<T> {
    /// Constructs a new `FenwickTree`. The size of `FenwickTree` should be specified by `size`.
    pub fn new(size: usize, init: T) -> FenwickTree<T> {
        FenwickTree {
            n: size + 1,
            data: vec![init; size + 1],
            init: init,
        }
    }

    pub fn add(&mut self, k: usize, value: T) {
        let mut x = k;
        while x < self.n {
            self.data[x] += value;
            x |= x + 1;
        }
    }

    /// Returns a sum of range `[l, r)`
    pub fn sum(&self, l: usize, r: usize) -> T {
        self.sum_one(r) - self.sum_one(l)
    }

    /// Returns a sum of range `[0, k)`
    pub fn sum_one(&self, k: usize) -> T {
        if k >= self.n {
            panic!("");
        }

        let mut result = self.init;
        let mut x = k as i32 - 1;
        while x >= 0 {
            result += self.data[x as usize];
            x = (x & (x + 1)) - 1;
        }

        result
    }
}
