fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.read();
    let w: usize = sc.read();
    let d: usize = sc.read();
    let a: Vec<Vec<usize>> = (0..h).map(|_| {
        (0..w).map(|_| sc.read()).collect()
    }).collect();

    let mut place = vec![(0, 0); h * w + 1];
    for i in 0..h {
        for j in 0..w {
            let x = a[i][j];
            place[x] = (i, j);
        }
    }

    let mut sum = vec![0; h * w + 1];

    for e in 1..(d + 1) {
        let mut cur = e;
        while cur + d <= h * w {
            let (fi, fj) = place[cur];
            let (ti, tj) = place[cur + d];
            let ei = if fi > ti { fi - ti } else { ti - fi };
            let ej = if fj > tj { fj - tj } else { tj - fj };

            sum[cur + d] = sum[cur] + (ei + ej);
            cur += d;
        }
    }


    let q: usize = sc.read();
    for _ in 0..q {
        let l: usize = sc.read();
        let r: usize = sc.read();
        let a = sum[r] - sum[l];
        println!("{}", a);
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

