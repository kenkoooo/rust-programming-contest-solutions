use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let a: usize = sc.read();
    let b: usize = sc.read();

    let mut v: Vec<usize> = (0..n).map(|_| sc.read()).collect();
    v.sort();
    v.reverse();

    let mut sum = 0;
    for i in 0..a {
        sum += v[i];
    }

    let mut selectable = 0;
    for i in 0..n {
        if v[i] == v[a - 1] {
            selectable += 1;
        }
    }

    let mut big_count = 0;
    for i in 0..n {
        if v[i] > v[a - 1] {
            big_count += 1;
        }
    }

    let mut ans = 0;


    let from = a - big_count;
    let to = if v[a - 1] * a != sum { from } else { cmp::min(b - big_count, selectable) };
    for i in from..(to + 1) {
        let mut combination: usize = 1;
        for j in 0..i {
            combination *= (selectable - j);
            combination /= (j + 1);
        }
        ans += combination;
    }

    println!("{}", (sum as f64) / (a as f64));
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

