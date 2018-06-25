fn s(x: usize) -> usize {
    let mut x = x;
    let mut cur = 0;
    while x > 0 {
        cur += x % 10;
        x /= 10;
    }
    cur
}

fn main() {
    let mut sc = Scanner::new();
    let mut k = sc.usize_read();
    for i in 1..10 {
        println!("{}", i);
        k -= 1;
        if k == 0 {
            return;
        }
    }
    for i in 1..10 {
        println!("{}9", i);
        k -= 1;
        if k == 0 {
            return;
        }
    }
    for i in 1..10 {
        println!("{}99", i);
        k -= 1;
        if k == 0 {
            return;
        }
    }

    let mut d = 2;
    let mut cur = 20;
    loop {
        for i in 10..cur {
            print!("{}", i);
            for _ in 0..d {
                print!("9");
            }
            println!();
            k -= 1;
            if k == 0 {
                return;
            }
        }
        let s = cur / 10;
        for i in s..10 {
            print!("{}9", i);
            for _ in 0..d {
                print!("9");
            }
            println!();
            k -= 1;
            if k == 0 {
                return;
            }
        }
        d += 1;
        cur += 10;
        if cur == 110 {
            cur = 10;
        }
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
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

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
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
