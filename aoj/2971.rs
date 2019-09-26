fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut edges = vec![];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let x = sc.read::<u64>();
        edges.push((a, b, x));
    }

    for &m in [1_000_000_007, 1_000_000_009, 998244353].iter() {
        let mut graph = vec![vec![]; n];
        for &(a, b, x) in edges.iter() {
            graph[a].push((b, x));
            graph[b].push((a, mod_pow(x, m - 2, m)));
        }
        if !solve(&graph, m) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn solve(graph: &Vec<Vec<(usize, u64)>>, m: u64) -> bool {
    let n = graph.len();
    let mut items = vec![0; n];
    let mut visit = vec![None; n];
    for i in 0..n {
        if visit[i].is_none() {
            visit[i] = Some(i);
            items[i] = 1;
            if !dfs(i, i, graph, &mut items, &mut visit, m) {
                return false;
            }
        }
    }
    true
}

fn dfs(
    v: usize,
    start: usize,
    graph: &Vec<Vec<(usize, u64)>>,
    items: &mut Vec<u64>,
    visit: &mut Vec<Option<usize>>,
    m: u64,
) -> bool {
    for &(next, gain) in graph[v].iter() {
        match visit[next] {
            Some(_) => {
                if items[next] != 0 && items[next] != (items[v] * gain) % m {
                    return false;
                }
            }
            None => {
                visit[next] = Some(start);
                assert_eq!(items[next], 0);
                items[next] = (items[v] * gain) % m;
                if !dfs(next, start, graph, items, visit, m) {
                    return false;
                }
                items[next] = 0;
            }
        }
    }
    true
}

fn mod_pow(x: u64, mut e: u64, m: u64) -> u64 {
    let mut result = 1;
    let mut cur = x;
    while e > 0 {
        if e & 1 != 0 {
            result = (result * cur) % m;
        }
        e >>= 1;
        cur = (cur * cur) % m;
    }
    result
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
