const MOD: u64 = 998244353;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let a_sum = a.iter().sum::<usize>();
    let mut dp = vec![0; a_sum + 1];
    dp[0] = 1;
    for i in 0..n {
        let a = a[i];
        for i in (0..(a_sum - a + 1)).rev() {
            dp[i + a] += dp[i];
            dp[i + a] %= MOD;
            dp[i] *= 2;
            dp[i] %= MOD;
        }
    }
    let mut dp2 = vec![0; a_sum + 1];
    dp2[0] = 1;
    for i in 0..n {
        let a = a[i];
        for i in (0..(a_sum - a + 1)).rev() {
            dp2[i + a] += dp2[i];
            dp2[i + a] %= MOD;
        }
    }

    let mut half = 0;
    for i in ((a_sum + 1) / 2)..(a_sum + 1) {
        half += dp[i];
        half %= MOD;
    }

    let all = mod_pow(3, n);
    if a_sum % 2 == 0 {
        let ans = (all + MOD - (half * 3) % MOD) % MOD;
        println!("{}", (ans + dp2[a_sum / 2] * 3) % MOD);
    } else {
        println!("{}", (all + MOD - (half * 3) % MOD) % MOD);
    }
}

fn mod_pow(mut x: u64, mut e: usize) -> u64 {
    let mut result = 1;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * x) % MOD;
        }
        e >>= 1;
        x = (x * x) % MOD;
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
