fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let v = sc.read::<usize>() - 1;
        let u = sc.read::<usize>() - 1;

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut bipartite: usize = 0;
    let mut single: usize = 0;
    let mut others: usize = 0;
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            color[i] = 1;
            if graph[i].is_empty() {
                single += 1;
            } else if color_dfs(i, &graph, &mut color) {
                bipartite += 1;
            } else {
                others += 1;
            }
        }
    }

    let ans1 = single * n * 2 - single * single;
    let ans2 = others * (others + bipartite) * 2 - others * others;
    let ans3 = bipartite * bipartite * 2;
    println!("{}", ans1 + ans2 + ans3);
}

fn color_dfs(v: usize, graph: &[Vec<usize>], color: &mut [usize]) -> bool {
    let mut ok = true;
    for &next in graph[v].iter() {
        if color[next] == 0 {
            color[next] = if color[v] == 1 { 2 } else { 1 };
            ok &= color_dfs(next, graph, color);
        } else if color[next] == color[v] {
            ok = false;
        }
    }
    ok
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
