const INF: i64 = 1e17 as i64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let a: Vec<i64> = sc.read_vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let v = sc.read::<usize>() - 1;
        let u = sc.read::<usize>() - 1;
        graph[v].push(u);
        graph[u].push(v);
    }

    let mut dp1 = vec![vec![]; n];
    let mut dp2 = vec![vec![]; n];
    dfs(0, 0, &graph, &a, &mut dp1, &mut dp2);

    for i in 0..n {
        if dp1[0][i] < INF {
            println!("{}", i);
            return;
        }
        if dp2[0][i] < 0 {
            println!("{}", i);
            return;
        }
    }
}

fn dfs(
    v: usize,
    p: usize,
    graph: &Vec<Vec<usize>>,
    a: &Vec<i64>,
    dp1: &mut Vec<Vec<i64>>,
    dp2: &mut Vec<Vec<i64>>,
) {
    dp1[v].push(if a[v] > 0 { a[v] } else { INF });
    dp2[v].push(a[v]);
    for &next in graph[v].iter() {
        if next == p { continue; }
        dfs(next, v, graph, a, dp1, dp2);

        dp1[v] = connect(&dp1[v], &dp1[next], &dp2[next], true);
        dp2[v] = connect(&dp2[v], &dp1[next], &dp2[next], false);
    }
}

fn connect(dp: &Vec<i64>, child1: &Vec<i64>, child2: &Vec<i64>, power_only: bool) -> Vec<i64> {
    let mut ans = vec![INF; dp.len() + child1.len() + 1];
    for cuts in 0..dp.len() {
        if dp[cuts] == INF { continue; }
        for child_cuts in 0..child1.len() {
            let total_cuts = cuts + child_cuts + 1;

            if child1[child_cuts] != INF {
                ch_min(&mut ans[total_cuts - 1], dp[cuts] + child1[child_cuts]);
            }
            if child2[child_cuts] != INF && !power_only {
                ch_min(&mut ans[total_cuts - 1], dp[cuts] + child2[child_cuts]);
            }
            if child1[child_cuts] != INF || child2[child_cuts] < 0 {
                ch_min(&mut ans[total_cuts], dp[cuts]);
            }
        }
    }
    ans
}

fn ch_min<T: PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
