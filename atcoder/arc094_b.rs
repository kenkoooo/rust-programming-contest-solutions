use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let q: usize = sc.read();
    for _ in 0..q {
        let a: i64 = sc.read();
        let b: i64 = sc.read();
        let (a, b) = if a > b { (b, a) } else { (a, b) };
        if a * b <= a + 1 {
            println!("0");
            continue;
        }

        let mut ok = 1;
        let mut ng = b;
        while ng - ok > 1 {
            let m = (ok + ng) / 2;
            let mut max = cmp::max(a + m, m * (a + 1));
            let x1 = (m + 1 - a) / 2;
            if 1 <= x1 && x1 <= m {
                max = cmp::max((m + 1 - x1) * (a + x1), max);
            }
            let x2 = (m + 1 - a + 1) / 2;
            if 1 <= x2 && x2 <= m {
                max = cmp::max((m + 1 - x2) * (a + x2), max);
            }

            if a * b > max {
                ok = m;
            } else {
                ng = m;
            }
        }
        println!("{}", ok + a - 1);
    }
}

/// A<B
/// (A-i)*(B+i) = AB-iB+iA-i^2<AB
/// 1...m, A+1,...,A+m
/// (A+i)(m+1-i) 1<=i<=m
/// -i2 + (m+1-A)i + A(m+1)
/// -(i - (m+1-A)/2)^2 + (m+1-A)^2/4+A(m+1)<AB
/// (A^2)/4 - (m+1)/2 + (m+1)^2/4 + A(m+1) < AB
/// m+1-A

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
