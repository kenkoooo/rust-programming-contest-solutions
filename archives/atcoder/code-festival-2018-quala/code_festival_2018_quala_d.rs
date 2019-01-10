use mod_int::ModInt;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let d = sc.read();
    let f: usize = sc.read();
    let t: usize = sc.read();
    let n = sc.read();
    let mut x: Vec<usize> = sc.read_vec(n);

    x.insert(0, 0);
    x.push(d);
    let n = x.len();

    let mut dp = vec![ModInt(0); n];
    let mut sum = vec![ModInt(0); n + 1];
    let mut pow = vec![ModInt(1)];
    for i in 0..n {
        let t = pow[i] * 2;
        pow.push(t);
    }
    dp[0] = ModInt(1);

    for i in 0..n {
        if i > 0 {
            sum[i] += sum[i - 1];
            dp[i] = sum[i];
        }

        let from = x.upper_bound(x[i] + f - t);
        let to = x.upper_bound(x[i] + f);
        let skip = from - 1 - i;

        let c = dp[i] * pow[skip];
        if from < sum.len() {
            sum[from] += c;
        }
        if to < sum.len() {
            sum[to] -= c;
        }
    }

    let mut ans = ModInt(0);
    for i in 0..(n - 1) {
        if x[n - 1] - x[i] <= f - t {
            let skip = (n - 1) - 1 - i;
            ans += dp[i] * pow[skip];
        }
    }

    ans += dp[n - 1];
    println!("{}", ans.0);
}

trait UpperBound<T> {
    fn upper_bound(&self, x: T) -> usize;
}

impl UpperBound<usize> for Vec<usize> {
    fn upper_bound(&self, x: usize) -> usize {
        self.binary_search_by_key(&(x * 2 + 1), |&v| v * 2)
            .err()
            .unwrap()
    }
}

pub mod mod_int {
    use super::MOD;
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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
        pub fn new(value: Num) -> Self {
            ModInt(value)
        }

        pub fn pow(self, e: usize) -> ModInt<Num> {
            let mut result = ModInt::new(1);
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
