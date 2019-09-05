const MOD: u64 = 1e9 as u64 + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let a: Vec<u64> = sc.vec(n);
    let mut stack = 0;
    let mut ans = 1;
    for i in 0..n {
        if 2 * stack + 1 > a[i] {
            ans *= stack + 1;
            ans %= MOD;
        } else {
            stack += 1;
        }
    }
    for i in 1..(stack + 1) {
        ans *= i;
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
