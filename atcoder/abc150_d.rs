fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: i64 = sc.read();
    let a: Vec<i64> = sc.vec(n);

    let mut b = vec![];
    for i in 0..n {
        b.push(a[i] / 2);
    }

    match chinese_remainder_theorem(&b, &a) {
        Some((x, y)) => {
            if x > m {
                println!("0");
            } else {
                let z = m - x;
                if x > 0 {
                    println!("{}", z / y + 1);
                } else {
                    println!("{}", z / y);
                }
            }
        }
        _ => {
            println!("0");
        }
    }
}

pub fn extended_gcd(a: i64, b: i64, p: &mut i64, q: &mut i64) -> i64 {
    if b == 0 {
        *p = 1;
        *q = 0;
        a
    } else {
        let d = extended_gcd(b, a % b, q, p);
        *q -= a / b * *p;
        d
    }
}

pub fn chinese_remainder_theorem(b: &Vec<i64>, modulo: &Vec<i64>) -> Option<(i64, i64)> {
    let (mut r, mut m) = (0, 1);
    for i in 0..b.len() {
        let (mut p, mut q) = (0, 0);
        let d = extended_gcd(m, modulo[i], &mut p, &mut q);
        if (b[i] - r) % d != 0 {
            return None;
        }
        let tmp = ((b[i] - r) / d * p) % (modulo[i] / d);
        r += m * tmp;
        m *= modulo[i] / d;
    }
    Some(((r % m + m) % m, m))
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
