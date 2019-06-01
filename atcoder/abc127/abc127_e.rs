const MOD: u64 = 1e9 as u64 + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: u64 = sc.read();
    let m: u64 = sc.read();
    let k: usize = sc.read();

    let mut a = 0;
    for w in 2..(m + 1) {
        a += (m - w + 1);
        a %= MOD;
    }

    let mut x = 0;
    for w in 2..(m + 1) {
        x += (n + 1) * (m - w + 1) + MOD - ((m - w + 1) * (w - 2)) % MOD;
        x %= MOD;
    }

    let mut z = 0;
    for w in 2..(m + 1) {
        z += (n + 1) * (m - w + 1) * (w - 2);
        z %= MOD;
    }

    let mut ans = 0;
    for h in 2..(n + 1) {
        // w = 2..(m+1) (n-h+1)*(m-w+1)*(h+w-2)
        // (n-h+1)*(m-w+1)*(h+w-2)
        // -(m-w+1)*h^2 + (n+1)*(m-w+1)*h - (m-w+1)*(w-2)h + (n-1)(m-w+1)(w-2)
        // w = 3 (n-h+1)
        ans += x * h + z + MOD - a * (h * h) % MOD;
        ans %= MOD;
    }
    ans *= 2;

    for w in 2..(m + 1) {
        ans += (w - 1) * n * (m - w + 1);
        ans %= MOD;
    }
    for h in 2..(n + 1) {
        ans += (h - 1) * m * (n - h + 1);
        ans %= MOD;
    }

    // (n*m-2)C(k-2)

    let mut inv = vec![1; k];
    for i in 1..k {
        inv[i] = mod_pow(i as u64, MOD - 2);
    }
    let mut comb = 1;
    for i in 1..(k - 2 + 1) {
        comb *= n * m - 2 - (i as u64 - 1);
        comb %= MOD;
        comb *= inv[i];
        comb %= MOD;
    }
    ans *= comb;
    ans %= MOD;

    println!("{}", ans);
}

fn mod_pow(x: u64, mut e: u64) -> u64 {
    let mut cur = x;
    let mut result = 1;
    while e > 0 {
        if e & 1 == 1 {
            result *= cur;
            result %= MOD;
        }
        e >>= 1;
        cur = (cur * cur) % MOD;
    }
    result
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
