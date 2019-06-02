use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut pos_b = vec![0; n];
    let mut pos_w = vec![0; n];
    for i in 0..(2 * n) {
        let c = sc.read::<String>();
        let a = sc.read::<usize>() - 1;
        if c == "W" {
            pos_w[a] = i;
        } else {
            pos_b[a] = i;
        }
    }

    let calc_right = |pos: &Vec<usize>| {
        let mut right = vec![0; n];
        for i in 0..n {
            let mut sum = 0;
            for j in 0..i {
                if pos[i] < pos[j] {
                    sum += 1;
                }
            }
            right[i] = sum;
        }
        right
    };

    let right_b = calc_right(&pos_b);
    let right_w = calc_right(&pos_w);

    let calc_right_other = |pos: &Vec<usize>, other: &Vec<usize>| {
        let mut right = vec![vec![0; n + 1]; n];
        for i in 0..n {
            let mut sum = 0;
            for j in 0..n {
                if pos[i] < other[j] {
                    sum += 1;
                }
                right[i][j + 1] = sum;
            }
        }
        right
    };

    let other_b = calc_right_other(&pos_b, &pos_w);
    let other_w = calc_right_other(&pos_w, &pos_b);

    let inf = std::usize::MAX;
    let mut dp = vec![vec![inf; n + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..(n + 1) {
        for j in 0..(n + 1) {
            if dp[i][j] == inf {
                continue;
            }
            if i + 1 <= n {
                dp[i + 1][j] = cmp::min(
                    dp[i + 1][j],
                    dp[i][j] + pos_b[i] + right_b[i] + other_b[i][j] - (i + j),
                );
            }

            if j + 1 <= n {
                dp[i][j + 1] = cmp::min(
                    dp[i][j + 1],
                    dp[i][j] + pos_w[j] + right_w[j] + other_w[j][i] - (i + j),
                );
            }
        }
    }

    println!("{}", dp[n][n]);
}

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
