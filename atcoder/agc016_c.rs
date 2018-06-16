fn main() {
    let mut sc = Scanner::new();
    let h = sc.usize_read();
    let w = sc.usize_read();
    let sh = sc.usize_read();
    let sw = sc.usize_read();

    if h % sh == 0 && w % sw == 0 {
        println!("No");
    } else if h % sh == 0 {
        let row = create_one(w, sw);
        println!("Yes");
        for _ in 0..h {
            for &r in &row {
                print!("{} ", r);
            }
            println!();
        }
    } else {
        let column = create_one(h, sh);
        println!("Yes");
        for &c in &column {
            for _ in 0..w {
                print!("{} ", c);
            }
            println!();
        }
    }
}

fn create_one(w: usize, sw: usize) -> Vec<i64> {
    let mut sum: Vec<i64> = vec![10000000; w + 1];
    sum[0] = 0;
    for i in 0..(w + 1 - sw) {
        sum[i + sw] = sum[i] - 1;
    }
    (0..w).map(|i| sum[i + 1] - sum[i]).collect()
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
