fn main() {
    let mut sc = Scanner::new();
    let (x1, y1): (f64, f64) = (sc.read(), sc.read());
    let (x2, y2): (f64, f64) = (sc.read(), sc.read());
    let (x3, y3): (f64, f64) = (sc.read(), sc.read());
    let a = dist(x1, y1, x2, y2);
    let b = dist(x1, y1, x3, y3);
    let c = dist(x2, y2, x3, y3);
    let s = (a + b + c) / 2.0;
    let t = (s * (s - a) * (s - b) * (s - c)).sqrt();
    let r = t * 2.0 / (a + b + c);
    let mut max = if a > b { a } else { b };
    max = if max > c { max } else { c };

    let x = max * r / (2.0 * r + max);
    println!("{}", x);
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
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
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

