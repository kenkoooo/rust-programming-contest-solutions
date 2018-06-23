use std::cmp;
use std::ops::{Div, MulAssign, RemAssign};
const MOD: usize = 998244353;

fn mod_pow(x: usize, e: usize, modulo: usize) -> usize {
    let mut result = 1;
    let mut cur = x;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result *= cur;
            result %= modulo;
        }
        cur *= cur;
        cur %= modulo;
        e >>= 1;
    }
    result
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.usize_read();
    let v: Vec<(i64, i64)> = (0..n).map(|_| (sc.read(), sc.read())).collect();

    let mut ans = mod_pow(2, n, MOD);
    ans = (ans + MOD - 1 - n) % MOD;
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = v[i];
            let (x2, y2) = v[j];
            let dx = x1 - x2;
            let dy = y1 - y2;

            let mut count = 0;
            let mut left = i;
            let mut right = j;
            for k in 0..n {
                let (xk, yk) = v[k];
                let kx = xk - x1;
                let ky = yk - y1;
                if dx * ky == kx * dy {
                    left = cmp::min(left, k);
                    right = cmp::max(right, k);
                    count += 1;
                }
            }

            if left != i || right != j {
                continue;
            }

            let subset = mod_pow(2, count, MOD);
            ans = (ans + MOD - subset + count + 1) % MOD;
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
