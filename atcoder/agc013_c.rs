fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let l: usize = sc.read();
    let t: usize = sc.read();
    let xw: Vec<(usize, usize)> = (0..n).map(|_| (sc.read(), sc.read())).collect();

    let mut clockwise = 0;
    let mut counter_clock_wise = 0;
    for (x, w) in xw.iter().cloned() {
        if w == 1 {
            clockwise += (x + t) / l;
        } else if t > x {
            counter_clock_wise += ((l - x) + t - 1) / l;
        }
    }

    let mut next = xw
        .into_iter()
        .map(|(x, w)| {
            if w == 1 {
                (x + t) % l
            } else {
                (l - ((l - x) + t) % l) % l
            }
        })
        .collect::<Vec<_>>();
    next.sort();
    let over = clockwise % n + n - counter_clock_wise % n;
    for i in 0..n {
        println!("{}", next[(i + over) % n]);
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
