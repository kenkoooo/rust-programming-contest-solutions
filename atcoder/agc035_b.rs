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
    let mut edges = vec![];
    for i in 0..m {
        let x = sc.read::<usize>() - 1;
        let y = sc.read::<usize>() - 1;
        graph[x].push((y, i));
        graph[y].push((x, i));
        edges.push((x, y));
    }

    let mut used = vec![false; m];
    let mut tree = vec![vec![]; n];
    let mut vis = vec![false; n];
    vis[0] = true;
    dfs(0, &graph, &mut used, &mut vis, &mut tree);
    let mut ans = vec![vec![]; n];
    for (v, e) in graph.into_iter().enumerate() {
        for (u, edge_id) in e.into_iter() {
            if used[edge_id] {
                continue;
            }
            used[edge_id] = true;
            ans[v].push(u);
        }
    }

    ans_dfs(0, &tree, &mut ans);

    for from in 0..n {
        for &to in ans[from].iter() {
            println!("{} {}", from + 1, to + 1);
        }
    }
}

fn ans_dfs(v: usize, tree: &Vec<Vec<usize>>, graph: &mut Vec<Vec<usize>>) {
    for &next in tree[v].iter() {
        ans_dfs(next, tree, graph);
        if graph[next].len() % 2 == 0 {
            graph[v].push(next);
        } else {
            graph[next].push(v);
        }
    }
}

fn dfs(
    v: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    used: &mut Vec<bool>,
    vis: &mut Vec<bool>,
    tree: &mut Vec<Vec<usize>>,
) {
    for &(next, edge_id) in graph[v].iter() {
        if used[edge_id] {
            continue;
        }
        if vis[next] {
            continue;
        }
        vis[next] = true;
        used[edge_id] = true;
        tree[v].push(next);
        dfs(next, graph, used, vis, tree);
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
