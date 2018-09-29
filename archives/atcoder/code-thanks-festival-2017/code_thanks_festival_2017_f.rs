const MOD: usize = 1_000_000_007;

fn mod_pow(x: usize, e: usize) -> usize {
    let mut cur = x;
    let mut res = 1;
    let mut e = e;
    while e > 0 {
        if e & 1 != 0 {
            res = (res * cur) % MOD;
        }
        cur = (cur * cur) % MOD;
        e >>= 1;
    }
    res
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let k: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let mut count = vec![0; 100001];
    for &a in &a {
        count[a] += 1;
    }

    let mut pairs = vec![];
    for i in 0..count.len() {
        if count[i] > 0 {
            pairs.push((i, count[i]));
        }
    }

    let mut dp = vec![0; 100001];
    dp[0] = 1;
    for &(value, count) in &pairs {
        let mut next = vec![0; 100001];
        for i in 0..100001 {
            if dp[i] == 0 {
                continue;
            }
            next[i] += dp[i] * mod_pow(2, count - 1);
            next[i] %= MOD;
            next[i ^ value] += dp[i] * mod_pow(2, count - 1);
            next[i ^ value] %= MOD;
        }
        dp = next;
    }

    println!("{}", dp[k]);
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
