const C: usize = 7;
const BUF: usize = 600;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let mut k: usize = sc.read();
    let mut precompute = [[0; C]; BUF];

    for from in 0..BUF {
        for to in from..BUF {
            for next in 0..C {
                precompute[to][next] += if next == 0 {
                    1
                } else {
                    precompute[from][next - 1]
                };
            }
        }
    }

    let mut ans = String::new();
    for i in (0..BUF).rev() {
        let count = k / precompute[i][C - 1];
        k %= precompute[i][C - 1];
        for _ in 0..count {
            ans.push('L');
        }
        ans.push_str("AVITSEF");
    }

    ans = ans.chars().rev().collect();
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
