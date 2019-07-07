use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let (_, v) = bfs(0, &graph);
    let (dist, _) = bfs(v, &graph);
    if dist == 0 {
        println!("First");
        return;
    }

    let mut grundy = vec![0; dist + 1];
    grundy[0] = 1;
    grundy[1] = 0;
    for i in 2..(dist + 1) {
        let next = [grundy[i - 1], grundy[i - 2]];
        grundy[i] = (0..).find(|i| !next.contains(i)).unwrap();
    }
    println!("{}", if grundy[dist] == 0 { "Second" } else { "First" });
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> (usize, usize) {
    let n = graph.len();
    let mut dist = vec![n; n];
    dist[start] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if dist[next] > dist[v] + 1 {
                dist[next] = dist[v] + 1;
                q.push_back(next);
            }
        }
    }
    dist.into_iter()
        .enumerate()
        .map(|(i, d)| (d, i))
        .max()
        .unwrap()
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
