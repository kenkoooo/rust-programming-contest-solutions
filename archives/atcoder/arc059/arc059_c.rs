use self::mod_int::ModInt;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n = sc.read();
    let c: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let b: Vec<usize> = sc.read_vec(n);
    let b_max = *b.iter().max().unwrap();

    // pow[x][a] := x^a
    let mut pow = vec![vec![ModInt::new(0); c + 1]; b_max + 1];
    for i in 1..(b_max + 1) {
        pow[i][0] = ModInt::new(1);
        for j in 0..c {
            pow[i][j + 1] = pow[i][j] * i;
        }
    }

    // sum[x+1][a] := pow[0][a] + pow[1][a] + ... + pow[x][a]
    let mut sum = vec![vec![ModInt::new(0); c + 1]; b_max + 2];
    for a in 0..(c + 1) {
        for x in 0..(b_max + 1) {
            sum[x + 1][a] = sum[x][a] + pow[x][a];
        }
    }

    let mut dp = vec![ModInt::new(0); c + 1];
    dp[0] = ModInt::new(1);
    for i in 0..n {
        let mut next = vec![ModInt::new(0); c + 1];
        let from = a[i];
        let to = b[i];
        for add in 0..(c + 1) {
            for prev in 0..(c + 1) {
                if prev + add > c {
                    break;
                }
                let x = sum[to + 1][add] - sum[from][add];
                next[prev + add] += dp[prev] * x;
                // for x in from..(to + 1) {
                //     next[prev + add] += dp[prev] * pow[x][add];
                // }
            }
        }

        dp = next;
    }

    println!("{}", dp[c].0);
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
