use std::cmp;
use std::collections::BinaryHeap;

const INF: i64 = std::i64::MAX;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let k: usize = sc.read();
    let bread: Vec<i64> = sc.vec(n);
    let jam_kind: Vec<usize> = sc.vec::<usize>(n).into_iter().map(|k| k - 1).collect();
    let jam_value: Vec<i64> = sc.vec(n);

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        let t = sc.read::<i64>();
        graph[v].push((u, t));
        graph[u].push((v, t));
    }

    let dist = dijkstra(&graph);

    let bread_dist = bread_dijkstra(&graph, &bread, &dist);
    let mut ans = vec![-INF; k];
    for i in 0..n {
        let bread_dist = bread_dist[i];
        let dist = dist[i];
        let jam_kind = jam_kind[i];
        let jam_value = jam_value[i];
        ans[jam_kind] = cmp::max(ans[jam_kind], jam_value - dist - bread_dist);
    }

    for ans in ans.into_iter() {
        println!("{}", ans);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut q = BinaryHeap::new();
    q.push((0, 0));

    let mut vis = vec![false; n];
    while let Some((_, v)) = q.pop() {
        if vis[v] {
            continue;
        }
        vis[v] = true;
        for &(next, weight) in graph[v].iter() {
            if dist[next] > dist[v] + weight {
                dist[next] = dist[v] + weight;
                q.push((-dist[next], next));
            }
        }
    }
    dist
}

fn bread_dijkstra(
    graph: &Vec<Vec<(usize, i64)>>,
    bread: &Vec<i64>,
    prev_dist: &Vec<i64>,
) -> Vec<i64> {
    let n = graph.len();
    let mut dist = vec![INF; n];
    let mut q = BinaryHeap::new();
    for i in 0..n {
        dist[i] = prev_dist[i] - bread[i];
        q.push((-dist[i], i));
    }

    let mut vis = vec![false; n];
    while let Some((_, v)) = q.pop() {
        if vis[v] {
            continue;
        }
        vis[v] = true;

        for &(next, weight) in graph[v].iter() {
            if dist[next] > dist[v] + weight {
                dist[next] = dist[v] + weight;
                q.push((-dist[next], next));
            }
        }
    }
    dist
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
