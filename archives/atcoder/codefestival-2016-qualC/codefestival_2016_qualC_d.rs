use std::cmp;

const INF: u64 = 1e15 as u64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let c: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut map: Vec<Vec<char>> = vec![vec!['a'; h]; w];
    for i in 0..h {
        for j in 0..w {
            map[j][i] = c[i][j];
        }
    }

    let mut ans = 0;
    for i in 1..w {
        ans += solve(&map[i - 1], &map[i]);
    }
    println!("{}", ans);
}

fn solve<T: cmp::PartialEq>(a: &Vec<T>, b: &Vec<T>) -> u64 {
    let n = a.len();
    let mut cost = vec![vec![0; n + 1]; n + 1];
    for removed in 0..(n + 1) {
        let mut c = 0;
        for i in 0..(n - removed) {
            if a[i + removed] == b[i] {
                c += 1;
            }
        }
        cost[0][removed] = c;
    }
    for removed in 0..(n + 1) {
        let mut c = 0;
        for i in 0..(n - removed) {
            if a[i] == b[i + removed] {
                c += 1;
            }
        }
        cost[removed][0] = c;
    }

    for d in 0..n {
        for i in 0..n {
            if i + 1 + d > n { break; }
            cost[i + 1][i + 1 + d] = cost[i][i + d] - if a[n - 1 - i] == b[n - i - 1 - d] { 1 } else { 0 };
            cost[i + 1 + d][i + 1] = cost[i + d][i] - if a[n - 1 - d - i] == b[n - 1 - i] { 1 } else { 0 };
        }
    }


    let mut dp = vec![vec![INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..n {
            dp[i][j + 1] = cmp::min(dp[i][j + 1], dp[i][j] + cost[i][j]);
            dp[i + 1][j] = cmp::min(dp[i + 1][j], dp[i][j] + cost[i][j]);
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
