use std::cmp;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m = sc.read();

    let mut left_x = vec![vec![]; n + 1];
    for _ in 0..m {
        let left = sc.read::<usize>();
        let right = sc.read::<usize>();
        let x: usize = sc.read();
        left_x[right].push((left, x));
    }

    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for r in 0..n {
        for g in 0..n {
            for b in 0..n {
                let max = cmp::max(cmp::max(r, g), b);
                let check = |a: usize, b: usize| left_x[max + 1].iter().all(|&(left, x)| {
                    let mut count = 1;
                    if a >= left { count += 1; }
                    if b >= left { count += 1; }
                    count == x
                });

                if check(g, b) {
                    dp[max + 1][g][b] += dp[r][g][b];
                    dp[max + 1][g][b] %= MOD;
                }
                if check(r, g) {
                    dp[r][g][max + 1] += dp[r][g][b];
                    dp[r][g][max + 1] %= MOD;
                }
                if check(r, b) {
                    dp[r][max + 1][b] += dp[r][g][b];
                    dp[r][max + 1][b] %= MOD;
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
