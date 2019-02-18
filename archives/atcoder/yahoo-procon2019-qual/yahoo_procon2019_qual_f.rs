use std::cmp;

const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc.chars();
    let n = s.len();
    let a = s
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();

    let mut dp = vec![0; 2 * n + 1];
    dp[a[0]] = 1;
    for turn in 1..(2 * n + 1) {
        let mut next = vec![0; 2 * n + 1];
        let add = if turn < n { a[turn] } else { 0 };
        let total = cmp::min(turn * 2, n * 2) - turn + 1;

        for red in 0..(total + 1) {
            let blue = total - red;
            if red > 0 {
                next[red + add - 1] += dp[red];
            }
            if blue > 0 {
                next[red + add] += dp[red];
            }
        }
        dp = next.into_iter().map(|next| next % MOD).collect();
    }
    println!("{}", dp[0]);
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
