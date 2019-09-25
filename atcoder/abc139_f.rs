fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut v: Vec<_> = (0..n)
        .map(|_| (sc.read::<f64>(), sc.read::<f64>()))
        .collect();
    v.sort_by(|&(x1, y1), &(x2, y2)| y1.atan2(x1).partial_cmp(&y2.atan2(x2)).unwrap());
    let tmp = v.clone();
    v.extend(v.clone());

    let mut ans = 0.0;
    for head in 0..n {
        for tail in head..(head + n) {
            let (x, y) = (head..(tail + 1))
                .map(|i| v[i])
                .fold((0.0, 0.0), |(x, y), (vx, vy)| (x + vx, y + vy));
            update_max(&mut ans, x * x + y * y);
        }
    }
    println!("{}", ans.sqrt());
}

fn update_max<T: PartialOrd>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
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
