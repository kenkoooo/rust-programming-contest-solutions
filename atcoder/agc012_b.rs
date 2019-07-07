use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[v].push(u);
        graph[u].push(v);
    }

    let q: usize = sc.read();
    let queries = (0..q)
        .map(|_| (sc.read::<usize>() - 1, sc.read::<usize>(), sc.read::<u32>()))
        .collect::<Vec<_>>();
    let mut color = vec![0; n];

    let mut dist = vec![0; n];
    for (v, d, c) in queries.into_iter().rev() {
        let mut q = VecDeque::new();
        if color[v] == 0 {
            color[v] = c;
        }
        q.push_back((v, d));
        while let Some((v, d)) = q.pop_front() {
            if d == 0 {
                continue;
            }
            for &next in graph[v].iter() {
                if color[next] == 0 {
                    color[next] = c;
                }
                if dist[next] < d - 1 {
                    dist[next] = d - 1;
                    q.push_back((next, dist[next]));
                }
            }
        }
    }

    for color in color.into_iter() {
        println!("{}", color);
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
