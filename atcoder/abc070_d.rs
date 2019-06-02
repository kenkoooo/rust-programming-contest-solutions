use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut graph: Vec<Vec<(usize, usize)>> = (0..n).map(|_| Vec::new()).collect();
    for _ in 0..(n - 1) {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read::<usize>();
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let q: usize = sc.read();
    let k: usize = sc.read::<usize>() - 1;

    let mut dist = vec![std::usize::MAX; n];
    dist[k] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(k);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for e in &graph[v] {
            let (to, cost) = *e;
            if dist[to] <= dist[v] + cost { continue; }
            dist[to] = dist[v] + cost;
            queue.push_back(to);
        }
    }

    for _ in 0..q {
        let x = sc.read::<usize>() - 1;
        let y = sc.read::<usize>() - 1;
        println!("{}", dist[x] + dist[y]);
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

