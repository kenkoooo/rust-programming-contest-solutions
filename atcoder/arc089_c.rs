fn main() {
    let mut sc = Scanner::new();
    let a: usize = sc.read();
    let b: usize = sc.read();
    let d: Vec<Vec<usize>> = (0..a).map(|_| (0..b).map(|_| sc.read()).collect()).collect();

    let n = 300;
    let max_dist = 100;
    let max_cost = 100;

    let mut char_edges = Vec::new();
    let xs = (0..(max_dist + 1)).map(|i| i).collect::<Vec<_>>();
    let ys = (0..(max_dist + 1)).map(|i| n - 1 - i).collect::<Vec<_>>();
    for i in 0..max_dist {
        char_edges.push((xs[i], xs[i + 1], b'X'));
        char_edges.push((ys[i + 1], ys[i], b'Y'));
    }

    let mut used = vec![vec![false; b]; a];
    let mut int_edges = Vec::new();
    for x_cnt in 0..xs.len() {
        for y_cnt in 0..ys.len() {
            if x_cnt + y_cnt > max_dist { continue; }

            let mut bridge_cost = 0;
            for x in 0..a {
                for y in 0..b {
                    let base_cost = x_cnt * (x + 1) + y_cnt * (y + 1);
                    if bridge_cost + base_cost < d[x][y] { bridge_cost = d[x][y] - base_cost; }
                }
            }
            if bridge_cost > max_cost { continue; }
            int_edges.push((xs[x_cnt], ys[y_cnt], bridge_cost));
            for x in 0..a {
                for y in 0..b {
                    let base_cost = x_cnt * (x + 1) + y_cnt * (y + 1);
                    assert!(d[x][y] <= base_cost + bridge_cost);
                    if base_cost + bridge_cost == d[x][y] { used[x][y] = true; }
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
    println!("{} {}", n, int_edges.len() + char_edges.len());
    for e in &char_edges {
        let (from, to, c) = *e;
        let x = if c == b'X' { "X" } else { "Y" };
        println!("{} {} {}", from + 1, to + 1, x);
    }
    for e in &int_edges {
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

