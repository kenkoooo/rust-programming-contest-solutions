use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let m = sc.read::<usize>();

    let mut odd = VecDeque::new();
    let mut even = VecDeque::new();

    for _ in 0..m {
        let a = sc.read::<usize>();
        if a % 2 == 0 {
            even.push_back(a);
        } else {
            odd.push_back(a);
        }
    }

    if odd.len() > 2 {
        println!("Impossible");
        return;
    }

    let mut ans_a = Vec::new();
    let mut ans_b = Vec::new();

    if !odd.is_empty() {
        let p = odd.pop_front().unwrap();
        ans_a.push(p);
        if p > 1 {
            ans_b.push(p - 1);
        }
    } else {
        let p = even.pop_front().unwrap();
        ans_a.push(p);
        ans_b.push(p - 1);
    }

    while !even.is_empty() {
        let p = even.pop_front().unwrap();
        ans_a.push(p);
        ans_b.push(p);
    }

    if !odd.is_empty() {
        let p = odd.pop_front().unwrap();
        ans_a.push(p);
        ans_b.push(p + 1);
    } else {
        ans_b.push(1);
    }

    for i in 0..ans_a.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans_a[i]);
    }
    println!();
    println!("{}", ans_b.len());
    for i in 0..ans_b.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans_b[i]);
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

