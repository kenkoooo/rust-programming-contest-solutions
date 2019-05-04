const MOD: usize = 1e9 as usize + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let c: usize = sc.read();
    let a: Vec<usize> = sc.vec(n);
    let b: Vec<usize> = sc.vec(n);
    let max_b: usize = *b.iter().max().unwrap();

    let mut pow = vec![vec![0; c + 1]; max_b + 1];
    for x in 0..(max_b + 1) {
        pow[x][0] = 1;
        for i in 0..c {
            pow[x][i + 1] = (pow[x][i] * x) % MOD;
        }
    }

    let mut sum_pow = vec![vec![0; c + 1]; max_b + 2];
    for i in 0..(c + 1) {
        for x in 0..(max_b + 1) {
            sum_pow[x + 1][i] = (sum_pow[x][i] + pow[x][i]) % MOD;
        }
    }

    let mut dp = vec![0; c + 1];
    dp[0] = 1;
    for i in 0..n {
        let (a, b) = (a[i], b[i]);
        let mut next = vec![0; c + 1];
        for cur in 0..(c + 1) {
            if dp[cur] == 0 {
                continue;
            }
            for add in 0..(c + 1) {
                if cur + add > c {
                    break;
                }

                next[cur + add] += dp[cur] * (sum_pow[b + 1][add] + MOD - sum_pow[a][add]);
                next[cur + add] %= MOD;
            }
        }
        dp = next;
    }
    println!("{}", dp[c]);
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
