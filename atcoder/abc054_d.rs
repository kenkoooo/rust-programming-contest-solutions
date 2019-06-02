use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let ma: usize = sc.read();
    let mb: usize = sc.read();
    let drugs: Vec<(usize, usize, usize)> = (0..n).map(|_| (sc.read(), sc.read(), sc.read())).collect();

    let a_sum: usize = drugs.iter().map(|t| {
        let (a, _, _) = *t;
        a
    }).sum();
    let b_sum: usize = drugs.iter().map(|t| {
        let (_, b, _) = *t;
        b
    }).sum();

    let inf = std::usize::MAX;
    let mut dp = vec![vec![inf; b_sum + 1]; a_sum + 1];
    dp[0][0] = 0;
    for t in &drugs {
        let (a, b, c) = *t;
        for from_a in (0..a_sum).rev() {
            for from_b in (0..b_sum).rev() {
                if from_a + a > a_sum || from_b + b > b_sum {
                    continue;
                }
                if dp[from_a][from_b] == inf {
                    continue;
                }
                dp[from_a + a][from_b + b] = cmp::min(dp[from_a + a][from_b + b], dp[from_a][from_b] + c);
            }
        }
    }

    let mut ans = inf;
    let mut cur = 1;
    while ma * cur <= a_sum && mb * cur <= b_sum {
        ans = cmp::min(ans, dp[ma * cur][mb * cur]);
        cur += 1;
    }
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
    }
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

