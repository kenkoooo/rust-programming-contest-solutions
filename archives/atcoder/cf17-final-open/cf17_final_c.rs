use std::cmp;
const MAX_D: usize = 13;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut count = vec![0; MAX_D];
    count[0] = 1;
    for _ in 0..n {
        let d: usize = sc.read();
        count[d] += 1;
    }

    for i in 0..MAX_D {
        if count[i] >= 3 {
            println!("0");
            return;
        }
    }

    let mut max = 0;
    for mask in 0..(1 << MAX_D) {
        let mut v = Vec::new();

        // construct
        for i in 0..MAX_D {
            if count[i] == 2 {
                v.push(12 - i);
                v.push(12 + i);
            } else if count[i] == 1 {
                if (mask >> i) & 1 != 0 {
                    v.push(12 - i);
                } else {
                    v.push(12 + i);
                }
            }
        }

        // check
        v.sort();
        let mut min = 30;
        for i in 0..v.len() {
            let from = v[i];
            for j in (i + 1)..v.len() {
                let to = v[j];
                let d = to - from;
                let d = cmp::min(d, 24 - d);
                min = cmp::min(min, d);
            }
        }

        max = cmp::max(max, min);
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
