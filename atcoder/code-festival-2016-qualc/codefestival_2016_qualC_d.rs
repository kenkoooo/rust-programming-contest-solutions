use std::cmp;
const INF: u32 = 1e9 as u32;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let h: usize = sc.read();
    let w: usize = sc.read();
    let board: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let rotated: Vec<Vec<char>> = (0..w)
        .map(|i| (0..h).map(|j| board[j][i]).collect())
        .collect();

    let mut ans = 0;
    for i in 1..w {
        ans += solve(&rotated[i - 1], &rotated[i]);
    }
    println!("{}", ans);
}

fn solve<T: PartialEq>(a: &[T], b: &[T]) -> u32 {
    let n = a.len();
    let mut cost = vec![vec![0; n + 1]; n + 1];
    for removed in 0..n {
        cost[0][removed] = (0..(n - removed))
            .filter(|&i| a[i + removed] == b[i])
            .count() as u32;
        cost[removed][0] = (0..(n - removed))
            .filter(|&i| a[i] == b[i + removed])
            .count() as u32;
    }

    for i in 0..n {
        for j in 0..n {
            cost[i + 1][j + 1] = cost[i][j] - if a[n - 1 - i] == b[n - 1 - j] { 1 } else { 0 };
        }
    }

    let mut dp = vec![vec![INF; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..(n + 1) {
        for j in 0..(n + 1) {
            if i + 1 <= n {
                dp[i + 1][j] = cmp::min(dp[i + 1][j], dp[i][j] + cost[i][j]);
            }
            if j + 1 <= n {
                dp[i][j + 1] = cmp::min(dp[i][j + 1], dp[i][j] + cost[i][j]);
            }
        }
    }
    dp[n][n]
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
