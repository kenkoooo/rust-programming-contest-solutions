const MAX_P: usize = 100_000;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();

    if n <= 5 {
        let t = vec![2, 3, 5, 7, 11];
        for i in 0..n {
            print!("{} ", t[i]);
        }
        println!();
        return;
    }

    let mut is_prime = vec![true; MAX_P];
    for p in 3..MAX_P {
        if !is_prime[p] {
            continue;
        }
        for i in 2..(MAX_P / p) {
            is_prime[p * i] = false;
        }
    }

    let mut ans = Vec::new();
    let mut cur = 7;
    loop {
        ans.push(cur);
        if ans.len() == n {
            break;
        }
        cur += 10;
        while !is_prime[cur] {
            cur += 10;
        }
    }

    for &p in &ans {
        print!("{} ", p);
    }
    println!();
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
