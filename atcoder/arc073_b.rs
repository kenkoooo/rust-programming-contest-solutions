use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let w: usize = sc.read();
    let (vs, base_w) = {
        let wv: Vec<(usize, usize)> = (0..n).map(|_| (sc.read(), sc.read())).collect();
        let (w0, _) = wv[0];
        let mut vs = vec![vec![]; 4];
        for t in &wv {
            let (w, v) = *t;
            vs[w - w0].push(v);
        }
        for i in 0..4 {
            vs[i].sort();
            vs[i].reverse();
        }
        (vs, w0)
    };
    let (w0, w1, w2, w3) = (vs[0].len(), vs[1].len(), vs[2].len(), vs[3].len());


    let mut dp = vec![vec![vec![vec![0; w3 + 1]; w2 + 1]; w1 + 1]; w0 + 1];
    for i0 in 0..(w0 + 1) {
        for i1 in 0..(w1 + 1) {
            for i2 in 0..(w2 + 1) {
                for i3 in 0..(w3 + 1) {
                    if i0 < w0 {
                        dp[i0 + 1][i1][i2][i3] = cmp::max(dp[i0 + 1][i1][i2][i3], dp[i0][i1][i2][i3] + vs[0][i0])
                    }
                    if i1 < w1 {
                        dp[i0][i1 + 1][i2][i3] = cmp::max(dp[i0][i1 + 1][i2][i3], dp[i0][i1][i2][i3] + vs[1][i1])
                    }
                    if i2 < w2 {
                        dp[i0][i1][i2 + 1][i3] = cmp::max(dp[i0][i1][i2 + 1][i3], dp[i0][i1][i2][i3] + vs[2][i2])
                    }
                    if i3 < w3 {
                        dp[i0][i1][i2][i3 + 1] = cmp::max(dp[i0][i1][i2][i3 + 1], dp[i0][i1][i2][i3] + vs[3][i3])
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i0 in 0..(w0 + 1) {
        for i1 in 0..(w1 + 1) {
            for i2 in 0..(w2 + 1) {
                for i3 in 0..(w3 + 1) {
                    if (i0 + i1 + i2 + i3) * base_w + i1 + 2 * i2 + 3 * i3 > w { continue; }
                    ans = cmp::max(ans, dp[i0][i1][i2][i3]);
                }
            }
        }
    }

    println!("{}", ans);
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

