use std::cmp;

const INF: u64 = 1e9 as u64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let h: usize = sc.read();
    let w: usize = sc.read();
    let c: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();
    let mut d = vec![vec![' '; h]; w];
    for i in 0..h {
        for j in 0..w {
            d[j][i] = c[i][j];
        }
    }

    let mut ans = 0;
    for i in 1..w {
        ans += solve(&d[i - 1], &d[i]);
    }
    println!("{}", ans);
}

fn solve(a: &[char], b: &[char]) -> u64 {
    let h = a.len();
    let mut cost = vec![vec![0; h + 1]; h + 1];
    for i in 0..(h + 1) {
        cost[0][i] = 0;
        for j in 0..h {
            if i + j < h && a[i + j] == b[j] {
                cost[0][i] += 1;
            }
        }

        cost[i][0] = 0;
        for j in 0..h {
            if i + j < h && a[j] == b[i + j] {
                cost[i][0] += 1;
            }
        }
    }

    for i in 0..h {
        for j in 0..h {
            if i + j < h {
                cost[j + 1][i + j + 1] = cost[j][i + j]
                    - if a[h - 1 - j] == b[h - 1 - (i + j)] {
                        1
                    } else {
                        0
                    };
                cost[i + j + 1][j + 1] = cost[i + j][j]
                    - if a[h - 1 - (i + j)] == b[h - 1 - j] {
                        1
                    } else {
                        0
                    };
            }
        }
    }

    let mut dp = vec![vec![INF; h + 1]; h + 1];
    dp[0][0] = 0;
    for i in 0..(h + 1) {
        for j in 0..(h + 1) {
            if i + 1 <= h {
                dp[i + 1][j] = cmp::min(dp[i + 1][j], dp[i][j] + cost[i][j]);
            }
            if j + 1 <= h {
                dp[i][j + 1] = cmp::min(dp[i][j + 1], dp[i][j] + cost[i][j]);
            }
        }
    }
    dp[h][h]
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
