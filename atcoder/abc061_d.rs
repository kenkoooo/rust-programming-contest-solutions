fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let c = sc.read::<i64>();
        graph[a].push((b, -c));
    }

    let (dist, negative) = shortest_path(&graph, 0, std::i64::MAX);
    if negative[n - 1] {
        println!("inf");
    } else {
        println!("{}", -dist[n - 1]);
    }
}

pub fn shortest_path(
    graph: &Vec<Vec<(usize, i64)>>,
    start: usize,
    inf: i64,
) -> (Vec<i64>, Vec<bool>) {
    let n = graph.len();
    let mut dist = vec![inf; n];
    dist[start] = 0;
    for _ in 0..n {
        for v in 0..n {
            for &(to, cost) in &graph[v] {
                if dist[v] == inf || dist[to] <= dist[v] + cost {
                    continue;
                }
                dist[to] = dist[v] + cost;
            }
        }
    }

    let mut negative = vec![false; n];
    for _ in 0..n {
        for v in 0..n {
            for &(to, cost) in &graph[v] {
                if dist[v] == inf {
                    continue;
                }
                if dist[to] > dist[v] + cost {
                    dist[to] = dist[v] + cost;
                    negative[to] = true;
                }
                if negative[v] {
                    negative[to] = true;
                }
            }
        }
    }

    return (dist, negative);
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
