use self::mod_int::ModInt;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let mut sc = Scanner::new();
    let d: usize = sc.read();
    let f: usize = sc.read();
    let t: usize = sc.read();
    let n: usize = sc.read();
    let mut x: Vec<usize> = sc.read_vec(n);

    let mut pow2 = vec![ModInt::new(1); n + 1];
    for i in 0..n {
        pow2[i + 1] = pow2[i] * 2;
    }

    x.insert(0, 0);
    x.push(d);
    let n = x.len();

    let mut dp = vec![ModInt::new(0); n + 1];
    dp[0] = ModInt::new(1);
    let mut sum = vec![ModInt::new(0); n + 1];

    for i in 0..n {
        if i > 0 {
            sum[i] += sum[i - 1];
            dp[i] = sum[i];
        }

        let from = x.upper_bound(x[i] + f - t);
        let to = x.upper_bound(x[i] + f);

        let skip = from - 1 - i;
        let combinations = dp[i] * pow2[skip];

        if from < dp.len() {
            sum[from] += combinations;
        }

        if to < dp.len() {
            sum[to] -= combinations;
        }
    }

    let mut ans = ModInt::new(0);
    for i in 0..(n - 1) {
        if x[n - 1] - x[i] <= f - t {
            let skip = (n - 1) - 1 - i;
            ans += dp[i] * pow2[skip];
        }
    }

    ans += dp[n - 1];
    println!("{}", ans.value);
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}

trait VecBound {
    fn upper_bound(&self, x: usize) -> usize;
}

impl VecBound for Vec<usize> {
    fn upper_bound(&self, x: usize) -> usize {
        self.binary_search_by_key(&(x * 2 + 1), |&v| v * 2)
            .err()
            .unwrap()
    }
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
                modulo: super::MOD,
            }
        }
    }
}
