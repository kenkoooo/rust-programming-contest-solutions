use std::ops::{AddAssign, Sub};

#[derive(Debug, Copy, Clone)]
struct F64(f64);

impl PartialEq for F64 {
    fn eq(&self, other: &F64) -> bool {
        self.0 == other.0
    }
}
impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &F64) -> std::cmp::Ordering {
        if self.0 < other.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &F64) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(lines: &Vec<(F64, F64, F64, F64)>) -> f64 {
    let n = lines.len();
    let count = ((n - 1) * n / 2 + 1) / 2;

    let mut ok = -1e15;
    let mut ng = 1e15;
    let mut prev_ok = ok;
    let mut prev_ng = ng;
    loop {
        let x = (ok + ng) / 2.0;

        let mut v = vec![];
        for (i, &(_, a, b, c)) in lines.iter().enumerate() {
            let y = (c.0 - a.0 * x) / b.0;
            v.push((F64(y), i));
        }
        v.sort();

        let mut sub_count = 0;
        let mut bit = FenwickTree::new(n, 0);
        for &(_, i) in v.iter() {
            sub_count += bit.sum(i, n);
            bit.add(i, 1);
        }

        if sub_count < count {
            ok = x;
        } else {
            ng = x;
        }

        if ok == prev_ok && ng == prev_ng {
            break;
        }
        prev_ok = ok;
        prev_ng = ng;
    }
    ok
}

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let mut lines_x = vec![];
    let mut lines_y = vec![];
    for _ in 0..n {
        let a = sc.read::<f64>();
        let b = sc.read::<f64>();
        let c = sc.read::<f64>();
        lines_x.push((F64(-a / b), F64(a), F64(b), F64(c)));
        lines_y.push((F64(-b / a), F64(b), F64(a), F64(c)));
    }
    lines_x.sort();
    lines_x.reverse();
    lines_y.sort();
    lines_y.reverse();

    let x = solve(&lines_x);
    let y = solve(&lines_y);
    println!("{} {}", x, y);
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
