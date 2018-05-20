// extern crate rand;
// use rand::prelude::*;
use std::cmp;
fn main() {
    // test();
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<i64> = (0..n).map(|_| sc.read()).collect();
    println!("{}", solve(&a));
}

fn test() {
    // let mut rng = thread_rng();
    // let n = 10;
    // let t = 4;
    // let mut x = vec![0; n];
    // for _ in 0..t {
    //     let i = rng.gen_range(0, n - 1);
    //     x[i + 1] = x[i] + 1;
    // }
    // if solve(&x) > t {
    //     println!("{:?}", x);
    //     println!("{}", solve(&x));
    // }
}

fn solve(a: &Vec<i64>) -> i64 {
    let n = a.len();

    if a[0] != 0 {
        return -1;
    }

    for i in 1..n {
        if a[i] > a[i - 1] + 1 {
            return -1;
        }
    }

    let mut ans = 0;
    let mut cur = 0;
    for i in (0..n).rev() {
        if cur < a[i] {
            ans += a[i];
        }
        cur = cmp::max(a[i] - 1, 0);
    }

    return ans;
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
