fn main() {
    let mut sc = Scanner::new();
    let w: usize = sc.read();
    let h: usize = sc.read();

    let mut edges = Vec::new();
    for _ in 0..w {
        let c: usize = sc.read();
        edges.push((c, true));
    }
    for _ in 0..h {
        let c: usize = sc.read();
        edges.push((c, false));
    }
    edges.sort();

    let h = h + 1;
    let w = w + 1;

    let mut ans = 0;
    let mut count_h = 0;
    let mut count_v = 0;
    for &(cost, horizontal) in &edges {
        if horizontal {
            if count_v >= h {
                continue;
            }
            ans += cost * (h - count_v);
            count_h += 1;
        } else {
            if count_h >= w {
                continue;
            }
            ans += cost * (w - count_h);
            count_v += 1;
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
