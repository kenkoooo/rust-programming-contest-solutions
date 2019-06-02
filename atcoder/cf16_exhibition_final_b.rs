fn main() {
    let mut sc = Scanner::new();
    let a: (f64, f64) = (sc.read(), sc.read());
    let b: (f64, f64) = (sc.read(), sc.read());
    let c: (f64, f64) = (sc.read(), sc.read());

    let ab = dist(a, b);
    let bc = dist(b, c);
    let ca = dist(c, a);

    let s = (ab + bc + ca) / 2.0;
    let t = (s * (s - ab) * (s - bc) * (s - ca)).sqrt();
    let r = t / s;

    let m = max(ab, max(bc, ca));
    let x = r / (2.0 * r / m + 1.0);
    println!("{}", x);
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (x, y) = a;
    let (z, w) = b;
    let dx = x - z;
    let dy = y - w;
    (dx * dx + dy * dy).sqrt()
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
