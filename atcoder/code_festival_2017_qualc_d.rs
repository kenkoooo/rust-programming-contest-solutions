use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s: Vec<usize> = sc
        .chars()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect();
    let n = s.len();
    let mut prefix_dp = vec![n as u32; 1 << 26];

    let mut cur: usize = 0;
    for c in s.into_iter() {
        cur ^= 1 << c;
        if cur.count_ones() <= 1 {
            prefix_dp[cur] = 1;
        } else {
            let mut min = prefix_dp[cur];
            for i in 0..26 {
                let suffix = 1 << i;
                let prefix = cur ^ suffix;
                min = cmp::min(min, prefix_dp[prefix] + 1);
            }
            prefix_dp[cur] = min;
        }
    }

    println!("{}", prefix_dp[cur]);
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
