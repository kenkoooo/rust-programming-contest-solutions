fn main() {
    let mut sc = Scanner::new();
    let (sx, sy): (f64, f64) = (sc.read(), sc.read());
    let (tx, ty): (f64, f64) = (sc.read(), sc.read());
    let n: usize = sc.read();
    let mut v = vec![(sx, sy, 0.0)];
    for _ in 0..n {
        let x: f64 = sc.read();
        let y: f64 = sc.read();
        let r: f64 = sc.read();
        v.push((x, y, r));
    }
    v.push((tx, ty, 0.0));
    let n = v.len();

    let mut dist = vec![std::f64::INFINITY; n];
    let mut vis = vec![false; n];
    dist[0] = 0.0;
    for _ in 0..n {
        let mut now = n;
        for i in 0..n {
            if vis[i] {
                continue;
            }
            if now == n {
                now = i;
            } else if dist[now] > dist[i] {
                now = i;
            }
        }
        vis[now] = true;
        let (x, y, r) = v[now];
        for i in 0..n {
            let (tx, ty, tr) = v[i];
            let dx = tx - x;
            let dy = ty - y;
            let d = (dx * dx + dy * dy).sqrt() - tr - r;
            let d = if d > 0.0 { d } else { 0.0 };
            if dist[i] > dist[now] + d {
                dist[i] = dist[now] + d;
            }
        }
    }

    println!("{}", dist[n - 1]);
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
