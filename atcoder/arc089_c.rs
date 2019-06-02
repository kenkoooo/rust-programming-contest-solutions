use std::cmp;

const MAX_V: usize = 300;
const MAX_C: usize = 100;

fn main() {
    let mut sc = Scanner::new();
    let a: usize = sc.read();
    let b: usize = sc.read();
    let dist_table: Vec<Vec<usize>> = (0..a)
        .map(|_| (0..b).map(|_| sc.read()).collect())
        .collect();

    let n = MAX_V;
    let mut used = vec![vec![false; b]; a];
    let mut edges = Vec::new();
    for x_count in 0..(MAX_C + 1) {
        for y_count in 0..(MAX_C + 1) {
            let mut edge = 0;
            for i in 0..a {
                for j in 0..b {
                    let dx = i + 1;
                    let dy = j + 1;
                    let base = dx * x_count + dy * y_count;
                    if dist_table[i][j] < base {
                        continue;
                    }
                    edge = cmp::max(edge, dist_table[i][j] - base);
                }
            }

            for i in 0..a {
                for j in 0..b {
                    let dx = i + 1;
                    let dy = j + 1;
                    let base = dx * x_count + dy * y_count;
                    if edge + base == dist_table[i][j] {
                        used[i][j] = true;
                    }
                }
            }

            let from = x_count;
            let to = n - 1 - y_count;
            edges.push((from, to, edge));
        }
    }

    for i in 0..a {
        for j in 0..b {
            if !used[i][j] {
                println!("Impossible");
                return;
            }
        }
    }

    println!("Possible");
    println!("{} {}", n, edges.len() + MAX_C * 2);
    for i in 0..MAX_C {
        println!("{} {} X", i + 1, i + 2);
    }
    for i in 0..MAX_C {
        println!("{} {} Y", n - 1 - i, n - i);
    }
    for &(from, to, cost) in &edges {
        println!("{} {} {}", from + 1, to + 1, cost);
    }
    println!("{} {}", 1, n);
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
