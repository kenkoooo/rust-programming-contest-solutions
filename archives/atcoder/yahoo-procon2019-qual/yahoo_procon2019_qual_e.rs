use self::mod_int::ModInt;

const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let a = (0..n).map(|_| (0..m).map(|_| sc.read()).collect()).collect();
    let rank = f2_gauss_jordan(a);
    let two = ModInt::new(2);
    let ans = (two.pow(n) - two.pow(n - rank)) * two.pow(m - 1);
    println!("{}", ans.0);
}

fn f2_gauss_jordan(mut a: Vec<Vec<usize>>) -> usize {
    let n = a.len();
    let m = a[0].len();
    let mut rank = 0;
    let mut row = 0;
    for col in 0..m {
        match (row..n).filter(|&i| a[i][col] == 1).next() {
            Some(pivot) => {
                a.swap(row, pivot);
                rank += 1;
                for k in (row + 1)..n {
                    if a[k][col] == 1 {
                        for j in 0..m {
                            a[k][j] ^= a[row][j];
                        }
                    }
                }

                row += 1;
            }
            None => {}
        }

        if row >= n {
            break;
        }
    }
    rank
}

pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

    use super::MOD;

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
