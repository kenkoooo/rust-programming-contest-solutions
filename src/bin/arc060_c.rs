use std::collections::VecDeque;

const MAX_BIT: usize = 32;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let x: Vec<usize> = sc.read_vec(n);
    let l = sc.usize_read();
    let mut go = vec![vec![0; n]; MAX_BIT];
    let mut q = VecDeque::new();
    q.push_back(0);

    for i in 1..n {
        while !q.is_empty() {
            let head = q.pop_front().unwrap();
            let dx = x[i] - x[head];
            if dx > l {
                go[0][head] = i - 1;
            } else {
                q.push_front(head);
                break;
            }
        }
        q.push_back(i);
    }
    while let Some(head) = q.pop_front() {
        go[0][head] = n - 1;
    }

    for b in 1..MAX_BIT {
        for i in 0..n {
            go[b][i] = go[b - 1][go[b - 1][i]];
        }
    }

    let q = sc.read();
    for _ in 0..q {
        let a = sc.usize_read() - 1;
        let b = sc.usize_read() - 1;
        let (a, b) = if a > b { (b, a) } else { (a, b) };
        let mut cur = a;
        let mut ng = 0;
        for bit in (0..MAX_BIT).rev() {
            if go[bit][cur] < b {
                ng += (1 << bit);
                cur = go[bit][cur];
            }
        }
        println!("{}", ng + 1);
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
