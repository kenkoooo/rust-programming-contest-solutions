use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let q: usize = sc.read();
    for _ in 0..q {
        let a: i64 = sc.read();
        let b: i64 = sc.read();
        println!("{}", solve(a, b));
    }
}

fn solve(a: i64, b: i64) -> i64 {
    let (a, b) = if a > b { (b, a) } else { (a, b) };

    let mut ok = 0;
    let mut ng = b;
    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        if get_max(a, m) < a * b {
            ok = m;
        } else {
            ng = m;
        }
    }
    ok + a - 1
}
// (a+1)*m, (a+2)(m-1), ... (a+m-1)*2, (a+m)
// 1 <= x <= m
// (a+x)(m+1-x) = -(x-m-1)(x+a)
// -(xx +(a-m-1)x +..)
// -(x+(a-m-1)/2)^2...
// x= -(a-m-1)/2
fn get_max(a: i64, m: i64) -> i64 {
    // 1 <= -(a-m-1)/2 <= m
    // 2 <= -(a-m-1) <= 2m
    if 2 <= -(a - m - 1) && -(a - m - 1) <= 2 * m {
        let x1 = -(a - m - 1) / 2;
        let x2 = x1 + 1;
        [x1, x2]
            .into_iter()
            .filter(|&&x| 1 <= x && x <= m)
            .map(|&x| (a + x) * (m + 1 - x))
            .max()
            .unwrap()
    } else {
        cmp::max((a + 1) * m, a + m)
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
