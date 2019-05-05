fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let a: Vec<usize> = sc.read_vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let x: usize = sc.read();
        let y: usize = sc.read();
        graph[x].push(y);
        graph[y].push(x);
    }

    let mut vis = vec![false; n];
    let mut trees = Vec::new();
    for i in 0..n {
        if !vis[i] {
            let mut tree = Vec::new();
            dfs(i, &mut tree, &mut vis, &graph);
            tree.sort_by_key(|&i| a[i]);
            trees.push(tree);
        }
    }

    let mut used = vec![false; n];
    let mut ans = 0;
    for i in 0..trees.len() {
        ans += a[trees[i][0]];
        used[trees[i][0]] = true;
    }

    let mut rest = Vec::new();
    for i in 0..n {
        if !used[i] {
            rest.push(a[i]);
        }
    }
    rest.sort();

    let needed = (n - 1 - m) * 2;
    if rest.len() + trees.len() < needed {
        println!("Impossible");
        return;
    }
    if m == n - 1 {
        println!("0");
        return;
    }

    for i in 0..(needed - trees.len()) {
        ans += rest[i];
    }
    println!("{}", ans);
}

fn dfs(v: usize, tree: &mut Vec<usize>, vis: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    tree.push(v);
    vis[v] = true;
    for &to in &graph[v] {
        if !vis[to] {
            dfs(to, tree, vis, graph);
        }
    }
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
