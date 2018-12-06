use std::cmp;

const INF: usize = 1e15 as usize;

fn min_cost(x: &Vec<usize>, y: &Vec<usize>) -> usize {
    assert_eq!(x.len(), y.len());
    let h = x.len();
    let mut cost = vec![vec![0; h + 1]; h + 1];
    for left in 0..h {
        let mut count = 0;
        for i in 0..h {
            if h - 1 < i + left {
                continue;
            }
            let left_piece = x[h - 1 - i - left];
            let right_pirce = y[h - 1 - i];
            if left_piece == right_pirce {
                count += 1;
            }
        }

        cost[left][0] = count;
        for right in 0..h {
            if h - 1 < left + right {
                continue;
            }
            if x[h - 1 - left - right] == y[h - 1 - right] {
                count -= 1;
            }
            cost[left + right + 1][right + 1] = count;
        }
    }
    for right in 0..h {
        let mut count = 0;
        for i in 0..h {
            if h - 1 < i + right {
                continue;
            }
            let left_piece = x[h - 1 - i];
            let right_pirce = y[h - 1 - i - right];
            if left_piece == right_pirce {
                count += 1;
            }
        }

        cost[0][right] = count;
        for left in 0..h {
            if h - 1 < left + right {
                continue;
            }
            if x[h - 1 - left] == y[h - 1 - left - right] {
                count -= 1;
            }
            cost[left + 1][left + right + 1] = count;
        }
    }

    let mut dp = vec![vec![INF; h + 1]; h + 1];
    dp[0][0] = 0;
    for left in 0..(h + 1) {
        for right in 0..(h + 1) {
            let next_cost = dp[left][right] + cost[left][right];
            if left + 1 <= h {
                dp[left + 1][right] = cmp::min(dp[left + 1][right], next_cost);
            }
            if right + 1 <= h {
                dp[left][right + 1] = cmp::min(dp[left][right + 1], next_cost);
            }
        }
    }

    dp[h][h]
}

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let h: usize = sc.read();
    let w: usize = sc.read();
    let c: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();

    let mut rotated = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            rotated[j][i] = c[i][j] as usize - ('a' as usize);
        }
    }

    let ans: usize = (1..w).map(|i| min_cost(&rotated[i - 1], &rotated[i])).sum();
    println!("{}", ans);
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
