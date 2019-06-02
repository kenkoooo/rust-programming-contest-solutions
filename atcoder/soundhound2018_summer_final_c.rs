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
    input!(n: usize, d: usize);
    let comb = Combination::new(10000, MOD);
    let mut pow2 = vec![ModInt::new(1); 10000];
    for i in 1..pow2.len() {
        pow2[i] = pow2[i - 1] * 2;
    }

    let mut dp = vec![vec![vec![ModInt::new(0); n + 1]; n + 1]; n + 1];
    dp[0][1][1] = ModInt::new(1);

    for dist in 1..n {
        for used in 1..n {
            for last in 1..(n + 1) {
                for next in 1..(n + 1) {
                    if used + next > n {
                        continue;
                    }
                    if used < last {
                        continue;
                    }

                    let selectable = if dist <= d { n - used - 1 } else { n - used };
                    let selecting = if dist == d { next - 1 } else { next };
                    if selectable < selecting {
                        continue;
                    }

                    let c = comb.get(selectable, selecting);

                    let available_edges = next * (next - 1) / 2;
                    let in_next = pow2[available_edges];

                    let from_last = pow2[last] - 1;
                    let last_to_next = pow(from_last, next);

                    dp[dist][used + next][next] +=
                        dp[dist - 1][used][last] * c * in_next * last_to_next;
                }
            }
        }
    }

    let mut ans = ModInt::new(0);
    for dist in d..n {
        for used in 1..(n + 1) {
            for last in 1..(used + 1) {
                let free_vertices = n - used;
                if free_vertices == 0 {
                    ans += dp[dist][used][last];
                } else {
                    let available_edges = free_vertices * (free_vertices - 1) / 2;
                    ans += dp[dist][used][last] * pow2[available_edges];
                }
            }
        }
    }
    println!("{}", ans.value);
}

fn pow(x: ModInt<usize>, mut e: usize) -> ModInt<usize> {
    let mut cur = x;
    let mut result = ModInt::new(1);
    while e > 0 {
        if e & 1 == 1 {
            result *= cur;
        }
        e >>= 1;
        cur = cur * cur;
    }
    result
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
        assert!(x >= y, "{}<{}", x, y);
        let t = self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo;
        ModInt::new(t)
    }
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
    pub fn new(v: usize) -> Self {
        ModInt {
            value: v,
            modulo: MOD,
        }
    }
}
