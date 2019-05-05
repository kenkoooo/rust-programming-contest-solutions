const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let a: Vec<Vec<i64>> = (0..n).map(|_| sc.vec(m)).collect();
    let rank = f2_gauss_jordan(a);

    let mut pow2 = vec![1; n + m + 1];
    for i in 1..pow2.len() {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }

    let ans = pow2[n + m - 1] + MOD - pow2[n + m - rank - 1];
    println!("{}", ans % MOD);
}

fn f2_gauss_jordan(mut a: Vec<Vec<i64>>) -> usize {
    let n = a.len();
    let m = a[0].len();
    let mut rank = 0;
    let mut row = 0;
    for col in 0..m {
        match (row..n).filter(|&i| a[i][col] == 1).next() {
            Some(pivot) => {
                a.swap(row, pivot);
                rank += 1;
                for k in (row + 1)..n {
                    if a[k][col] == 1 {
                        for i in 0..m {
                            a[k][i] ^= a[row][i];
                        }
                    }
                }

                row += 1;
            }
            None => {}
        }

        if row >= n {
            break;
        }
    }
    rank
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
