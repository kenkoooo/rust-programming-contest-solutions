use std::collections::VecDeque;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();

    let mut graph: Vec<Vec<usize>> = (0..n).map(|_| Vec::new()).collect();
    for _ in 0..(n - 1) {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let dist_first = calc_dist(0, n, &graph);
    let dist_second = calc_dist(n - 1, n, &graph);
    let mut first = 0;
    let mut second = 0;
    for i in 0..n {
        if dist_first[i] <= dist_second[i] { first += 1; } else { second += 1; }
    }

    if first > second {
        println!("Fennec");
    } else { println!("Snuke"); }
}

fn calc_dist(start: usize, n: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut q = VecDeque::new();
    let mut dist = vec![std::usize::MAX; n];
    dist[start] = 0;
    q.push_back(start);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for next in &graph[v] {
            if dist[*next] <= dist[v] { continue; }
            dist[*next] = dist[v] + 1;
            q.push_back(*next);
        }
    }
    return dist;
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

