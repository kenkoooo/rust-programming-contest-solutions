use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let a: Vec<i64> = sc.vec(n);

    let mut ok = 0;
    let mut ng = 1e10 as i64;
    while ng - ok > 1 {
        let x = (ng + ok) / 2;
        let a = a
            .iter()
            .map(|&a| if a >= x { 1 } else { -1 })
            .collect::<Vec<i64>>();
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + a[i];
        }

        let map = acc
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<BTreeMap<_, _>>();
        let acc = acc
            .into_iter()
            .map(|v| *map.get(&v).unwrap())
            .collect::<Vec<_>>();
        let m = map.len();
        let mut bit = fenwick_tree::FenwickTree::new(m, 0);
        let mut negative_segments = 0;
        for i in 0..(n + 1) {
            negative_segments += i - bit.sum_one(acc[i] + 1);
            bit.add(acc[i], 1);
        }

        //        eprintln!(
        //            "x={} a={:?} acc={:?} negative_segments={}",
        //            x, a, acc, negative_segments
        //        );

        if negative_segments * 2 > (n + 1) * n / 2 {
            ng = x;
        } else {
            ok = x;
        }
    }

    println!("{}", ok);
}
pub mod fenwick_tree {
    use std::ops::{AddAssign, Sub};
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
            assert!(k < self.n, "k={} n={}", k, self.n);

            let mut result = self.init;
            let mut x = k as i32 - 1;
            while x >= 0 {
                result += self.data[x as usize];
                x = (x & (x + 1)) - 1;
            }

            result
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
