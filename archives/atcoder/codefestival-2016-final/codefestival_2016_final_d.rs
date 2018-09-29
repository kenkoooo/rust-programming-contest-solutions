use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let x: Vec<usize> = sc.read_vec(n);
    let max: usize = x.iter().map(|&x| x).max().unwrap();
    let mut count = vec![0; max + 1];
    for &x in &x {
        count[x] += 1;
    }

    let mut mod_count = vec![0; m];
    let mut pair_count = vec![0; m];
    for i in 0..(max + 1) {
        mod_count[i % m] += count[i];
        pair_count[i % m] += count[i] / 2;
    }

    let mut ans = 0;
    for i in 1..m {
        if i * 2 == m {
            continue;
        }
        let p = cmp::min(mod_count[i], mod_count[m - i]);
        ans += p;
        mod_count[i] -= p;
        mod_count[m - i] -= p;
    }

    if m % 2 == 0 {
        ans += mod_count[m / 2] / 2;
        mod_count[m / 2] %= 2;
    }

    for i in 1..m {
        ans += cmp::min(mod_count[i] / 2, pair_count[i]);
    }
    ans += mod_count[0] / 2;

    println!("{}", ans);
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
