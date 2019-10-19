fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut e = vec![];
    for _ in 0..n {
        let x: f64 = sc.read();
        let y: f64 = sc.read();
        e.push((x, y));
    }
    e.sort_by(|&(x1, y1), &(x2, y2)| {
        let t1 = y1.atan2(x1);
        let t2 = y2.atan2(x2);
        t1.partial_cmp(&t2).unwrap()
    });

    let mut ans = 0.0;
    for head in 0..n {
        let (mut sx, mut sy) = (0.0, 0.0);
        for i in 0..n {
            let (x, y) = e[(head + i) % n];
            sx += x;
            sy += y;
            let s = sx * sx + sy * sy;
            if ans < s {
                ans = s;
            }
        }
    }

    println!("{}", ans.sqrt());
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
