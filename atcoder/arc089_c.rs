const MAX_D: usize = 100;
const MAX_C: usize = 100;
const MAX_V: usize = 300;

fn main() {
    let mut sc = Scanner::new();
    let a: usize = sc.read();
    let b: usize = sc.read();
    let dist: Vec<Vec<usize>> = (0..a).map(|_| (0..b).map(|_| sc.read()).collect()).collect();

    let n = MAX_V;

    let mut char_edge = Vec::new();
    let xs = (0..(MAX_D + 1)).map(|i| i).collect::<Vec<_>>();
    let ys = (0..(MAX_D + 1)).map(|i| n - 1 - i).collect::<Vec<_>>();
    for i in 0..MAX_D {
        char_edge.push((xs[i], xs[i + 1], 'X'));
        char_edge.push((ys[i + 1], ys[i], 'Y'));
    }

    let mut used = vec![vec![false; b]; a];
    let mut int_edge = Vec::new();
    for from in 0..(MAX_D + 1) {
        for to in 0..(MAX_D + 1) {
            if from + to > MAX_D {
                continue;
            }
            let mut edge_cost = 0;
            for i in 0..a {
                for j in 0..b {
                    let cost_x = i + 1;
                    let cost_y = j + 1;
                    let base_cost = from * cost_x + to * cost_y;
                    if edge_cost + base_cost < dist[i][j] {
                        edge_cost = dist[i][j] - base_cost;
                    }
                }
            }
            if edge_cost > MAX_C { continue; }
            int_edge.push((xs[from], ys[to], edge_cost));

            for i in 0..a {
                for j in 0..b {
                    let cost_x = i + 1;
                    let cost_y = j + 1;
                    let base_cost = from * cost_x + to * cost_y;
                    assert!(dist[i][j] <= base_cost + edge_cost);
                    if base_cost + edge_cost == dist[i][j] { used[i][j] = true; }
                }
            }
        }
    }

    for x in 0..a {
        for y in 0..b {
            if !used[x][y] {
                println!("Impossible");
                return;
            }
        }
    }

    println!("Possible");
    println!("{} {}", n, int_edge.len() + char_edge.len());
    for e in &char_edge {
        let (from, to, c) = *e;
        println!("{} {} {}", from + 1, to + 1, c);
    }
    for e in &int_edge {
        let (from, to, c) = *e;
        println!("{} {} {}", from + 1, to + 1, c);
    }
    println!("{} {}", xs[0] + 1, ys[0] + 1);
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
        while Scanner::is_space(b) { b = self.byte(); }

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

