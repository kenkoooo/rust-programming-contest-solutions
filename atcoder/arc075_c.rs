use std::collections::BTreeSet;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: i64 = sc.read();
    let a: Vec<i64> = sc.read_vec(n);

    let mut set = BTreeSet::new();
    let mut b = a.clone();
    for i in 0..n {
        if i > 0 {
            b[i] += b[i - 1];
        }
        b[i] -= k;
        set.insert(b[i]);
    }

    let set: Vec<i64> = set.iter().map(|&c| c).collect();
    let c: Vec<usize> = b.iter().map(|&b| set.binary_search(&b).unwrap()).collect();

    let mut ans = 0;
    let mut bit = FenwickTree::new(n);
    for &c in &c {
        ans += bit.sum_one(c + 1);
        bit.add(c, 1);
    }

    for &b in &b {
        if b >= 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

pub struct FenwickTree {
    n: usize,
    data: Vec<usize>,
}

impl FenwickTree {
    /// Constructs a new `FenwickTree`. The size of `FenwickTree` should be specified by `size`.
    pub fn new(size: usize) -> FenwickTree {
        FenwickTree {
            n: size + 1,
            data: vec![0; size + 1],
        }
    }

    fn add(&mut self, k: usize, value: usize) {
        let mut x = k;
        while x < self.n {
            self.data[x] += value;
            x |= x + 1;
        }
    }

    /// Returns a sum of range `[l, r)`
    pub fn sum(&self, l: usize, r: usize) -> usize {
        return self.sum_one(r) - self.sum_one(l);
    }

    /// Returns a sum of range `[0, k)`
    pub fn sum_one(&self, k: usize) -> usize {
        if k >= self.n {
            panic!("");
        }

        let mut result = 0;
        let mut x = k as i32 - 1;
        while x >= 0 {
            result += self.data[x as usize];
            x = (x & (x + 1)) - 1;
        }

        return result;
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
