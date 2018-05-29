use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let k: usize = sc.read();
    let q: usize = sc.read();
    let a: Vec<i64> = sc.read_vec(n);
    let mut ans = std::i64::MAX;
    for &min in &a {
        let mut escape = Vec::new();
        for i in 0..n {
            if a[i] < min {
                escape.push(i);
            }
        }
        escape.push(n);

        let extract = |from: usize, to: usize| -> Vec<i64> {
            if from > to || to - from + 1 < k {
                return Vec::new();
            }

            let mut v = Vec::new();
            for i in from..(to + 1) {
                v.push(a[i]);
            }
            v.sort();
            let removable = to - from + 1 - (k - 1);
            return (0..removable).map(|i| v[i]).collect();
        };

        let mut max = Vec::new();
        if escape[0] > 0 {
            max.append(&mut extract(0, escape[0] - 1));
        }
        for i in 0..(escape.len() - 1) {
            max.append(&mut extract(escape[i] + 1, escape[i + 1] - 1));
        }
        max.sort();
        if max.len() < q {
            continue;
        }
        ans = cmp::min(ans, max[q - 1] - min);
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
