use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    if n == 2 {
        println!("{}", if a[0] == a[1] { "YES" } else { "NO" });
        return;
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let a = sc.usize_read() - 1;
        let b = sc.usize_read() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut root = 0;
    for i in 0..n {
        if graph[root].len() < graph[i].len() {
            root = i;
        }
    }
    assert!(graph[root].len() > 1);

    let mut up = vec![0; n];
    if !dfs(&graph, root, root, &mut up, &a) || up[root] != 0 {
        println!("NO");
    } else {
        println!("YES");
    }
}

fn dfs(graph: &Vec<Vec<usize>>, v: usize, p: usize, up: &mut Vec<usize>, a: &Vec<usize>) -> bool {
    if graph[v].len() == 1 && graph[v][0] == p {
        up[v] = a[v];
        return true;
    }

    let mut sum = 0;
    let mut max = 0;
    for &next in &graph[v] {
        if next == p {
            continue;
        }
        if !dfs(graph, next, v, up, a) {
            return false;
        }
        sum += up[next];
        max = cmp::max(max, up[next]);
    }
    if 2 * a[v] < sum {
        return false;
    }
    up[v] = 2 * a[v] - sum;
    if sum < up[v] || (sum - up[v]) % 2 == 1 {
        return false;
    }
    let rest = sum - max;
    (sum - up[v]) / 2 <= cmp::min(rest, sum / 2)
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
