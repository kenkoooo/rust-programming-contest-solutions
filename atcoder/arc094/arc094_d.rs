use std::collections::BTreeSet;

const MOD: usize = 998244353;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc
        .chars()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let n = s.len();
    if n <= 3 {
        println!("{}", brute_force(s));
        return;
    }
    if (1..n).all(|i| s[i - 1] == s[i]) {
        println!("1");
        return;
    }
    let mut dp = vec![vec![vec![0; 2]; 3]; 3];
    dp[0][0][0] = 1;
    for i in 0..n {
        let mut next_dp = vec![vec![vec![0; 2]; 3]; 3];
        for last in 0..3 {
            for cur_mod in 0..3 {
                for next in 0..3 {
                    let next_mod = (cur_mod + next) % 3;
                    for in_a_row in 0..2 {
                        let next_in_a_row = if (last == next && i > 0) || in_a_row == 1 {
                            1
                        } else {
                            0
                        };

                        next_dp[next][next_mod][next_in_a_row] += dp[last][cur_mod][in_a_row];
                        next_dp[next][next_mod][next_in_a_row] %= MOD;
                    }
                }
            }
        }
        dp = next_dp;
    }

    let modulo = s.iter().sum::<usize>() % 3;
    let mut ans = 0;
    for last in 0..3 {
        ans += dp[last][modulo][1];
    }
    if (1..n).all(|i| s[i - 1] != s[i]) {
        ans += 1;
    }
    println!("{}", ans % MOD);
}

fn brute_force(s: Vec<usize>) -> usize {
    let mut set = BTreeSet::new();
    let n = s.len();
    set.insert(s);
    loop {
        let mut next = BTreeSet::new();
        for s in set.iter() {
            for i in 1..n {
                if s[i - 1] != s[i] {
                    let mut s = s.clone();
                    let replace = match s[i - 1] + s[i] {
                        1 => 2,
                        2 => 1,
                        3 => 0,
                        _ => unreachable!(),
                    };
                    s[i - 1] = replace;
                    s[i] = replace;
                    next.insert(s);
                }
            }
        }
        let prev = set.len();
        set.extend(next);
        if prev == set.len() {
            return set.len();
        }
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
