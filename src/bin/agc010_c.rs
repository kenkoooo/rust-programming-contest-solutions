use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize_read();
    let a: Vec<usize> = sc.read_vec(n);
    if n == 2 {
        if a[0] == a[1] {
            println!("YES");
        } else {
            println!("NO");
        }
        return;
    }
    let mut graph = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let u = sc.usize_read() - 1;
        let v = sc.usize_read() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut root = 0;
    for i in 0..n {
        if graph[i].len() > 1 {
            root = i;
            break;
        }
    }
    let mut up = vec![0; n];
    if !dfs(root, root, &graph, &mut up, &a) || up[root] != 0 {
        println!("NO");
    } else {
        println!("YES");
    }
}

fn dfs(v: usize, p: usize, graph: &Vec<Vec<usize>>, up: &mut Vec<usize>, a: &Vec<usize>) -> bool {
    if graph[v].len() == 1 && graph[v][0] == p {
        up[v] = a[v];
        return true;
    }

    let mut sum = 0;
    let mut max = 0;
    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        if !dfs(next, v, graph, up, a) {
            return false;
        }
        sum += up[next];
        max = cmp::max(max, up[next]);
    }
    if sum < a[v] {
        return false;
    }
    let pair = sum - a[v];
    if sum < pair * 2 || sum - max < pair {
        return false;
    }
    up[v] = sum - pair * 2;
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
