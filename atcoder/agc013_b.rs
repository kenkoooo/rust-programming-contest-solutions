use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut vis = vec![false; n];
    let mut path = VecDeque::new();
    path.push_back(0);
    path.push_back(graph[0][0]);
    vis[0] = true;
    vis[graph[0][0]] = true;
    loop {
        let start = path.len();
        let head = path.pop_front().unwrap();
        path.push_front(head);
        let tail = path.pop_back().unwrap();
        path.push_back(tail);

        for &v in &graph[head] {
            if !vis[v] {
                vis[v] = true;
                path.push_front(v);
                break;
            }
        }

        for &v in &graph[tail] {
            if !vis[v] {
                vis[v] = true;
                path.push_back(v);
                break;
            }
        }

        if path.len() == start {
            break;
        }
    }

    println!("{}", path.len());
    while !path.is_empty() {
        print!("{} ", path.pop_front().unwrap() + 1);
    }
    println!();
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

