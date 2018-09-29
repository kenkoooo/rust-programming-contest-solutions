use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();
    let mut inverted_graph = vec![vec![true; n]; n];
    for _ in 0..m {
        let a = sc.usize_read() - 1;
        let b = sc.usize_read() - 1;
        inverted_graph[a][b] = false;
        inverted_graph[b][a] = false;
    }

    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if inverted_graph[i][j] {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let mut colors = vec![];
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        let mut connected = vec![];
        color[i] = 1;
        if !coloring_dfs(i, &mut connected, &graph, &mut color) {
            println!("-1");
            return;
        }

        let mut count = 0;
        for &c in &connected {
            if color[c] == 1 {
                count += 1;
            }
        }
        colors.push((count, connected.len() - count));
    }

    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for &(a, b) in &colors {
        let mut next = vec![false; n + 1];
        for i in 0..n {
            if dp[i] {
                next[i + a] = true;
                next[i + b] = true;
            }
        }
        dp = next;
    }

    let mut ans = n;
    for i in 0..n {
        if dp[i] {
            let j = n - i;
            ans = cmp::min(ans, cmp::max(i, j));
        }
    }

    if ans == n {
        println!("{}", n * (n - 1) / 2);
    } else {
        println!("{}", ans * (ans - 1) / 2 + (n - ans) * (n - ans - 1) / 2);
    }
}

fn coloring_dfs(
    v: usize,
    connected: &mut Vec<usize>,
    graph: &Vec<Vec<usize>>,
    color: &mut Vec<usize>,
) -> bool {
    connected.push(v);
    for &next in &graph[v] {
        if color[next] == 0 {
            color[next] = if color[v] == 1 { 2 } else { 1 };
            if !coloring_dfs(next, connected, graph, color) {
                return false;
            }
        } else {
            if color[next] == color[v] {
                return false;
            }
        }
    }
    return true;
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
