#[derive(PartialEq)]
struct F64(f64);

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 > other.0 {
            std::cmp::Ordering::Greater
        } else if self.0 < other.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl Eq for F64 {}

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let mut a: Vec<f64> = vec![];
    let mut b: Vec<f64> = vec![];
    let mut c: Vec<f64> = vec![];
    for _ in 0..n {
        a.push(sc.read());
        b.push(sc.read());
        c.push(sc.read());
    }

    let x = solve(&a, &b, &c);
    let y = solve(&b, &a, &c);
    println!("{} {}", x, y);
}

fn solve(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    let n = a.len();
    let points = n * (n - 1) / 2;
    let num = (points + 1) / 2;
    let mut low = -1e15;
    let mut high = 1e15;
    let mut p_low = low;
    let mut p_high = high;

    let mut lines = (0..n)
        .map(|i| (F64((c[i] - a[i] * low) / b[i]), i))
        .collect::<Vec<_>>();
    lines.sort();

    loop {
        let x = (high + low) / 2.0;

        let mut lines = lines
            .iter()
            .map(|&(_, i)| (a[i], b[i], c[i]))
            .enumerate()
            .map(|(i, (a, b, c))| (F64((c - a * x) / b), i))
            .collect::<Vec<_>>();
        lines.sort();

        let mut bit = fenwick_tree::FenwickTree::new(n, 0);
        let mut ans = 0;
        for (_, i) in lines.into_iter().rev() {
            ans += bit.sum_one(i);
            bit.add(i, 1);
        }

        if ans >= num {
            high = x;
        } else {
            low = x;
        }

        if high == p_high && low == p_low {
            break;
        }
        p_low = low;
        p_high = high;
    }
    (high + low) / 2.0
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
