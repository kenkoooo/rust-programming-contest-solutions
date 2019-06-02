const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut constraint = vec![vec![]; n + 1];
    for _ in 0..m {
        let l: usize = sc.read();
        let r: usize = sc.read();
        let x: usize = sc.read();
        constraint[r].push((l, x));
    }

    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for pos in 1..(n + 1) {
        for last_x in 0..pos {
            for last_y in 0..pos {
                for &(last_r, last_g, last_b) in [
                    (last_x, last_y, pos - 1),
                    (last_x, pos - 1, last_y),
                    (pos - 1, last_x, last_y),
                ]
                .into_iter()
                {
                    if dp[last_r][last_g][last_b] == 0 {
                        continue;
                    }
                    if last_r > 0 && (last_r == last_g || last_r == last_b) {
                        continue;
                    }
                    if last_g > 0 && last_g == last_b {
                        continue;
                    }
                    for &(r, g, b) in [
                        (last_r, last_g, pos),
                        (last_r, pos, last_b),
                        (pos, last_g, last_b),
                    ]
                    .into_iter()
                    {
                        let mut ok = true;
                        for &(left, x) in constraint[pos].iter() {
                            let count = if left <= r { 1 } else { 0 }
                                + if left <= g { 1 } else { 0 }
                                + if left <= b { 1 } else { 0 };
                            if count != x {
                                ok = false;
                                break;
                            }
                        }

                        if ok {
                            dp[r][g][b] += dp[last_r][last_g][last_b];
                            dp[r][g][b] %= MOD;
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += dp[i][j][n];
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
