const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc.chars();
    let n = s.len();
    let mut dp = vec![0; 13];
    dp[0] = 1;
    for i in 0..n {
        let mut next = vec![0; 13];
        let c = s[i];
        for r in 0..13 {
            if c == '?' {
                for c in 0..10 {
                    let next_r = (r * 10 + c) % 13;
                    next[next_r] += dp[r];
                    next[next_r] %= MOD;
                }
            } else {
                let c = c as usize - '0' as usize;
                let next_r = (r * 10 + c) % 13;
                next[next_r] += dp[r];
                next[next_r] %= MOD;
            }
        }
        dp = next;
    }
    println!("{}", dp[5]);
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
