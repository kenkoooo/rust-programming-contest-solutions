use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut vs = Vec::new();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for _ in 0..n {
        let x: i64 = sc.read();
        let y: i64 = sc.read();
        vs.push((x, y));
        xs.push(x);
        ys.push(y);
    }
    xs.sort();
    ys.sort();
    let mut ans = std::i64::MAX;

    for x1 in 0..n {
        for x2 in x1..n {
            for y1 in 0..n {
                for y2 in 0..n {
                    let lx = xs[x1];
                    let rx = xs[x2];
                    let ly = ys[y1];
                    let ry = ys[y2];

                    let mut count = 0;
                    for t in &vs {
                        let (x, y) = *t;
                        if lx <= x && x <= rx && ly <= y && y <= ry {
                            count += 1;
                        }
                    }
                    if count >= k {
                        ans = cmp::min(ans, (rx - lx) * (ry - ly));
                    }
                }
            }
        }
    }

    println!("{}", ans);
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

