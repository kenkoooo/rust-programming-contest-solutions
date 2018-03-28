const MAX_N: usize = 100000;

fn main() {
    let mut sc = Scanner::new();

    let mut is_prime = vec![true; MAX_N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..(MAX_N + 1) {
        if !is_prime[p] {
            continue;
        }
        let mut cur = p * 2;
        while cur <= MAX_N {
            is_prime[cur] = false;
            cur += p;
        }
    }

    let mut is_liked = vec![false; MAX_N + 1];
    for n in 3..(MAX_N + 1) {
        if n % 2 == 1 && is_prime[n] && is_prime[(n + 1) / 2] {
            is_liked[n] = true;
        }
    }
    let mut sum = vec![0; is_liked.len() + 1];
    for i in 0..is_liked.len() {
        sum[i + 1] = sum[i] + if is_liked[i] { 1 } else { 0 };
    }

    println!("{:?}", sum);
//    let q: usize = sc.read();
//    for _ in 0..q {
//        let l: usize = sc.read();
//        let r: usize = sc.read();
//        println!("{}", sum[r + 1] - sum[l]);
//    }
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

