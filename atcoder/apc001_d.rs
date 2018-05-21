use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    let a: Vec<usize> = (0..n).map(|_| sc.read()).collect();

    for _ in 0..m {
        let x: usize = sc.read();
        let y: usize = sc.read();
        graph[x].push(y);
        graph[y].push(x);
    }

    let mut queue = VecDeque::new();
    let mut used = vec![false; n];
    let mut groups = Vec::new();
    for i in 0..n {
        if used[i] {
            continue;
        }
        queue.push_back(i);
        used[i] = true;
        let mut group = vec![a[i]];
        while let Some(v) = queue.pop_front() {
            for &to in &graph[v] {
                if used[to] {
                    continue;
                }
                used[to] = true;
                queue.push_back(to);
                group.push(a[to]);
            }
        }

        group.sort();
        groups.push(group);
    }

    let mut required = (n - 1 - m) * 2;
    if required == 0 {
        println!("0");
        return;
    }
    let mut ans = 0;
    let mut rest = Vec::new();
    for &ref v in &groups {
        ans += v[0];
        required -= 1;
        for i in 1..v.len() {
            rest.push(v[i]);
        }
    }
    if required > rest.len() {
        println!("Impossible");
        return;
    }

    rest.sort();
    for i in 0..required {
        ans += rest[i];
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
