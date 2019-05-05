use std::collections::VecDeque;
fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let m = sc.usize_read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.usize_read() - 1;
        let b = sc.usize_read() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let q = sc.usize_read();
    let queries: Vec<(usize, usize, usize)> = (0..q)
        .map(|_| (sc.usize_read() - 1, sc.usize_read(), sc.usize_read()))
        .collect();

    let mut ans = vec![0; n];
    let mut max_d = vec![-1; n];
    for q in (0..q).rev() {
        let (v, d, c) = queries[q];
        let mut queue = VecDeque::new();
        queue.push_back((v, d as i32));
        while let Some((v, d)) = queue.pop_front() {
            if max_d[v] >= d {
                continue;
            }
            max_d[v] = d;
            if ans[v] == 0 {
                ans[v] = c;
            }
            for &to in &graph[v] {
                queue.push_back((to, d - 1));
            }
        }
    }
    for &c in &ans {
        println!("{}", c);
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
