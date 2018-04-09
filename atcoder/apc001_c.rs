fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();

    let mut from = 0;
    println!("{}", from);
    let mut from_s = state(&sc.read::<String>());

    if from_s == 2 {
        return;
    }

    let mut to = n - 1;
    println!("{}", to);
    let mut to_s = state(&sc.read::<String>());

    if to_s == 2 {
        return;
    }


    for _ in 0..18 {
        let mid = (from + to) / 2;
        println!("{}", mid);
        let mut mid_s = state(&sc.read::<String>());
        if mid_s == 2 { return; }
        let prefix = mid - from + 1;
        if (mid - from) % 2 == 0 {
            if from_s != mid_s {
                to = mid;
                to_s = mid_s;
            } else {
                from = mid;
                from_s = mid_s;
            }
        } else {
            if from_s == mid_s {
                to = mid;
                to_s = mid_s;
            } else {
                from = mid;
                from_s = mid_s;
            }
        }
    }
    panic!();
}

fn state(s: &str) -> usize {
    if s == "Female" { 0 } else if s == "Male" { 1 } else { 2 }
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

