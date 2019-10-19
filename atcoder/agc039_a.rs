use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let mut s = sc.chars();
    let n = s.len();
    let k: usize = sc.read();

    if s[0] != s[n - 1] {
        let ans = calc(&mut s);
        println!("{}", ans * k);
        return;
    }

    let mut s1 = s.clone();
    let ans1 = calc(&mut s1);
    if s1[n - 1] == '.' {
        println!("{}", ans1 * k);
        return;
    }

    let ans_t = ans1 * k + k - 1;

    let mut s2 = s.clone();
    s2[0] = '.';
    let ans2 = calc(&mut s2);
    if s2[n - 1] == '.' {
        let ans = (k + 1) / 2 * ans1 + k / 2 * (ans2 + 1);
        println!("{}", cmp::min(ans, ans_t));
        return;
    }
    // s1: a.xa 1
    // s2: .axa 1
    // a.xa .axa .axa

    let edge = (k - 1) / 2;
    let ans = (k + 1) / 2 * ans1 + k / 2 * (ans2 + 1) + edge;
    let ans_s = ans1 + (ans2 + 1) * (k - 1);
    println!("{}", cmp::min(ans, cmp::min(ans_s, ans_t)));
}

fn calc(s: &mut Vec<char>) -> usize {
    let n = s.len();
    let mut ans = 0;
    for i in 0..(n - 1) {
        if s[i] == s[i + 1] {
            s[i + 1] = '.';
            ans += 1;
        }
    }
    ans
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
