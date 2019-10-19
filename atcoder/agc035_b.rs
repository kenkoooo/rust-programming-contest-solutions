fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    if m % 2 == 1 {
        println!("-1");
        return;
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push((b, i));
        graph[b].push((a, i));
    }
    let mut used = vec![false; m];
    dfs(0, &mut vec![false; n], &graph, &mut used);

    let mut direction = vec![None; m];
    for v in 0..n {
        for &(next, edge_id) in graph[v].iter() {
            if !used[edge_id] && direction[edge_id].is_none() {
                direction[edge_id] = Some((v, next));
            }
        }
    }
    dfs2(0, 0, &graph, &used, &mut direction);
    for i in 0..m {
        let (from, to) = direction[i].unwrap();
        println!("{} {}", from + 1, to + 1);
    }
}

fn dfs2(
    v: usize,
    p: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    used: &[bool],
    direction: &mut [Option<(usize, usize)>],
) {
    for &(next, edge_id) in graph[v].iter() {
        if p == next {
            continue;
        }
        if used[edge_id] {
            dfs2(next, v, graph, used, direction);
        }
    }

    let forward = graph[v]
        .iter()
        .flat_map(|&(_, edge_id)| direction[edge_id])
        .filter(|&(from, _)| from == v)
        .count();

    for &(next, edge_id) in graph[v].iter() {
        if used[edge_id] && direction[edge_id].is_none() {
            if forward % 2 == 0 {
                direction[edge_id] = Some((next, v));
            } else {
                direction[edge_id] = Some((v, next));
            }
        }
    }
}

fn dfs(v: usize, visited: &mut [bool], graph: &Vec<Vec<(usize, usize)>>, used: &mut [bool]) {
    visited[v] = true;
    for &(next, edge_id) in graph[v].iter() {
        if visited[next] {
            continue;
        }
        used[edge_id] = true;
        dfs(next, visited, graph, used);
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
