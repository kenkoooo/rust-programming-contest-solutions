use self::fenwick_tree::FenwickTree;

const INF: f64 = 1e15;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let abc: Vec<_> = (0..n).map(|_| (sc.read(), sc.read(), sc.read())).collect();
    let bac = abc.iter().map(|&(a, b, c)| (b, a, c)).collect();

    let x = solve(abc);
    let y = solve(bac);
    println!("{} {}", x, y);
}

fn solve(mut abc: Vec<(f64, f64, f64)>) -> f64 {
    let comparator = |(a1, b1, c1): (f64, f64, f64), (a2, b2, c2): (f64, f64, f64), x: f64| {
        let k1 = (c1 - a1 * x) / b1;
        let k2 = (c2 - a2 * x) / b2;
        if k1 > k2 {
            std::cmp::Ordering::Less
        } else if k1 < k2 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    };

    let n = abc.len();
    let total = n * (n - 1) / 2;
    abc.sort_by(|&(a1, b1, c1), &(a2, b2, c2)| comparator((a1, b1, c1), (a2, b2, c2), -INF));
    let mut abc: Vec<_> = abc
        .into_iter()
        .enumerate()
        .map(|(i, (a, b, c))| (a, b, c, i))
        .collect();

    let mut ok = INF;
    let mut ng = -INF;
    let mut p_ok = ok;
    let mut p_ng = ng;
    loop {
        let x = (ok + ng) / 2.0;
        abc.sort_by(|&(a1, b1, c1, _), &(a2, b2, c2, _)| comparator((a1, b1, c1), (a2, b2, c2), x));

        let mut ans: usize = 0;
        let mut bit = FenwickTree::new(n, 0);
        for i in 0..n {
            let (_, _, _, x) = abc[i];
            ans += bit.sum(x, n);
            bit.add(x, 1);
        }

        if ans * 2 >= total {
            ok = x;
        } else {
            ng = x;
        }

        if p_ok == ok && p_ng == ng {
            break;
        }
        p_ok = ok;
        p_ng = ng;
    }
    ok
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
