use std::cmp::max;
use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut xy = vec![];
    for _ in 0..n {
        let x: i64 = sc.read();
        let y: i64 = sc.read();
        xy.push((x, y));
    }

    xy.sort_by(|&(x1, y1), &(x2, y2)| {
        let t1 = (y1 as f64).atan2(x1 as f64);
        let t2 = (y2 as f64).atan2(x2 as f64);
        t1.partial_cmp(&t2).unwrap()
    });

    let tmp = xy.clone();
    xy.extend(tmp);

    let mut ans = 0;
    for head in 0..n {
        for tail in head..(head + n) {
            let mut cur = (0, 0);
            for i in head..(tail + 1) {
                cur.0 += xy[i].0;
                cur.1 += xy[i].1;
            }
            let (x, y) = cur;
            ans = max(ans, x * x + y * y);
        }
    }
    println!("{}", (ans as f64).sqrt());
}

fn update_max<T: PartialOrd>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b.abs(), a.abs() % b.abs())
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
