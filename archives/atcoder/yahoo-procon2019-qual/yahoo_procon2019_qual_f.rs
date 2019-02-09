use std::cmp;

const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let red: Vec<usize> = sc.chars().iter().map(|&c| c as usize - '0' as usize).collect();
    let n = red.len();

    let mut dp = vec![0; 2 * n + 1];
    dp[red[0]] = 1;
    for turn in 0..(2 * n) {
        let mut next = vec![0; 2 * n + 1];

        let cur_balls = 2 * cmp::min(turn + 1, n) - turn;
        let supplied_red = if turn + 1 < n { red[turn + 1] } else { 0 };

        for cur_red in 0..(cur_balls + 1) {
            if cur_red > 0 {
                // pop red
                next[cur_red + supplied_red - 1] += dp[cur_red];
                next[cur_red + supplied_red - 1] %= MOD;
            }
            if cur_balls - cur_red > 0 {
                // pop blue
                next[cur_red + supplied_red] += dp[cur_red];
                next[cur_red + supplied_red] %= MOD;
            }
        }

        dp = next;
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
