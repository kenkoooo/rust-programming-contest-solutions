use std::collections::BTreeMap;

const MOD: usize = 1_000_000_007;
const MAX_A: usize = 100000;

fn mod_pow(x: usize, e: usize, modulo: usize) -> usize {
    let mut result = 1;
    let mut cur = x;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result *= cur;
            result %= modulo;
        }
        cur *= cur;
        cur %= modulo;
        e >>= 1;
    }
    result
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let k: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let mut count = vec![0; MAX_A + 1];
    for &a in &a {
        count[a] += 1;
    }

    let b: Vec<usize> = (0..(MAX_A + 1)).filter(|&i| count[i] > 0).collect();

    let mut state = BTreeMap::new();
    state.insert(0, 1);
    for &b in &b {
        let count = count[b];
        let mut next = BTreeMap::new();
        for (&key, &value) in state.iter() {
            let next_key = key ^ b;

            match next.get(&key) {
                Some(&v) => next.insert(key, (v + value * mod_pow(2, count - 1, MOD) % MOD)),
                None => next.insert(key, (value * mod_pow(2, count - 1, MOD)) % MOD),
            };
            match next.get(&next_key) {
                Some(&v) => next.insert(next_key, (v + value * mod_pow(2, count - 1, MOD) % MOD)),
                None => next.insert(next_key, (value * mod_pow(2, count - 1, MOD)) % MOD),
            };
        }
        state = next;
    }

    match state.get(&k) {
        Some(&v) => println!("{}", v),
        None => println!("0"),
    }
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
