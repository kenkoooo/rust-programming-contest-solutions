const MOD: usize = 998244353;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut a: Vec<Vec<i64>> = (0..n)
        .map(|_| (0..m).map(|_| sc.read()).collect())
        .collect();

    let mut rank = 0;
    for col in 0..m {
        match (rank..n).find(|&i| a[i][col] == 1) {
            Some(row) => {
                a.swap(rank, row);
                assert_eq!(a[rank][col], 1);
                for i in 0..n {
                    if i == rank || a[i][col] != 1 {
                        continue;
                    }
                    for j in 0..m {
                        a[i][j] ^= a[rank][j];
                    }
                }
                rank += 1;
            }
            None => {}
        }
    }

    let mut pow2 = vec![0; n + m + 1];
    pow2[0] = 1;
    for i in 1..pow2.len() {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }
    let ans = pow2[n + m - 1] + MOD - pow2[n + m - rank - 1];
    println!("{}", ans % MOD);

    // C(rank, x) * 2^(x-1) * 2^(m-x) * 2^(n-rank)
    // 2^(n+m-rank-1) * sum C(rank, x)
    // 2^(n+m-rank-1) * 2^rank - 2^(n+m-rank-1)
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
