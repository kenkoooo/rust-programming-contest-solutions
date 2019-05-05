use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: u32 = sc.read();
    let mut a: Vec<u32> = vec![0; n];
    let mut b: Vec<u64> = vec![0; n];
    for i in 0..n {
        a[i] = sc.read();
        b[i] = sc.read();
    }

    let mut ans = 0;
    for pos in 0..32 {
        if (k >> pos) & 1 == 0 {
            continue;
        }

        let suffix = (1 << pos) - 1;
        let prefix = (k >> (pos + 1)) << (pos + 1);
        let x = prefix + suffix;

        let mut value = 0;
        for i in 0..n {
            if (a[i] | x) == x {
                value += b[i];
            }
        }

        ans = cmp::max(ans, value);
    }
    let mut value = 0;
    for i in 0..n {
        if (a[i] | k) == k {
            value += b[i];
        }
    }

    ans = cmp::max(ans, value);
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
