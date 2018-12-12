const MOD: usize = 998244353;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let s: Vec<usize> = sc
        .read::<String>()
        .chars()
        .map(|c| c as usize - ('a' as usize))
        .collect();
    let n = s.len();
    if n <= 5 {
        use std::collections::{BTreeSet, VecDeque};
        let mut q = VecDeque::new();
        let mut set = BTreeSet::new();
        q.push_back(s.clone());
        set.insert(s);
        while let Some(v) = q.pop_front() {
            for i in 1..v.len() {
                if v[i] != v[i - 1] {
                    let mut next = v.clone();
                    let mut x = vec![v[i], v[i - 1]];
                    x.sort();
                    match (x[0], x[1]) {
                        (0, 1) => {
                            next[i - 1] = 2;
                            next[i] = 2;
                        }
                        (0, 2) => {
                            next[i - 1] = 1;
                            next[i] = 1;
                        }
                        (1, 2) => {
                            next[i - 1] = 0;
                            next[i] = 0;
                        }
                        _ => unreachable!(),
                    }
                    if !set.contains(&next) {
                        set.insert(next.clone());
                        q.push_back(next);
                    }
                }
            }
        }
        println!("{}", set.len());
        return;
    }

    if s.iter().all(|&x| x == s[0]) {
        println!("1");
        return;
    }

    let remain = s.iter().sum::<usize>() % 3;

    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 2]; 3]; 3]; n + 1];
    dp[0][0][0][0] = 1;
    for i in 0..n {
        for tail in 0..3 {
            for next in 0..3 {
                for remain in 0..3 {
                    let next_remain = (remain + next) % 3;
                    let doubled = if tail == next && i > 0 { 1 } else { 0 };
                    dp[i + 1][next][next_remain][doubled] += dp[i][tail][remain][0];
                    dp[i + 1][next][next_remain][doubled] %= MOD;

                    dp[i + 1][next][next_remain][1] += dp[i][tail][remain][1];
                    dp[i + 1][next][next_remain][1] %= MOD;
                }
            }
        }
    }
    let mut ans: usize = 0;
    for tail in 0..3 {
        ans += dp[n][tail][remain][1];
        ans %= MOD;
    }

    let mut doubled = false;
    for i in 1..n {
        if s[i - 1] == s[i] {
            doubled = true;
            break;
        }
    }
    if !doubled {
        ans += 1;
    }
    ans %= MOD;
    println!("{}", ans);
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
