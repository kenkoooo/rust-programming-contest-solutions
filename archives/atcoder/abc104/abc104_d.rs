const MOD: usize = 1e9 as usize + 7;

fn mod_pow(x: usize, e: usize) -> usize {
    let mut cur = x;
    let mut result = 1;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result *= cur;
            result %= MOD;
        }
        cur = (cur * cur) % MOD;
        e >>= 1;
    }
    result
}

fn main() {
    let mut sc = Scanner::new();
    let s: Vec<char> = sc.read::<String>().chars().collect();
    let n = s.len();
    let mut dp = vec![vec![0; 4]; 4];
    dp[0][0] = 1;

    for i in 0..n {
        for used in (0..4).rev() {
            match s[i] {
                'A' => {
                    dp[used][1] += dp[used][0];
                    dp[used][1] %= MOD;
                }
                'B' => {
                    dp[used][2] += dp[used][1];
                    dp[used][2] %= MOD;
                }
                'C' => {
                    dp[used][3] += dp[used][2];
                    dp[used][3] %= MOD;
                }
                '?' => {
                    if used < 3 {
                        dp[used + 1][3] += dp[used][2];
                        dp[used + 1][3] %= MOD;

                        dp[used + 1][2] += dp[used][1];
                        dp[used + 1][2] %= MOD;

                        dp[used + 1][1] += dp[used][0];
                        dp[used + 1][1] %= MOD;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let q = s
        .iter()
        .map(|&c| if c == '?' { 1 } else { 0 })
        .sum::<usize>();

    let mut ans = 0;
    for used in 0..4 {
        if q >= used {
            ans += dp[used][3] * mod_pow(3, q - used);
            ans %= MOD;
        }
    }

    println!("{}", ans);
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

    fn usize_read(&mut self) -> usize {
        self.read()
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
