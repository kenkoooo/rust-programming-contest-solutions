use self::fenwick_tree::FenwickTree;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let a: Vec<i64> = sc.vec(n);
    let mut ok = 0;
    let mut ng = 1e10 as i64;
    while ng - ok > 1 {
        let x = (ng + ok) / 2;
        let b = a
            .iter()
            .map(|&a| if a >= x { 1 } else { -1 })
            .collect::<Vec<_>>();
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = acc[i] + b[i];
        }

        let map = acc
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .enumerate()
            .map(|(i, acc)| (acc, i))
            .collect::<BTreeMap<_, _>>();
        let indices = acc
            .iter()
            .map(|acc| *map.get(acc).unwrap())
            .collect::<Vec<_>>();

        let m = map.len();
        let mut bit = FenwickTree::new(m, 0);
        let mut bigger = 0;
        for i in 0..(n + 1) {
            let v = indices[i];
            bigger += bit.sum(v + 1, m);
            bit.add(v, 1);
        }

        let total = n * (n - 1) / 2 + n;
        if bigger * 2 > total {
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
