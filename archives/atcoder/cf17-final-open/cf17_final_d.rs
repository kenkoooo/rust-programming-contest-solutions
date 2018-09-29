use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let mut hp: Vec<(usize, usize)> = (0..n).map(|_| (sc.read(), sc.read())).collect();
    hp.sort_by_key(|&(h, p)| h + p);

    let inf = hp.iter().map(|&(_, p)| p).sum::<usize>() + 1;

    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (h, p) = hp[i];
        for j in 0..n {
            dp[i + 1][j] = cmp::min(dp[i][j], dp[i + 1][j]);
            if h >= dp[i][j] {
                dp[i + 1][j + 1] = cmp::min(dp[i + 1][j + 1], dp[i][j] + p);
            }
        }
    }

    let mut max = 0;
    for i in 0..n {
        if dp[n][i + 1] < inf {
            max = i + 1;
        }
    }

    println!("{}", max);
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
