fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let mut dp = vec![vec![vec![0.0; 2]; k + 1]; n + 1];
    let mut vis = vec![vec![vec![false; 2]; k + 1]; n + 1];
    println!("{}", search(&mut dp, &mut vis, false, n, k));
}

fn search(
    dp: &mut Vec<Vec<Vec<f64>>>,
    vis: &mut Vec<Vec<Vec<bool>>>,
    have_max: bool,
    rest: usize, taking: usize) -> f64 {
    if rest == 0 {
        return if have_max { 1.0 } else { 0.0 };
    }
    let have_max = if have_max { 1 } else { 0 };
    if vis[rest][taking][have_max] {
        return dp[rest][taking][have_max];
    }

    let n = dp.len() - 1;
    let turn = n - rest + 1;
    let max_appear = 1.0 / (turn as f64);

    let p = if taking > 0 {
        max_appear * max(
            search(dp, vis, true, rest - 1, taking - 1),
            search(dp, vis, false, rest - 1, taking),
        )
    } else { 0.0 } + (1.0 - max_appear) * search(dp, vis, have_max == 1, rest - 1, taking);

    vis[rest][taking][have_max] = true;
    dp[rest][taking][have_max] = p;

    return p;
}

fn max(x: f64, y: f64) -> f64 { if x > y { x } else { y } }

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

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
