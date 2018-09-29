use std::cmp;

const INF: usize = 1e18 as usize;

fn dfs(v: usize, parent: usize, x: &Vec<usize>, graph: &Vec<Vec<usize>>) -> usize {
    let mut dp = vec![INF; x[v] + 1];
    dp[0] = 0;

    for &child in graph[v].iter() {
        if child == parent {
            continue;
        }
        let white = dfs(child, v, x, graph);
        let black = x[child];
        let mut next = vec![INF; x[v] + 1];
        for i in 0..(x[v] + 1) {
            if dp[i] != INF {
                if i + white <= x[v] {
                    next[i + white] = cmp::min(next[i + white], dp[i] + black);
                }
                if i + black <= x[v] {
                    next[i + black] = cmp::min(next[i + black], dp[i] + white);
                }
            }
        }
        dp = next;
    }

    *dp.iter().min().unwrap()
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let mut graph = vec![vec![]; n];
    for i in 0..(n - 1) {
        let p = sc.read::<usize>() - 1;
        graph[p].push(i + 1);
        graph[i + 1].push(p);
    }
    let x = sc.read_vec(n);

    if dfs(0, INF, &x, &graph) != INF {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
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
