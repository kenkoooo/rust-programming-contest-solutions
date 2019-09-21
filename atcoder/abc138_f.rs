const MOD: usize = 1e9 as usize + 7;
const SIZE: usize = 64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let l: usize = sc.read();
    let r: usize = sc.read();

    // y%x == y^x
    // highest(x) == highest(y)
    //
    // dp[p][q][r]
    // p
    // x[bit], y[bit] = (0,0), (0,1), (1,1)

    let l = (0..SIZE).rev().map(|i| (l >> i) & 1).collect::<Vec<_>>();
    let r = (0..SIZE).rev().map(|i| (r >> i) & 1).collect::<Vec<_>>();

    let mut dp = vec![vec![vec![0; 2]; 2]; 2];
    dp[0][0][0] = 1;

    for bit in 0..SIZE {
        let mut next = vec![vec![vec![0; 2]; 2]; 2];
        for has_highest in 0..2 {
            for y_low_r in 0..2 {
                for x_up_l in 0..2 {
                    // (x, y) = (1, 1)
                    if y_low_r == 1 || r[bit] == 1 {
                        let next_x_up_l = if l[bit] == 0 { 1 } else { x_up_l };
                        next[1][y_low_r][next_x_up_l] += dp[has_highest][y_low_r][x_up_l];
                        next[1][y_low_r][next_x_up_l] %= MOD;
                    }

                    // (x, y) = (0, 1)
                    if has_highest == 1
                        && (y_low_r == 1 || r[bit] == 1)
                        && (x_up_l == 1 || l[bit] == 0)
                    {
                        next[1][y_low_r][x_up_l] += dp[has_highest][y_low_r][x_up_l];
                        next[1][y_low_r][x_up_l] %= MOD;
                    }

                    // (x, y) = (0, 0)
                    if x_up_l == 1 || l[bit] == 0 {
                        let next_y_low_r = if r[bit] == 1 { 1 } else { y_low_r };
                        next[has_highest][next_y_low_r][x_up_l] += dp[has_highest][y_low_r][x_up_l];
                        next[has_highest][next_y_low_r][x_up_l] %= MOD;
                    }
                }
            }
        }
        dp = next;
    }

    let mut sum = 0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                sum += dp[i][j][k];
            }
        }
    }
    println!("{}", sum % MOD);
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
