use std::cmp;

const MOD: usize = 1e9 as usize + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let m = sc.read();
    let mut constraints = vec![vec![]; n + 1];
    for _ in 0..m {
        let l = sc.read::<usize>();
        let r = sc.read::<usize>();
        let x: usize = sc.read();
        constraints[r].push((l, x));
    }

    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for r in 0..n {
        for g in 0..n {
            for b in 0..n {
                let max = max3(r, g, b);
                let next = vec![(max + 1, g, b), (r, max + 1, b), (r, g, max + 1)];
                for &(nr, ng, nb) in next.iter() {
                    let mut ok = true;
                    for &(left, x) in constraints[max + 1].iter() {
                        let mut count = 0;
                        if nr >= left {
                            count += 1;
                        }
                        if ng >= left {
                            count += 1;
                        }
                        if nb >= left {
                            count += 1;
                        }
                        if count != x {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        dp[nr][ng][nb] += dp[r][g][b];
                        dp[nr][ng][nb] %= MOD;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += dp[n][i][j];
            ans += dp[i][n][j];
            ans += dp[i][j][n];
            ans %= MOD;
        }
    }

    println!("{}", ans);
}

fn max3(x: usize, y: usize, z: usize) -> usize {
    cmp::max(cmp::max(x, y), z)
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
