use self::mod_int::ModInt;
use std::cmp;

const MOD: usize = 1e9 as usize + 7;

fn max(x: usize, y: usize, z: usize) -> usize {
    cmp::max(cmp::max(x, y), z)
}

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut constraints = vec![vec![]; n + 1];
    for _ in 0..m {
        let l: usize = sc.read();
        let r: usize = sc.read();
        let x: usize = sc.read();
        constraints[r].push((l, x));
    }

    let is_ok = |x: usize, y: usize, z: usize| -> bool {
        let max = max(x, y, z);
        for &(left, count) in constraints[max].iter() {
            let mut sum = 0;
            sum += if x >= left { 1 } else { 0 };
            sum += if y >= left { 1 } else { 0 };
            sum += if z >= left { 1 } else { 0 };
            if count != sum {
                return false;
            }
        }
        true
    };

    let mut dp = vec![vec![vec![ModInt::new(0); n + 1]; n + 1]; n + 1];
    dp[0][0][0] = ModInt::new(1);
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let max = max(x, y, z);
                let next = max + 1;
                if is_ok(next, y, z) {
                    dp[next][y][z] += dp[x][y][z];
                }
                if is_ok(x, next, z) {
                    dp[x][next][z] += dp[x][y][z];
                }
                if is_ok(x, y, next) {
                    dp[x][y][next] += dp[x][y][z];
                }
            }
        }
    }

    let mut ans = ModInt::new(0);
    for i in 0..n {
        for j in 0..n {
            ans += dp[n][i][j];
            ans += dp[i][n][j];
            ans += dp[i][j][n];
        }
    }

    println!("{}", ans.0);
}
pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;
    const MOD: Num = super::MOD;

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
