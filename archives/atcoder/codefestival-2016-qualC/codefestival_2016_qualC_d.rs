use std::cmp;

const INF: usize = 1e15 as usize;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let h: usize = sc.read();
    let w: usize = sc.read();
    let c: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut map = vec![vec![0; h]; w];
    for i in 0..w {
        for j in 0..h {
            map[i][j] = c[j][i] as usize - 'a' as usize;
        }
    }

    let mut ans = 0;
    for i in 1..w {
        ans += calc(&map[i - 1], &map[i]);
    }
    println!("{}", ans);
}

fn calc(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let n = a.len();
    let mut cost_dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        let mut cost = 0;
        for j in 0..(n - i) {
            if a[j + i] == b[j] {
                cost += 1;
            }
        }
        cost_dp[0][i] = cost;
    }
    for i in 0..n {
        let mut cost = 0;
        for j in 0..(n - i) {
            if b[j + i] == a[j] {
                cost += 1;
            }
        }
        cost_dp[i][0] = cost;
    }
    for d in 0..n {
        for i in 0..n {
            if i + 1 + d > n { break; }
            cost_dp[i + 1][i + 1 + d] = cost_dp[i][i + d] - if a[n - 1 - i] == b[n - 1 - d - i] { 1 } else { 0 };
            cost_dp[i + 1 + d][i + 1] = cost_dp[i + d][i] - if a[n - 1 - d - i] == b[n - 1 - i] { 1 } else { 0 };
        }
    }

    let mut dp = vec![vec![INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..n {
            dp[i + 1][j] = cmp::min(dp[i][j] + cost_dp[i][j], dp[i + 1][j]);
            dp[i][j + 1] = cmp::min(dp[i][j] + cost_dp[i][j], dp[i][j + 1]);
        }
    }
    let mut ans = INF;
    for i in 0..n {
        ans = cmp::min(ans, dp[n][i]);
        ans = cmp::min(ans, dp[i][n]);
    }
    ans
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
