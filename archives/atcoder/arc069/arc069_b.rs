fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let s: Vec<char> = sc.read::<String>().chars().collect();

    let check = |sheep: &mut Vec<bool>| {
        for pos in 2..n {
            if sheep[pos - 1] {
                if s[pos - 1] == 'o' {
                    sheep[pos] = sheep[pos - 2];
                } else {
                    sheep[pos] = !sheep[pos - 2];
                }
            } else {
                if s[pos - 1] != 'o' {
                    sheep[pos] = sheep[pos - 2];
                } else {
                    sheep[pos] = !sheep[pos - 2];
                }
            }
        }
        for i in 0..n {
            if sheep[i] {
                if s[i] == 'o' && sheep[(i - 1 + n) % n] != sheep[(i + 1) % n] {
                    return false;
                }
                if s[i] == 'x' && sheep[(i - 1 + n) % n] == sheep[(i + 1) % n] {
                    return false;
                }
            } else {
                if s[i] == 'x' && sheep[(i - 1 + n) % n] != sheep[(i + 1) % n] {
                    return false;
                }
                if s[i] == 'o' && sheep[(i - 1 + n) % n] == sheep[(i + 1) % n] {
                    return false;
                }
            }
        }
        return true;
    };

    let out = |sheep: &Vec<bool>| {
        for i in 0..n {
            if sheep[i] {
                print!("S");
            } else {
                print!("W");
            }
        }
        println!();
    };

    let mut sheep = vec![true; n];
    if check(&mut sheep) {
        out(&sheep);
        return;
    }

    sheep[0] = true;
    sheep[1] = false;
    if check(&mut sheep) {
        out(&sheep);
        return;
    }
    sheep[0] = false;
    sheep[1] = true;
    if check(&mut sheep) {
        out(&sheep);
        return;
    }
    sheep[0] = false;
    sheep[1] = false;
    if check(&mut sheep) {
        out(&sheep);
        return;
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
