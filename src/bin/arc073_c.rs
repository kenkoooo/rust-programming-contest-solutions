use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let mut x: Vec<usize> = vec![0; n];
    let mut y: Vec<usize> = vec![0; n];
    for i in 0..n {
        x[i] = sc.read();
        y[i] = sc.read();
    }

    let mut small = vec![];
    let mut large = vec![];
    for i in 0..n {
        small.push(cmp::min(x[i], y[i]));
        large.push(cmp::max(x[i], y[i]));
    }

    let &small_min = small.iter().min().unwrap();
    let &small_max = small.iter().max().unwrap();
    let &large_min = large.iter().min().unwrap();
    let &large_max = large.iter().max().unwrap();
    let mut ans = (small_max - small_min) * (large_max - large_min);

    let worst_side = large_max - small_min;
    let mut max = small_max;
    let mut min = 1e15 as usize;

    let mut v = vec![];
    for i in 0..n {
        v.push((small[i], large[i]));
    }
    v.sort();

    for i in 0..(n - 1) {
        let (_, large) = v[i];
        let (next_small, _) = v[i + 1];
        min = cmp::min(min, large);
        max = cmp::max(max, large);
        let tmp_min = cmp::min(min, next_small);
        ans = cmp::min(ans, (max - tmp_min) * worst_side);
    }
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
