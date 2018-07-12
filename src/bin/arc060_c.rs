use std::collections::VecDeque;

fn doubling(x: &Vec<usize>, l: usize) -> Vec<Vec<usize>> {
    let n = x.len();
    let mut t = vec![vec![0; 32]; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    for i in 1..n {
        while let Some(h) = q.pop_front() {
            if x[i] - x[h] > l {
                t[h][0] = i - 1;
            } else {
                q.push_front(h);
                break;
            }
        }
        q.push_back(i);
    }
    while let Some(h) = q.pop_front() {
        t[h][0] = n - 1;
    }

    for bit in 0..31 {
        for i in 0..n {
            t[i][bit + 1] = t[t[i][bit]][bit];
        }
    }
    t
}

fn query(t: &Vec<Vec<usize>>, a: usize, b: usize) -> usize {
    let mut cur = a;
    let mut ans = 0;
    for i in (0..32).rev() {
        if t[cur][i] < b {
            cur = t[cur][i];
            ans += (1 << i);
        }
    }
    ans + 1
}

fn check(day: usize, from: usize, t: &Vec<Vec<usize>>) -> usize {
    let mut cur = from;
    for i in 0..32 {
        if day & (1 << i) != 0 {
            cur = t[cur][i];
        }
    }
    cur
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let x: Vec<usize> = sc.read_vec(n);
    let l = sc.usize_read();

    let forward = doubling(&x, l);
    let x = x.iter().rev().map(|y| x[n - 1] - y).collect();
    let backward = doubling(&x, l);

    let q: usize = sc.read();
    for _ in 0..q {
        let a = sc.usize_read() - 1;
        let b = sc.usize_read() - 1;
        let ans = if a < b {
            query(&forward, a, b)
        } else {
            query(&backward, n - 1 - a, n - 1 - b)
        };
        println!("{}", ans);
    }
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
