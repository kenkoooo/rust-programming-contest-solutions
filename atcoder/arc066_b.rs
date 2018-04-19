use std::collections::BTreeMap;

const MOD: usize = 1_000_000_007;

fn main() {
    let mut sc = Scanner::new();
    let n: u64 = sc.read();

    let mut dp = BTreeMap::new();
    println!("{}", rec(n, n, &mut dp));
}

/// calculate the number of combinations of (a, b) (a<=b),
/// which are satisfied a+b<=sum and a^b<=xor
fn rec(sum: u64, xor: u64, dp: &mut BTreeMap<(u64, u64), usize>) -> usize {
    if sum == 0 {
        return 1;
    }

    if dp.contains_key(&(sum, xor)) {
        return dp[&(sum, xor)];
    }

    let a1 = rec(sum >> 1, xor >> 1, dp); // both are even
    let a2 = rec((sum - 1) >> 1, (xor - 1) >> 1, dp); // one is odd and another is even
    let a3 = if sum >= 2 {
        // both are odd
        rec((sum - 2) >> 1, xor >> 1, dp)
    } else {
        0
    };

    let ans = (a1 + a2 + a3) % MOD;
    dp.insert((sum, xor), ans);
    return ans;
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

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
        let mut sum = std::io::stdin();
        self.length = sum.read(&mut self.buf).unwrap();
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
