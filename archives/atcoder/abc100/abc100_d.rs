fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let m = sc.usize_read();
    let xyz: Vec<(i64, i64, i64)> = (0..n).map(|_| (sc.read(), sc.read(), sc.read())).collect();

    println!(
        "{}",
        (0..8).map(|mask| maximize(&xyz, mask, m)).max().unwrap()
    );
}

fn maximize(xyz: &Vec<(i64, i64, i64)>, mask: usize, m: usize) -> i64 {
    let mut v = xyz.iter()
        .map(|&(x, y, z)| {
            let x = if mask & (1 << 0) != 0 { -x } else { x };
            let y = if mask & (1 << 1) != 0 { -y } else { y };
            let z = if mask & (1 << 2) != 0 { -z } else { z };
            (x + y + z, x, y, z)
        })
        .collect::<Vec<_>>();
    v.sort();
    v.reverse();

    let mut sx = 0;
    let mut sy = 0;
    let mut sz = 0;
    for i in 0..m {
        let (_, x, y, z) = v[i];
        sx += x;
        sy += y;
        sz += z;
    }
    sx.abs() + sy.abs() + sz.abs()
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
