use self::mod_int::ModInt;

const MOD: usize = 1_000_000_007;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let x: u64 = sc.read();
    let y: u64 = sc.read();
    let z: u64 = sc.read();
    let sum = (x + y + z) as usize;
    let mut dp = vec![ModInt(0); 1 << sum];
    dp[0] = ModInt(1);
    for _ in 0..n {
        let mut next = vec![ModInt(0); 1 << sum];
        for mask in 0..(1 << sum) {
            if dp[mask].0 == 0 {
                continue;
            }

            let sum = sum_of(mask as u64);
            for add in 1..11 {
                let mut next_mask = mask as u64;
                push_back(&mut next_mask, add);
                if contains_xyz(next_mask, x, y, z) {
                    continue;
                }

                let mut sum = sum + add;
                while sum > x + y + z {
                    let removed = pop_front(&mut next_mask);
                    sum -= removed;
                }
                next[next_mask as usize] += dp[mask];
            }
        }
        dp = next;
    }

    let mut ans = ModInt(0);
    for x in dp.iter() {
        ans += x.0;
    }
    let ans = ModInt(10).pow(n) - ans;
    println!("{}", ans.0);
}

fn contains_xyz(mut mask: u64, x: u64, y: u64, z: u64) -> bool {
    let xyz = vec![x, y, z];
    let mut i = 2;
    let mut cur = 0;
    while mask > 0 {
        let a = pop_back(&mut mask);
        if cur + a == xyz[i] {
            if i == 0 {
                return true;
            }
            i -= 1;
            cur = 0;
        } else if cur + a > xyz[i] {
            return false;
        } else {
            cur += a;
        }
    }
    false
}

fn sum_of(mut mask: u64) -> u64 {
    let mut sum = 0;
    while mask > 0 {
        sum += pop_back(&mut mask);
    }
    sum
}

fn push_back(mask: &mut u64, x: u64) {
    *mask = (*mask << x) | (1 << (x - 1));
}

fn pop_front(mask: &mut u64) -> u64 {
    assert!(*mask > 0);
    if mask.count_ones() == 1 {
        let res = *mask;
        *mask = 0;
        res.trailing_zeros() as u64 + 1
    } else {
        let x = (*mask + 1).next_power_of_two() >> 1;
        *mask -= x;
        let y = (*mask + 1).next_power_of_two() >> 1;
        (x.trailing_zeros() - y.trailing_zeros()) as u64
    }
}

fn pop_back(mask: &mut u64) -> u64 {
    assert!(*mask > 0);
    let n = mask.trailing_zeros() as u64;
    *mask >>= n + 1;
    n + 1
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
