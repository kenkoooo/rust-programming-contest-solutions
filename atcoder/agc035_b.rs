fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
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

    let mut vis = vec![false; n];
    let mut edges: Vec<Option<(usize, usize)>> = vec![None; m];
    vis[0] = true;
    dfs(0, 0, &graph, &mut edges, &mut vis);
    for e in edges.into_iter() {
        let (from, to) = e.unwrap();
        sc.write(format!("{} {}\n", from + 1, to + 1));
    }
}

fn dfs(
    v: usize,
    p: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    edges: &mut Vec<Option<(usize, usize)>>,
    vis: &mut Vec<bool>,
) {
    for &(next, _) in graph[v].iter() {
        if vis[next] {
            continue;
        }
        vis[next] = true;
        dfs(next, v, graph, edges, vis);
    }

    let mut out = 0;
    for &(next, edge_id) in graph[v].iter() {
        if next == p {
            continue;
        }
        if edges[edge_id].is_none() {
            edges[edge_id] = Some((v, next));
        }
        let (from, _) = edges[edge_id].unwrap();
        if from == v {
            out += 1;
        }
    }
    for &(next, edge_id) in graph[v].iter() {
        if next == p {
            assert!(edges[edge_id].is_none());
            if out % 2 == 0 {
                edges[edge_id] = Some((next, v));
            } else {
                edges[edge_id] = Some((v, next));
            }
        }
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
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
