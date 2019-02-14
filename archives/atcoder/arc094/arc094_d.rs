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
    if n <= 5 {
        println!("{}", brute_force(s));
        return;
    }

    if s.iter().all(|&c| c == s[0]) {
        println!("1");
        return;
    }

    let mut dp = vec![vec![vec![0; 2]; 3]; 3];
    dp[0][0][0] = 1;

    for i in 0..n {
        let mut next = vec![vec![vec![0; 2]; 3]; 3];
        for m in 0..3 {
            for from in 0..3 {
                for distinct in 0..2 {
                    if dp[from][m][distinct] == 0 {
                        continue;
                    }
                    for to in 0..3 {
                        let next_distinct = if (distinct == 0 && from != to) || i == 0 {
                            0
                        } else {
                            1
                        };
                        let next_m = (m + to) % 3;
                        next[to][next_m][next_distinct] += dp[from][m][distinct];
                        next[to][next_m][next_distinct] %= MOD;
                    }
                }
            }
        }
        dp = next;
    }

    let m = s.iter().sum::<usize>() % 3;
    let is_distinct = (1..n).all(|i| s[i] != s[i - 1]);
    let mut ans = 0;
    for to in 0..3 {
        ans += dp[to][m][1];
    }
    if is_distinct {
        ans += 1;
    }
    ans %= MOD;
    println!("{}", ans);
}

fn brute_force(s: Vec<usize>) -> usize {
    let n = s.len();
    let mut set = BTreeSet::new();
    set.insert(s);
    loop {
        let mut next = BTreeSet::new();
        for s in set.iter() {
            for i in 1..n {
                let mut t = s.clone();
                if t[i - 1] != t[i] {
                    let next = match t[i - 1] + t[i] {
                        1 => 2,
                        2 => 1,
                        3 => 0,
                        _ => unreachable!(),
                    };
                    t[i - 1] = next;
                    t[i] = next;
                }
                next.insert(t);
            }
        }

        let prev = set.len();
        set.extend(next);
        if set.len() == prev {
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
