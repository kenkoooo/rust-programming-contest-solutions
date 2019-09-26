const MOD: usize = 998244353;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let sum = a.iter().sum::<usize>();

    let mut dp = vec![0; sum + 1];
    dp[0] = 1;
    for &a in a.iter() {
        let mut next = vec![0; sum + 1];
        for r in 0..(sum + 1) {
            if dp[r] > 0 {
                next[a + r] += dp[r];
                next[a + r] %= MOD;
                next[r] += dp[r] * 2;
                next[r] %= MOD;
            }
        }
        dp = next;
    }

    let all = mod_pow(3, n);

    let mut ans = 0;
    for (r, count) in dp.into_iter().enumerate() {
        if r * 2 >= sum {
            ans += count;
            ans %= MOD;
        }
    }
    if sum % 2 == 0 {
        let mut dp = vec![0; sum + 1];
        dp[0] = 1;
        for &a in a.iter() {
            let mut next = vec![0; sum + 1];
            for r in 0..(sum + 1) {
                if dp[r] > 0 {
                    next[a + r] += dp[r];
                    next[a + r] %= MOD;
                    next[r] += dp[r];
                    next[r] %= MOD;
                }
            }
            dp = next;
        }
        let half = dp[sum / 2];

        println!("{}", (all + 3 * half + MOD - (3 * ans) % MOD) % MOD);
    } else {
        println!("{} ", (all + MOD - (3 * ans) % MOD) % MOD);
    }
}
// 1 1 1 2
// r r g b
// r r b g
// r g r b
// r b r g
// r g g b
// r b b g

fn mod_pow(x: usize, mut e: usize) -> usize {
    let mut result = 1;
    let mut cur = x;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * cur) % MOD;
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
