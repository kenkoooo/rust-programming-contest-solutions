fn main() {
    let mut sc = Scanner::new();
    let mut s: Vec<usize> = sc.read::<String>().chars().map(|c| (c as usize) - ('a' as usize)).collect();

    if s.len() < 26 {
        let mut used = vec![false; 26];
        for c in &s {
            used[*c] = true;
        }

        for c in &s {
            print!("{}", ((*c as u8) + ('a' as u8)) as char);
        }
        for i in 0..26 {
            if used[i] { continue; }
            println!("{}", ((i as u8) + ('a' as u8)) as char);
            return;
        }
    }

    let mut used = vec![true; 26];
    for i in (0..26).rev() {
        let c = s[i];
        let mut back = 26;
        for j in c..26 {
            if !used[j] {
                back = j;
                break;
            }
        }
        if back == 26 {
            used[c] = false;
        } else {
            for j in 0..i {
                print!("{}", ((s[j] as u8) + ('a' as u8)) as char);
            }
            println!("{}", ((back as u8) + ('a' as u8)) as char);
            return;
        }
    }

    println!("-1");
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

