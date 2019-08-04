use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut has_c = vec![false; n];
    for _ in 0..m {
        let c = sc.read::<usize>() - 1;
        has_c[c] = true;
    }

    let mut q = VecDeque::new();
    let mut dist = vec![n; n];
    dist[0] = 0;
    q.push_back(0);
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if dist[next] > dist[v] + 1 {
                dist[next] = dist[v] + 1;
                q.push_back(next);
            }
        }
    }

    let mut far_c: Option<usize> = None;
    for i in 0..n {
        if !has_c[i] {
            continue;
        }
        match far_c {
            Some(cur) => {
                if dist[i] > dist[cur] {
                    far_c = Some(i);
                }
            }
            None => far_c = Some(i),
        }
    }

    let start = far_c.unwrap();
    let mut max = 0;
    let mut count = 0;
    dfs(start, start, &mut max, &mut count, &has_c, &graph);
    if max == m {
        println!("Yes");
    } else {
        println!("trumpet");
    }
}

fn dfs(
    v: usize,
    p: usize,
    max: &mut usize,
    count: &mut usize,
    has_c: &Vec<bool>,
    graph: &Vec<Vec<usize>>,
) {
    if has_c[v] {
        *count += 1;
        if *max < *count {
            *max = *count;
        }
    }
    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        dfs(next, v, max, count, has_c, graph);
    }
    if has_c[v] {
        *count -= 1;
    }
}

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
