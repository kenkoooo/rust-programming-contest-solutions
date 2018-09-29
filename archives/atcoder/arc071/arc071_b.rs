const MOD: i64 = 1_000_000_007;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let xs: Vec<i64> = (0..n).map(|_| sc.read()).collect();
    let ys: Vec<i64> = (0..m).map(|_| sc.read()).collect();

    let mut top = 0;
    for i in 0..(n - 1) {
        let mut x = xs[i + 1] - xs[i];
        x = (x * (i + 1) as i64) % MOD;
        x = (x * (n - 1 - i) as i64) % MOD;
        top = (top + x) % MOD;
    }

    let mut side = 0;
    for i in 0..(m - 1) {
        let mut y = ys[i + 1] - ys[i];
        y = (y * (i + 1) as i64) % MOD;
        y = (y * (m - 1 - i) as i64) % MOD;
        side = (side + y) % MOD;
    }

    let ans = (top * side) % MOD;
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

