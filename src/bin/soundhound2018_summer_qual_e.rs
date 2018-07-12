use std::cmp;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let m = sc.read();

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.usize_read() - 1;
        let v = sc.usize_read() - 1;
        let s: i64 = sc.read();
        graph[u].push((v, s));
        graph[v].push((u, s));
    }

    let mut check: Vec<Vec<Option<i64>>> = vec![vec![None, None]; n];
    check[0][0] = Some(0);
    if !dfs(0, 0, &graph, &mut check) {
        println!("0");
        return;
    }

    let mut ans = None;
    let mut min_left = 0;
    let mut min_right = 1e15 as i64;
    for v in &check {
        let left = v[0];
        let right = v[1];
        match left {
            Some(left) => {
                match right {
                    Some(right) => {
                        // left+a=right-a
                        // a+a = right-left
                        if (right - left) % 2 != 0 {
                            println!("0");
                            return;
                        }
                        match ans {
                            Some(ans) => {
                                if ans != (right - left) / 2 {
                                    println!("0");
                                    return;
                                }
                            }
                            None => {
                                ans = Some((right - left) / 2);
                            }
                        }
                    }
                    None => {
                        // left+a>0
                        min_left = cmp::min(min_left, left);
                    }
                }
            }
            None => {
                match right {
                    Some(right) => {
                        // right-a>0
                        min_right = cmp::min(min_right, right);
                    }
                    None => {
                        panic!();
                    }
                }
            }
        }
    }

    match ans {
        Some(ans) => {
            for v in &check {
                let left = v[0];
                let right = v[1];
                match left {
                    Some(left) => {
                        if left + ans <= 0 {
                            println!("0");
                            return;
                        }
                    }
                    _ => {}
                }
                match right {
                    Some(right) => {
                        if right - ans <= 0 {
                            println!("0");
                            return;
                        }
                    }
                    _ => {}
                }
            }
            println!("1");
        }
        None => {
            // min_right>ans>-min_left
            println!("{}", cmp::max(min_right - (-min_left) - 1, 0));
        }
    }
}

fn dfs(
    v: usize,
    side: usize,
    graph: &Vec<Vec<(usize, i64)>>,
    check: &mut Vec<Vec<Option<i64>>>,
) -> bool {
    let cur = check[v][side].unwrap();
    for &(next, sum) in &graph[v] {
        match check[next][side ^ 1] {
            Some(t) => if cur + t != sum {
                return false;
            },
            None => {
                check[next][side ^ 1] = Some(sum - cur);
                if !dfs(next, side ^ 1, graph, check) {
                    return false;
                }
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
