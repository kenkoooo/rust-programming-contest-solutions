use mod_int::ModInt;
use std::collections::BTreeMap;

const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let count: usize = sc.read();

    let mut ans = ModInt(3).pow(count);
    let c = Combination::new(count + 1, MOD);
    for k in 0..(count / 2) {
        ans -= c.get(count, k) * ModInt(2).pow(k) * 2;
    }
    println!("{}", ans.0);
}

pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Combination {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];
        inv[1] = 1;
        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Combination {
            fact: fact,
            inv_fact: inv_fact,
            modulo: modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> ModInt<usize> {
        assert!(x >= y);
        ModInt(self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo)
    }
}

pub mod mod_int {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;
    use super::MOD;

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
