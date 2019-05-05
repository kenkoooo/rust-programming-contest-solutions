fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.read();
    let w: usize = sc.read();

    let mut count = vec![0; 26];
    for _ in 0..h {
        let v: Vec<char> = sc.read::<String>().chars().collect();
        for c in &v {
            let c = (*c as usize) - ('a' as usize);
            count[c] += 1;
        }
    }

    let mut required = vec![0; 5];
    if h % 2 == 1 {
        required[2] += (w / 2);
    }
    if w % 2 == 1 {
        required[2] += (h / 2);
    }
    if h % 2 == 1 && w % 2 == 1 {
        required[1] += 1;
    }
    required[4] = (h * w - required[2] * 2 - required[1]) / 4;

    let mut merged_count = vec![0; 5];
    for i in 0..26 {
        let mut c = count[i];
        if c % 2 == 1 {
            c -= 1;
            merged_count[1] += 1;
        }
        if c % 4 == 2 {
            c -= 2;
            merged_count[2] += 1;
        }
        merged_count[4] += c / 4;
    }

    if merged_count[1] != required[1] {
        println!("No");
        return;
    }
    if merged_count[4] < required[4] {
        println!("No");
        return;
    }
    merged_count[4] -= required[4];
    merged_count[2] += merged_count[4] * 2;
    assert_eq!(merged_count[2], required[2]);
    println!("Yes");
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

