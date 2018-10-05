/// Thank you tanakh!!!
/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{AddAssign, Sub};

fn main() {
    input!(n: usize, a: [i64; n]);

    let mut ok = 0;
    let mut ng = 1e10 as i64;
    while ng - ok > 1 {
        let x = (ng + ok) / 2;
        let a = a
            .iter()
            .map(|&a| if a < x { -1 } else { 1 })
            .collect::<Vec<_>>();
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + a[i];
        }
        let mut set = BTreeSet::new();
        for i in 0..(n + 1) {
            set.insert(sum[i]);
        }
        let mut map = BTreeMap::new();
        for (i, &x) in set.iter().enumerate() {
            map.insert(x, i);
        }
        let mut compact = vec![0; n + 1];
        for i in 0..(n + 1) {
            compact[i] = *map.get(&sum[i]).unwrap();
        }

        let m = map.len();
        let mut ans = 0;
        let mut bit = FenwickTree::new(m, 0);
        for i in 0..(n + 1) {
            ans += bit.sum_one(compact[i] + 1);
            bit.add(compact[i], 1);
        }

        let total = n * (n - 1) / 2 + n;
        let half = (total + 1) / 2;
        if ans >= half {
            ok = x;
        } else {
            ng = x;
        }
    }
    println!("{}", ok);
}

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
