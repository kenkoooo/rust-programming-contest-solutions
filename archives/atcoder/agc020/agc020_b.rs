fn main() {
    let mut sc = Scanner::new();
    let k: usize = sc.read();
    let a: Vec<usize> = (0..k).map(|_| sc.read()).collect();

    let mut lower = 2;
    for &a in a.iter().rev() {
        if a > lower {
            lower = a;
        } else if a < lower {
            lower = (lower + a - 1) / a * a;
        }
    }

    let mut check_low = lower;
    for &a in &a {
        check_low = check_low / a * a;
    }

    if check_low != 2 {
        println!("-1");
        return;
    }

    let mut ok = lower;
    let mut ng = std::usize::MAX / 2;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut check = mid;
        for &a in &a {
            check /= a;
            check *= a;
        }
        if check == 2 {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{} {}", lower, ok);
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

