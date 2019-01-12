use std::cmp;

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

    for i in 1..(n + 1) {
        if dp1[0][i] < INF {
            println!("{}", i - 1);
            return;
        }
        if dp2[0][i] < 0 {
            println!("{}", i - 1);
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
    dp1[v].push(INF);
    dp1[v].push(if a[v] > 0 { a[v] } else { INF });
    dp2[v].push(INF);
    dp2[v].push(a[v]);
    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        dfs(next, v, graph, a, dp1, dp2);

        dp1[v] = tr(&dp1[v], &dp1[next], &dp2[next], true);
        dp2[v] = tr(&dp2[v], &dp1[next], &dp2[next], false);
    }
}

fn tr(dp: &Vec<i64>, ch1: &Vec<i64>, ch2: &Vec<i64>, flag: bool) -> Vec<i64> {
    let child_cuts = ch1.len();
    assert_eq!(child_cuts, ch2.len());
    let mut ans = vec![INF; dp.len() + child_cuts - 1];
    for i in 0..dp.len() {
        if dp[i] == INF {
            continue;
        }
        for j in 0..child_cuts {
            if i + j >= 1 {
                if ch1[j] != INF {
                    ans[i + j - 1] = cmp::min(ans[i + j - 1], dp[i] + ch1[j]);
                }
                if ch2[j] != INF && !flag {
                    ans[i + j - 1] = cmp::min(ans[i + j - 1], dp[i] + ch2[j]);
                }
            }
            if ch1[j] != INF || ch2[j] < 0 {
                ans[i + j] = cmp::min(ans[i + j], dp[i]);
            }
        }
    }
    ans
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
