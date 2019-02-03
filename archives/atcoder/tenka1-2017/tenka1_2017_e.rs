use self::fenwick_tree::FenwickTree;

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
    let mut sc = Scanner { reader: s.lock() };
    let n = sc.read();
    let lines = (0..n)
        .map(|_| (sc.read(), sc.read(), sc.read()))
        .collect::<Vec<(f64, f64, f64)>>();
    let lines_y = lines.iter().map(|&(a, b, c)| (b, a, c)).collect();
    let x = solve(lines);
    let y = solve(lines_y);
    println!("{} {}", x, y);
}

fn solve(mut lines: Vec<(f64, f64, f64)>) -> f64 {
    let n = lines.len();
    let center = (n * (n - 1) / 2 + 1) / 2;

    let mut low = -1e9;
    lines.sort_by_key(|&(a, b, c)| F64((-a * low + c) / b));
    let mut high = 1e9;
    let mut prev_low = low;
    let mut prev_high = high;
    loop {
        let x = (low + high) / 2.0;
        let mut indices = (0..n)
            .map(|i| {
                let (a, b, c) = lines[i];
                let y = (-a * x + c) / b;
                (F64(y), i)
            })
            .collect::<Vec<_>>();
        indices.sort();

        let mut bit = FenwickTree::new(n, 0);
        let mut rotation = 0;
        for &(_, i) in indices.iter() {
            rotation += bit.sum(i, n);
            bit.add(i, 1);
        }

        if rotation >= center {
            high = x;
        } else {
            low = x;
        }

        if low == prev_low && high == prev_high {
            break;
        }
        prev_high = high;
        prev_low = low;
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
