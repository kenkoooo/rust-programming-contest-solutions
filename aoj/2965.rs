fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let k: usize = sc.read();
    let modulo: usize = sc.read();

    let mut dp = vec![vec![0; 1 << k]; 2 * n + 1];
    dp[0][0] = 1;
    for i in 0..(2 * n) {
        for mask in 0..(1 << k) {
            if mask == (1 << k) - 1 {
                continue;
            }
            if mask & (1 << (k - 1)) == 0 {
                let next = (mask << 1) | 1;
                dp[i + 1][next] += dp[i][mask];
                dp[i + 1][next] %= modulo;
            }
            if mask > 0 {
                let next = (mask - mask.highest_one_bit()) << 1;
                dp[i + 1][next] += dp[i][mask];
                dp[i + 1][next] %= modulo;
            }
        }
    }

    let mut ans = dp[2 * n][0];
    if n == k {
        ans += 1;
    }
    println!("{}", ans % modulo);
}

trait HighestOneBit {
    fn highest_one_bit(self) -> usize;
}

impl HighestOneBit for usize {
    fn highest_one_bit(self) -> usize {
        (self + 1).next_power_of_two() >> 1
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
