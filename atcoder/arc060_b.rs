fn main() {
    let mut sc = Scanner::new();
    let n: u64 = sc.read();
    let s: u64 = sc.read();
    for b in 2..2000000 {
        let mut cur = n;
        let mut check = 0;
        while cur > 0 {
            check += cur % b;
            cur /= b;
        }
        if check == s {
            println!("{}", b);
            return;
        }
    }

    if n > s {
        for x in (1..2000000).rev() {
            if (n - s) % x != 0 {
                continue;
            }

            let b = (n - s) / x + 1;
            if s < x { continue; }
            let y = s - x;
            if n == b * x + y && s == x + y && b > y {
                println!("{}", b);
                return;
            }
        }
    }

    if n == s {
        println!("{}", n + 1);
    } else {
        println!("-1");
    }
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

