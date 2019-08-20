const MOD: usize = 1e9 as usize + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let l: u64 = sc.read();
    let r: u64 = sc.read();
    let mut dp = vec![vec![vec![0; 2]; 2]; 2];
    dp[0][0][0] = 1;
    for i in (0..63).rev() {
        let l = (l >> i) & 1;
        let r = (r >> i) & 1;
        let mut next = vec![vec![vec![0; 2]; 2]; 2];
        for lower_r in 0..2 {
            for upper_l in 0..2 {
                for same_highest in 0..2 {
                    for (x, y) in vec![(0, 1), (0, 0), (1, 1)].into_iter() {
                        if lower_r == 0 && r < y {
                            continue;
                        }
                        if upper_l == 0 && l > x {
                            continue;
                        }
                        if same_highest == 0 && x != y {
                            continue;
                        }
                        let next_lower_r = if lower_r == 1 || r > y { 1 } else { 0 };
                        let next_upper_l = if upper_l == 1 || l < x { 1 } else { 0 };
                        let next_same_highest = if same_highest == 1 || x == 1 && y == 1 {
                            1
                        } else {
                            0
                        };
                        next[next_lower_r][next_upper_l][next_same_highest] +=
                            dp[lower_r][upper_l][same_highest];
                        next[next_lower_r][next_upper_l][next_same_highest] %= MOD;
                    }
                }
            }
        }
        dp = next;
    }

    let mut ans = 0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                ans += dp[i][j][k];
            }
        }
    }
    println!("{}", ans % MOD);
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
