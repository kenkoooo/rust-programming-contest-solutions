fn main() {
    let mut sc = Scanner::new();
    let n = sc.read::<usize>();
    let k = sc.read::<usize>();

    let mut vis = vec![vec![vec![false; 2]; k + 1]; n + 1];
    let mut mem = vec![vec![vec![0.0; 2]; k + 1]; n + 1];

    println!("{}", search(0, 0, false, &mut mem, &mut vis));
}

fn search(n: usize, k: usize, b: bool, mem: &mut Vec<Vec<Vec<f64>>>, vis: &mut Vec<Vec<Vec<bool>>>) -> f64 {
    let total = mem.len() - 1;
    let take = mem[0].len() - 1;
    if n == total {
        return if b { 1.0 } else { 0.0 };
    }
    let b = if b { 1 } else { 0 };
    if vis[n][k][b] {
        return mem[n][k][b];
    }
    vis[n][k][b] = true;
    let mut result = 0.0;
    let p1 = 1.0 / (n + 1) as f64;
    let p2 = 1.0 - p1;

    let skip = search(n + 1, k, false, mem, vis);
    if k + 1 <= take {
        result += p1 * max_f64(skip, search(n + 1, k + 1, true, mem, vis));
    }
    result += p2 * search(n + 1, k, b == 1, mem, vis);

    mem[n][k][b] = result;
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

