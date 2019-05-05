const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let x: usize = sc.read();
    let mut s: Vec<usize> = sc.vec(n);
    s.sort();
    s.reverse();

    let mut dp = vec![0; x + 1];
    dp[x] = 1;
    for i in 0..n {
        let mut next = vec![0; x + 1];
        for j in 0..(x + 1) {
            // unused
            next[j] += dp[j] * (n - 1 - i);
            next[j] %= MOD;

            // used
            next[j % s[i]] += dp[j];
            next[j % s[i]] %= MOD;
        }
        dp = next;
    }

    let mut ans = 0;
    for i in 0..s[n - 1] {
        ans += dp[i] * i;
        ans %= MOD;
    }
    println!("{}", ans);
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
