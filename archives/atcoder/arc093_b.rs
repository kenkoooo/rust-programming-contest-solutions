const MAX_H: usize = 100;

fn main() {
    let mut sc = Scanner::new();
    let mut w: usize = sc.read::<usize>() - 1;
    let mut b: usize = sc.read::<usize>() - 1;

    let mut is_white = vec![vec![false; MAX_H]; MAX_H];
    for i in 0..MAX_H {
        for j in 0..(MAX_H / 2) {
            if i % 2 == 0 || j == (MAX_H / 2) - 1 || j % 2 == 1 {
                is_white[i][j] = true;
            } else if b == 0 {
                is_white[i][j] = true;
            } else {
                is_white[i][j] = false;
                b -= 1;
            }
        }
        for j in (MAX_H / 2)..MAX_H {
            if i % 2 == 0 || j == (MAX_H / 2) || j % 2 == 0 {
                is_white[i][j] = false;
            } else if w == 0 {
                is_white[i][j] = false;
            } else {
                is_white[i][j] = true;
                w -= 1;
            }
        }
    }

    println!("100 100");
    for i in 0..MAX_H {
        for j in 0..MAX_H {
            print!("{}", if is_white[i][j] { '.' } else { '#' });
        }
        println!()
    }
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

