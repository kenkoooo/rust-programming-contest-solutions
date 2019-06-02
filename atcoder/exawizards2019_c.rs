fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut s = sc.chars();
    let mut queries = (0..q)
        .map(|_| (sc.chars()[0], sc.chars()[0]))
        .collect::<Vec<_>>();

    let left = solve(&queries, &s);
    queries = queries
        .into_iter()
        .map(|(t, d)| (t, if d == 'R' { 'L' } else { 'R' }))
        .collect();
    s.reverse();
    let right = solve(&queries, &s);
    let rest = n as i32 - (left + 1) - (right + 1);
    println!("{}", rest);
}

fn solve(queries: &Vec<(char, char)>, s: &Vec<char>) -> i32 {
    let n = s.len();
    let mut ok: i32 = -1;
    let mut ng: i32 = n as i32;
    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        assert!(m >= 0);
        let mut cur = m;
        for &(t, d) in queries.iter() {
            if s[cur as usize] == t {
                match d {
                    'R' => {
                        cur += 1;
                    }
                    'L' => {
                        cur -= 1;
                    }
                    _ => unreachable!(),
                }
            }
            if cur < 0 || cur >= n as i32 {
                break;
            }
        }
        if cur < 0 {
            ok = m;
        } else {
            ng = m;
        }
    }
    ok
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
