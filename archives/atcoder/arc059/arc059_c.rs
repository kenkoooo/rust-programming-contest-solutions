use self::mod_int::ModInt;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
    };

    let n: usize = sc.read();
    let c: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let b: Vec<usize> = sc.read_vec(n);

    let b_max: usize = *b.iter().max().unwrap();

    // pow[i][j] := i**j
    let mut pow = vec![vec![ModInt::new(0); c + 1]; b_max + 1];
    for i in 1..(b_max + 1) {
        pow[i][0] = ModInt::new(1);
        for j in 0..c {
            pow[i][j + 1] = pow[i][j] * i;
        }
    }

    // sum[i][c] := sum(pow[a[i]..b[i]+1][c])
    let mut sum = vec![vec![ModInt::new(0); c + 1]; n];
    for i in 0..n {
        let from = a[i];
        let to = b[i];
        for c in 0..(c + 1) {
            for t in from..(to + 1) {
                sum[i][c] += pow[t][c];
            }
        }
    }

    let mut dp = vec![ModInt::new(0); c + 1];
    dp[0] = ModInt::new(1);
    for i in 0..n {
        let mut next = vec![ModInt::new(0); c + 1];
        for from in 0..(c + 1) {
            for add in 0..(c + 1) {
                if from + add > c {
                    continue;
                }
                next[from + add] += dp[from] * sum[i][add];
            }
        }
        dp = next;
    }
    println!("{}", dp[c].value);
}

pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Rem, Sub, SubAssign};

    #[derive(Copy)]
    pub struct ModInt<T> {
        pub value: T,
        modulo: T,
    }

    impl<T> Clone for ModInt<T>
    where
        T: Copy,
    {
        fn clone(&self) -> Self {
            ModInt {
                value: self.value,
                modulo: self.modulo,
            }
        }

        fn clone_from(&mut self, source: &ModInt<T>) {
            self.value = source.value;
            self.modulo = source.modulo;
        }
    }

    impl<T> Add<ModInt<T>> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn add(self, rhs: ModInt<T>) -> ModInt<T> {
            self + rhs.value
        }
    }

    impl<T> Add<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        type Output = ModInt<T>;
        fn add(self, rhs: T) -> ModInt<T> {
            let m = self.modulo;
            let mut t = rhs + self.value;
            if t >= m {
                t = t - m;
            }
            ModInt {
                value: t,
                modulo: self.modulo,
            }
        }
    }

    impl<T> Sub<T> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: T) -> ModInt<T> {
            let rhs = if rhs >= self.modulo {
                rhs % self.modulo
            } else {
                rhs
            };
            let value = if self.value < rhs {
                self.value + self.modulo
            } else {
                self.value
            };
            ModInt {
                value: value - rhs,
                modulo: self.modulo,
            }
        }
    }

    impl<T> Sub<ModInt<T>> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        type Output = ModInt<T>;
        fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
            self - rhs.value
        }
    }

    impl<T> AddAssign<T> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<T> AddAssign<ModInt<T>> for ModInt<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    {
        fn add_assign(&mut self, other: ModInt<T>) {
            *self = *self + other;
        }
    }

    impl<T> SubAssign<T> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }

    impl<T> SubAssign<ModInt<T>> for ModInt<T>
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
    {
        fn sub_assign(&mut self, other: ModInt<T>) {
            *self = *self - other;
        }
    }

    impl<T> Mul<ModInt<T>> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        type Output = ModInt<T>;

        fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
            self * rhs.value
        }
    }
    impl<T> Mul<T> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        type Output = ModInt<T>;

        fn mul(self, rhs: T) -> ModInt<T> {
            let t = (self.value * rhs) % self.modulo;
            ModInt {
                value: t,
                modulo: self.modulo,
            }
        }
    }

    impl<T> MulAssign<T> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = *self * rhs;
        }
    }

    impl<T> MulAssign<ModInt<T>> for ModInt<T>
    where
        T: Mul<Output = T> + Rem<Output = T> + Copy,
    {
        fn mul_assign(&mut self, rhs: ModInt<T>) {
            *self = *self * rhs;
        }
    }

    impl ModInt<usize> {
        pub fn new(value: usize) -> Self {
            ModInt {
                value: value,
                modulo: self::super::MOD,
            }
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
