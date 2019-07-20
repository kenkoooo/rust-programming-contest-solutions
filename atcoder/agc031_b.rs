use std::collections::BTreeMap;

const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let mut a = vec![];
    for _ in 0..n {
        let x: usize = sc.read();
        if a.is_empty() || *a.iter().next_back().unwrap() != x {
            a.push(x);
        }
    }

    let n = a.len();
    let mut dp = vec![0; n + 1];
    let mut last = BTreeMap::new();
    dp[0] = 1;
    for (i, a) in a.into_iter().enumerate() {
        dp[i + 1] += dp[i];
        if let Some(&last) = last.get(&a) {
            dp[i + 1] += dp[last];
        }
        last.insert(a, i + 1);
        dp[i + 1] %= MOD;
    }
    println!("{}", dp[n]);
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
