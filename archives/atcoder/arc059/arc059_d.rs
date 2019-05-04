const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let s: Vec<char> = sc.chars();
    let m = s.len();

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut next = vec![0; n + 1];
        for i in 0..(n + 1) {
            if i == 0 {
                next[0] += dp[i];
                next[0] %= MOD;
            } else {
                next[i - 1] += dp[i];
                next[i - 1] %= MOD;
            }
            if i < n {
                next[i + 1] += dp[i] * 2;
                next[i + 1] %= MOD;
            }
        }
        dp = next;
    }

    let inv2 = mod_pow(2, MOD - 2);
    let inv_pow = mod_pow(inv2, m as u64);
    println!("{}", (dp[m] * inv_pow) % MOD);
}

fn mod_pow(x: u64, mut e: u64) -> u64 {
    let mut result = 1;
    let mut cur = x;
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
