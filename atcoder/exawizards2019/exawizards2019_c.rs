use std::cmp;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let q: usize = sc.read();
    let s = sc.chars();
    let td: Vec<(char, bool)> = (0..q)
        .map(|_| (sc.chars()[0], sc.chars()[0] == 'R'))
        .collect();
    let s_rev = s.iter().rev().cloned().collect::<Vec<_>>();
    let td_rev = td.iter().map(|&(t, d)| (t, !d)).collect::<Vec<_>>();

    let to_left = binary_search(s, td);
    let to_right = binary_search(s_rev, td_rev);
    println!("{}", n - cmp::min(to_left + to_right, n));
}

fn binary_search(s: Vec<char>, queries: Vec<(char, bool)>) -> usize {
    let n = s.len();
    let mut ok = 0;
    let mut ng = n + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if solve(mid - 1, &s, &queries) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn solve(mut cur: usize, s: &Vec<char>, queries: &Vec<(char, bool)>) -> bool {
    let n = s.len();
    for &(c, is_right) in queries.iter() {
        if s[cur] == c {
            if is_right {
                if cur == n - 1 {
                    return false;
                } else {
                    cur += 1;
                }
            } else {
                if cur == 0 {
                    return true;
                } else {
                    cur -= 1;
                }
            }
        }
    }
    false
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
