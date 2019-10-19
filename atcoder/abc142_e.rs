use std::cmp;

const INF: u64 = std::u64::MAX / 2;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut keys = vec![];
    for _ in 0..m {
        let a: u64 = sc.read();
        let b: usize = sc.read();
        let mut mask = 0;
        for _ in 0..b {
            let c = sc.read::<usize>() - 1;
            mask |= (1 << c);
        }
        keys.push((a, mask));
    }

    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for (price, key_mask) in keys.into_iter() {
        for mask in 0..(1 << n) {
            let next = mask | key_mask;
            dp[next] = cmp::min(dp[next], dp[mask] + price);
        }
    }

    let ans = dp[(1 << n) - 1];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
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
