const MOD: usize = 1_000_000_007;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut v: Vec<(usize, bool)> = Vec::new();
    for _ in 0..n {
        v.push((sc.read(), true));
    }
    for _ in 0..n {
        v.push((sc.read(), false));
    }
    v.sort();

    let mut ans = 1;
    let mut a_stack = 0;
    let mut b_stack = 0;
    for &(_, is_a) in &v {
        if is_a {
            if b_stack > 0 {
                ans = (ans * b_stack) % MOD;
                b_stack -= 1;
            } else {
                a_stack += 1;
            }
        } else {
            if a_stack > 0 {
                ans = (ans * a_stack) % MOD;
                a_stack -= 1;
            } else {
                b_stack += 1;
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
