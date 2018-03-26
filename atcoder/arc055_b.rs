fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let k = sc.read::<usize>();

    let mut mem = vec![vec![vec![0.0; 2]; k + 1]; n + 1];
    let mut vis = vec![vec![vec![false; 2]; k + 1]; n + 1];

    println!("{}", search(0, 0, false, &mut mem, &mut vis, n, k));
}

fn search(
    i: usize,
    j: usize,
    b: bool,
    mem: &mut Vec<Vec<Vec<f64>>>,
    vis: &mut Vec<Vec<Vec<bool>>>,
    n: usize,
    k: usize) -> f64 {
    if i == k {
        return if b { 1.0 } else { 0.0 };
    }

    let b = if b { 1 } else { 0 };
    if vis[i][j][b] {
        return mem[i][j][b];
    }
    vis[i][j][b] = true;

    let mut result = 0.0;
    let p1 = 1.0 / (i + 1) as f64;
    let p2 = 1.0 - p1;

    if j + 1 <= k {
        result += p1 * max_f64(
            search(i + 1, j, false, mem, vis, n, k),
            search(i + 1, j + 1, true, mem, vis, n, k),
        );
    }
    result += p2 * search(i + 1, j, b == 1, mem, vis, n, k);

    mem[i][j][b] = result;
    return result;
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
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

