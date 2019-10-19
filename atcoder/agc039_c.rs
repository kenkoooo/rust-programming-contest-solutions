const MOD: usize = 998244353;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let x = sc
        .chars()
        .into_iter()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect::<Vec<_>>();

    let mut ans = vec![0; n * 2 + 1];

    for k in 2..(n * 2 + 1) {
        if (n * 2) % k != 0 || n % k == 0 {
            continue;
        }
        assert_eq!(k % 2, 0);
        let mut dp = [[0; 2]; 2];
        dp[0][0] = 1;
        for i in 0..(k / 2) {
            let x = x[i];
            let mut next = [[0; 2]; 2];
            for prev in 0..2 {
                for next_bit in 0..2 {
                    for below in 0..2 {
                        if below == 0 && next_bit > x {
                            continue;
                        }
                        let next_below = if next_bit < x { 1 } else { below };
                        next[next_below][next_bit] += dp[below][prev];
                        if next[next_below][next_bit] >= MOD {
                            next[next_below][next_bit] -= MOD;
                        }
                    }
                }
            }
            dp = next;
        }

        let below = (dp[1][0] + dp[1][1]) % MOD;
        let half = (dp[0][0] + dp[0][1]) % MOD;

        let mut ok = true;
        for i in 0..n {
            let t = i % (k / 2);
            let s = i % k;
            let y = if t == s { x[t] } else { x[t] ^ 1 };
            if x[i] < y {
                ok = false;
                break;
            }
            if x[i] > y {
                break;
            }
        }

        ans[k] = if ok { (below + half) % MOD } else { below };
    }

    for i in 1..(2 * n + 1) {
        let mut cur = 2 * i;
        while cur <= 2 * n {
            if ans[cur] > 0 {
                ans[cur] = (ans[cur] + MOD - ans[i]) % MOD;
            }
            cur += i;
        }
    }

    let ans = ans
        .into_iter()
        .enumerate()
        .fold(0, |ans, (i, v)| (ans + i * v) % MOD);
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
