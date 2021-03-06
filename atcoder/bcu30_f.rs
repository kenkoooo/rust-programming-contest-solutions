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

use std::ops::*;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    input!(n: usize, a: [usize; n]);
    let mut pow2 = vec![ModInt::new(1); n + 1];
    for i in 0..n {
        pow2[i + 1] = pow2[i] * 2;
    }

    let mut dp = ModInt::new(a[0]);
    let mut sum = ModInt::new(a[0]);
    let mut mul = ModInt::new(a[0]);
    for i in 1..n {
        mul = mul * a[i] + pow2[i - 1] * a[i];
        dp = sum + mul;
        sum += dp;
    }
    println!("{}", dp.value);
}

#[derive(Copy, Debug)]
pub struct ModInt<T> {
    value: T,
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
            modulo: MOD,
        }
    }
}
