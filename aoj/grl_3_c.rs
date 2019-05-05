fn main() {
    let mut sc = Scanner::new();
    let v: usize = sc.read();
    let e: usize = sc.read();
    let mut graph = vec![vec![]; v];
    for _ in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();
        graph[s].push(t);
    }

    let cmp = strongly_connected_components::decompose(&graph);
    let q: usize = sc.read();
    for _ in 0..q {
        let u: usize = sc.read();
        let v: usize = sc.read();
        if cmp[u] == cmp[v] {
            println!("1");
        } else {
            println!("0");
        }
    }
}


pub mod strongly_connected_components {
    use std::collections::VecDeque;

    pub fn decompose(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut vs = Vec::new();
        let num_v = graph.len();
        let mut cmp = vec![0; num_v];

        let mut reverse_graph = vec![vec![]; num_v];
        for i in 0..num_v {
            for v in &graph[i] {
                reverse_graph[*v].push(i);
            }
        }
        let mut used = vec![false; num_v];

        let mut stack = VecDeque::new();
        let mut added = vec![false; num_v];
        for i in 0..num_v {
            if used[i] { continue; }
            stack.push_front(i);
            while !stack.is_empty() {
                let v = stack.pop_front().unwrap();
                stack.push_front(v);
                used[v] = true;
                let mut pushed = false;
                for j in (0..graph[v].len()).rev() {
                    let u = graph[v][j];
                    if !used[u] {
                        stack.push_front(u);
                        pushed = true;
                    }
                }
                if !pushed {
                    stack.pop_front();
                    if !added[v] {
                        vs.push(v);
                        added[v] = true;
                    }
                }
            }
        }

        used = vec![false; num_v];
        let mut k = 0;
        vs.reverse();
        for i in &vs {
            let i = *i;
            if used[i] { continue; }
            stack.push_front(i);
            used[i] = true;
            cmp[i] = k;
            while !stack.is_empty() {
                let v = stack.pop_front().unwrap();
                stack.push_front(v);
                let mut pushed = false;
                for u in &reverse_graph[v] {
                    let u = *u;
                    if used[u] { continue; }
                    used[u] = true;
                    cmp[u] = k;
                    stack.push_front(u);
                    pushed = true;
                }
                if !pushed {
                    stack.pop_front();
                }
            }
            k += 1;
        }

        return cmp;
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

