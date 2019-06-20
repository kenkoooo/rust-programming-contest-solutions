const MOD: usize = 1e9 as usize + 7;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();

    let s: Vec<u32> = sc.vec(n);
    let t: Vec<u32> = sc.vec(m);

    let mut dp = vec![0; m + 1];
    dp[0] = 1;
    for i in 0..n {
        let s = s[i];
        let mut carry = 0;
        let mut next = dp.clone();
        next[0] = 1;
        for j in 0..m {
            carry += dp[j];
            carry %= MOD;
            if s == t[j] {
                next[j + 1] += carry;
                next[j + 1] %= MOD;
            }
        }
        //        println!("dp={:?}", dp);
        dp = next;
    }
    //    println!("dp={:?}", dp);

    let mut ans = 0;
    for a in dp.into_iter() {
        ans += a;
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
