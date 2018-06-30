use std::cmp;
fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();
    let mut graph = vec![vec![false; n]; n];
    for _ in 0..m {
        let a = sc.usize_read() - 1;
        let b = sc.usize_read() - 1;
        graph[a][b] = true;
        graph[b][a] = true;
    }

    for i in 0..n {
        graph[i][i] = true;
    }

    let mut graph2 = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if !graph[i][j] {
                graph2[i].push(j);
            }
        }
    }

    let mut color = vec![0; n];
    let mut st = vec![];
    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        let mut vis = vec![];
        color[i] = 1;
        if !dfs(i, &graph2, &mut color, &mut vis) {
            println!("-1");
            return;
        }
        let mut count = 0;
        for &v in &vis {
            if color[v] == 1 {
                count += 1;
            }
        }
        st.push((vis.len() - count, count));
    }

    let mut ok = vec![false; n + 1];
    ok[0] = true;
    for &(s, t) in &st {
        let mut next = vec![false; n + 1];
        for i in (0..n).rev() {
            if ok[i] {
                next[i + s] = true;
                next[i + t] = true;
            }
        }
        ok = next;
    }

    let mut ans = n * (n - 1) / 2;
    for i in 1..n {
        if ok[i] {
            let j = n - i;
            ans = cmp::min(ans, j * (j - 1) / 2 + i * (i - 1) / 2);
        }
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<usize>, vis: &mut Vec<usize>) -> bool {
    vis.push(v);
    for &to in &graph[v] {
        if color[to] != 0 && color[to] == color[v] {
            return false;
        }
        if color[to] != 0 {
            continue;
        }
        color[to] = if color[v] == 1 { 2 } else { 1 };
        if !dfs(to, graph, color, vis) {
            return false;
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
