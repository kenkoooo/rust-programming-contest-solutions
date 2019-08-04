use mod_int::ModInt;
use std::collections::{BTreeMap, BTreeSet};

const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        x.push(sc.read::<i64>());
        y.push(sc.read::<i64>());
    }

    let ans = solve(&x, &y);
    println!("{}", ans.0);
}

fn solve(x: &Vec<i64>, y: &Vec<i64>) -> ModInt<usize> {
    let n = x.len();
    let x = shrink(x);
    let y = shrink(y);
    let mut xy = vec![];
    for i in 0..n {
        xy.push((x[i], y[i]));
    }
    xy.sort();

    let mut upper_left = vec![0; n];
    let mut upper_right = vec![0; n];
    let mut lower_left = vec![0; n];
    let mut lower_right = vec![0; n];

    let mut left_bit = fenwick_tree::FenwickTree::new(n, 0);
    for x in 0..n {
        let (_, y) = xy[x];
        let upper = left_bit.sum(y, n);
        let lower = left_bit.sum(0, y);
        upper_left[x] = upper;
        lower_left[x] = lower;
        left_bit.add(y, 1);
    }
    let mut right_bit = fenwick_tree::FenwickTree::new(n, 0);
    for x in (0..n).rev() {
        let (_, y) = xy[x];
        let upper = right_bit.sum(y, n);
        let lower = right_bit.sum(0, y);
        upper_right[x] = upper;
        lower_right[x] = lower;
        right_bit.add(y, 1);
    }

    let mut pow2 = vec![ModInt(0); n + 1];
    pow2[0] = ModInt(1);
    for i in 0..n {
        pow2[i + 1] = pow2[i] * 2;
    }

    let mut ans = ModInt(0);
    for i in 0..n {
        let cycle = [
            upper_left[i],
            upper_right[i],
            lower_right[i],
            lower_left[i],
            upper_left[i],
        ];
        for i in 0..4 {
            let a = cycle[i];
            let b = cycle[i + 1];
            ans += (pow2[a] - 1) * (pow2[b] - 1);
            ans += pow2[a] - 1;
        }
    }
    (pow2[n] - 1) * n - ans
}

pub mod mod_int {
    use super::MOD;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;

    #[derive(Clone, Copy)]
    pub struct ModInt<T: Copy + Clone>(pub T);

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.0
        }
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: Num) -> ModInt<Num> {
            let mut t = rhs + self.0;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt(t)
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.0 < rhs { self.0 + MOD } else { self.0 };
            ModInt(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.0
        }
    }

    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other;
        }
    }
    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other;
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other;
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other;
        }
    }

    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: Num) -> ModInt<Num> {
            self * ModInt(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.0
        }
    }

    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }
    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;

        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.0
        }
    }
    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;

        fn mul(self, rhs: Num) -> ModInt<Num> {
            let t = (self.0 * rhs) % MOD;
            ModInt(t)
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs;
        }
    }

    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs;
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: usize) -> ModInt<Num> {
            let mut result = ModInt(1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }
    }
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
fn shrink(x: &Vec<i64>) -> Vec<usize> {
    let map = x
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect::<BTreeMap<_, _>>();
    x.iter().cloned().map(|x| *map.get(&x).unwrap()).collect()
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
