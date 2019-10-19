use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let q: usize = sc.read();
    for _ in 0..q {
        let a: i64 = sc.read();
        let b: i64 = sc.read();
        let ans = solve(a, b);
        println!("{}", ans);
    }
}

fn solve(a: i64, b: i64) -> i64 {
    if a == b {
        return (a - 1) + (b - 1);
    }
    let (a, b) = if a > b { (b, a) } else { (a, b) };

    let mut ok = 0;
    let mut ng = b + 1;
    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        if get_max(a, b, m) >= a * b {
            ng = m;
        } else {
            ok = m;
        }
    }
    ok + (a - 1)
}

fn get_max(a: i64, b: i64, m: i64) -> i64 {
    let left = (a + 1) * m;
    let right = a + m;

    let t1 = (m - a - 1) / 2;
    let t2 = t1 + 1;
    let mut max = cmp::max(left, right);
    if 0 <= t1 && t1 < m {
        max = cmp::max(max, (m - t1) * (a + 1 + t1));
    }
    if 0 <= t2 && t2 < m {
        max = cmp::max(max, (m - t2) * (a + 1 + t2));
    }
    max
}

// 1 ... (a-1) [a] (a+1) ...
// 1 ... (b-1) [b] (b+1) ...

// (a-x)(a+x) < ab
// m     (m-1) ...  1
// (a+1) (a+2) ... (a+m)
// (m-x)(a+1+x) = -(x+a+1)(x-m) = -x^2 - (a+1-m)x +m(a+1)
// x = (m-a-1)/2

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
