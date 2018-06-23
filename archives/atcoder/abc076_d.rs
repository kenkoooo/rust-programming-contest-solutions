use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let t: Vec<usize> = (0..n).map(|_| sc.read()).collect();
    let v: Vec<usize> = (0..n).map(|_| sc.read()).collect();

    let t_sum = t.iter().sum();
    let mut max_v = vec![0; t_sum + 1];

    let mut pos = 0;
    for i in 0..n {
        let ti = t[i];
        let vi = v[i];
        max_v[pos] = cmp::min(max_v[pos], vi);
        for _ in 0..ti {
            max_v[pos + 1] = cmp::min(max_v[pos] + 1, vi);
            pos += 1;
        }
    }

    pos = t_sum;
    max_v[t_sum] = 0;
    for i in (0..n).rev() {
        let ti = t[i];
        for _ in 0..ti {
            max_v[pos - 1] = cmp::min(max_v[pos] + 1, max_v[pos - 1]);
            pos -= 1;
        }
    }

    let mut ans = 0.0;
    pos = 0;
    for i in 0..n {
        let ti = t[i];
        let vi = v[i];
        for _ in 0..ti {
            let cur = max_v[pos] as f64;
            let next = max_v[pos + 1] as f64;
            ans += cur + (next - cur) / 2.0;
            if cur == next && vi as f64 > cur {
                ans += 0.25;
            }
            pos += 1;
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

