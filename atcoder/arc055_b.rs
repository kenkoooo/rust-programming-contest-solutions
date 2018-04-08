fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut dp = vec![vec![vec![0.0; 2]; k + 1]; n + 1];
    let mut vis = vec![vec![vec![false; 2]; k + 1]; n + 1];

    println!("{}", search(0, 0, false, &mut dp, &mut vis));
}

fn search(
    seen: usize,
    eaten: usize,
    max_eaten: bool,
    dp: &mut Vec<Vec<Vec<f64>>>,
    vis: &mut Vec<Vec<Vec<bool>>>) -> f64 {
    let total = dp.len() - 1;
    let can_eat = dp[0].len() - 1;

    if seen == total {
        return if max_eaten { 1.0 } else { 0.0 };
    }

    let max_eaten = if max_eaten { 1 } else { 0 };

    if vis[seen][eaten][max_eaten] {
        return dp[seen][eaten][max_eaten];
    }

    let mut result = 0.0;

    let current_max_appeared = 1.0 / (seen + 1) as f64;
    let non_max = 1.0 - current_max_appeared;
    if eaten < can_eat {
        let skip = search(seen + 1, eaten, false, dp, vis);
        let eat = search(seen + 1, eaten + 1, true, dp, vis);
        result += current_max_appeared * max(skip, eat);
    }
    result += non_max * search(seen + 1, eaten, max_eaten == 1, dp, vis);

    vis[seen][eaten][max_eaten] = true;
    dp[seen][eaten][max_eaten] = result;
    return result;
}

fn max(a: f64, b: f64) -> f64 {
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

