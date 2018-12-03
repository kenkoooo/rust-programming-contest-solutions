fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let t: usize = sc.read();
    for _ in 0..t {
        let a: i64 = sc.read();
        let b: i64 = sc.read();
        let c: i64 = sc.read();
        let d: i64 = sc.read();
        let ans = if d < b || a < b {
            "No"
        } else if c >= b {
            "Yes"
        } else {
            // b - a - 1 >= bx + dy >= c - a + 1
            let g = gcd(b, d);
            let left = b - a - 1;
            let right = c - a + 1;
            if left >= 0 {
                if left / g * g >= right {
                    "No"
                } else {
                    "Yes"
                }
            } else if right <= 0 {
                if right.abs() / g * g * (-1) <= left {
                    "No"
                } else {
                    "Yes"
                }
            } else {
                "No"
            }
        };
        println!("{}", ans);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
