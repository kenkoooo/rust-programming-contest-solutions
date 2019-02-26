use std::cmp;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut lx = vec![vec![]; n + 1];
    for _ in 0..m {
        let l = sc.read::<usize>();
        let r: usize = sc.read();
        let x: usize = sc.read();
        lx[r].push((l, x));
    }

    let count = |j: usize, k: usize, left: usize| {
        if cmp::min(j, k) >= left {
            3
        } else if cmp::max(j, k) >= left {
            2
        } else {
            1
        }
    };

    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if dp[i][j][k] == 0 {
                    continue;
                }
                let next = cmp::max(cmp::max(i, j), k) + 1;
                if lx[next].iter().all(|&(left, x)| x == count(j, k, left)) {
                    dp[next][j][k] += dp[i][j][k];
                    dp[next][j][k] %= MOD;
                }
                if lx[next].iter().all(|&(left, x)| x == count(i, j, left)) {
                    dp[i][j][next] += dp[i][j][k];
                    dp[i][j][next] %= MOD;
                }
                if lx[next].iter().all(|&(left, x)| x == count(i, k, left)) {
                    dp[i][next][k] += dp[i][j][k];
                    dp[i][next][k] %= MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += dp[i][j][n];
            ans += dp[i][n][j];
            ans += dp[n][j][i];
            ans %= MOD;
        }
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
