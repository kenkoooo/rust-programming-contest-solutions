use std::cmp;
fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let c = sc.usize_read();
    let d: Vec<Vec<i32>> = (0..c).map(|_| sc.read_vec(c)).collect();
    let cc: Vec<Vec<usize>> = (0..n).map(|_| sc.read_vec(n)).collect();

    let mut count = vec![vec![0; c + 1]; 3];
    for i in 0..n {
        for j in 0..n {
            count[(i + j) % 3][cc[i][j] - 1] += 1;
        }
    }

    let mut ans = std::i32::MAX;
    for c1 in 0..c {
        for c2 in 0..c {
            if c1 == c2 {
                continue;
            }
            for c3 in 0..c {
                if c1 == c3 || c2 == c3 {
                    continue;
                }
                let mut tmp = 0;
                let next = vec![c1, c2, c3];
                for i in 0..3 {
                    for prev in 0..c {
                        tmp += count[i][prev] * d[prev][next[i]];
                    }
                }

                ans = cmp::min(ans, tmp);
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
