use std::cmp;
const INF: i64 = 1e18 as i64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let a: Vec<i64> = sc.vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dp1 = vec![vec![]; n];
    let mut dp2 = vec![vec![]; n];
    dp(0, 0, &mut dp1, &mut dp2, &a, &graph);

    for i in 0..n {
        if dp1[0][i] < INF || dp2[0][i] < 0 {
            println!("{}", i);
            return;
        }
    }
}

fn dp(
    v: usize,
    p: usize,
    dp1: &mut Vec<Vec<i64>>,
    dp2: &mut Vec<Vec<i64>>,
    a: &Vec<i64>,
    graph: &Vec<Vec<usize>>,
) {
    dp1[v].push(if a[v] > 0 { a[v] } else { INF });
    dp2[v].push(a[v]);
    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        dp(next, v, dp1, dp2, a, graph);
        dp1[v] = connect(&dp1[v], &dp1[next], &dp2[next], true);
        dp2[v] = connect(&dp2[v], &dp1[next], &dp2[next], false);
    }
}

fn connect(dp: &Vec<i64>, child1: &Vec<i64>, child2: &Vec<i64>, is_power: bool) -> Vec<i64> {
    let mut next = vec![INF; dp.len() + child1.len() + 1];
    for parent_cut in 0..dp.len() {
        for child_cut in 0..child1.len() {
            let total_cut = parent_cut + child_cut;

            if child1[child_cut] < INF {
                next[total_cut] = cmp::min(next[total_cut], dp[parent_cut] + child1[child_cut]);
            }
            if child2[child_cut] < INF && !is_power {
                next[total_cut] = cmp::min(next[total_cut], dp[parent_cut] + child2[child_cut]);
            }
            if child1[child_cut] < INF || child2[child_cut] < 0 {
                next[total_cut + 1] = cmp::min(next[total_cut + 1], dp[parent_cut]);
            }
        }
    }
    next
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
