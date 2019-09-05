const MOD: u64 = 1e9 as u64 + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let k: u64 = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let mut before = vec![0; n];
    let mut after = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if a[j] > a[i] && j < i {
                before[i] += 1;
            }
            if a[j] > a[i] && i < j {
                after[i] += 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let b = (k * (k + 1) / 2) % MOD;
        let b = (before[i] * b) % MOD;

        let a = (k * (k - 1) / 2) % MOD;
        let a = (a * after[i]) % MOD;
        ans += a + b;
        ans %= MOD;
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
