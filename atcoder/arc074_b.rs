use std::collections::BinaryHeap;
use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: Vec<i64> = (0..(3 * n)).map(|_| sc.read()).collect();
    let mut prefix_sum = vec![0; 3 * n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(-a[i]);
        prefix_sum[i] = a[i];
        if i > 0 {
            prefix_sum[i] += prefix_sum[i - 1];
        }
    }

    for i in n..(3 * n) {
        let p = -heap.pop().unwrap();
        let s = cmp::max(p, a[i]);
        heap.push(-s);
        prefix_sum[i] = prefix_sum[i - 1] - p + s;
    }

    heap.clear();
    let mut suffix_sum = vec![0; 3 * n];
    for i in ((2 * n)..(3 * n)).rev() {
        heap.push(a[i]);
        suffix_sum[i] = a[i];
        if i < 3 * n - 1 {
            suffix_sum[i] += suffix_sum[i + 1];
        }
    }

    for i in (0..(2 * n)).rev() {
        let p = heap.pop().unwrap();
        let s = cmp::min(p, a[i]);
        heap.push(s);
        suffix_sum[i] = suffix_sum[i + 1] - p + s;
    }

    let mut max = std::i64::MIN;
    for i in n..(2 * n + 1) {
        let prefix = prefix_sum[i - 1];
        let suffix = suffix_sum[i];
        max = cmp::max(max, prefix - suffix);
    }
    println!("{}", max);
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

