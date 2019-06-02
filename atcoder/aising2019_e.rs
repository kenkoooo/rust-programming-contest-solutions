const INF: i64 = 1 << 50;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let a: Vec<i64> = sc.vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[v].push(u);
        graph[u].push(v);
    }

    let mut min_dp = vec![vec![]; n];
    let mut power_dp = vec![vec![]; n];
    dfs(0, 0, &a, &graph, &mut min_dp, &mut power_dp);
    for i in 0..n {
        if min_dp[0][i] < 0 || power_dp[0][i] < INF {
            println!("{}", i);
            return;
        }
    }
}

fn dfs(
    v: usize,
    p: usize,
    a: &Vec<i64>,
    graph: &Vec<Vec<usize>>,
    min_dp: &mut Vec<Vec<i64>>,
    power_dp: &mut Vec<Vec<i64>>,
) {
    min_dp[v].push(a[v]);
    power_dp[v].push(if a[v] > 0 { a[v] } else { INF });
    for &child in graph[v].iter() {
        if p == child {
            continue;
        }
        dfs(child, v, a, graph, min_dp, power_dp);
        power_dp[v] = connect(&power_dp[v], &power_dp[child], &min_dp[child], true);
        min_dp[v] = connect(&min_dp[v], &power_dp[child], &min_dp[child], false);
    }
}

fn connect(dp: &Vec<i64>, power: &Vec<i64>, min: &Vec<i64>, is_power: bool) -> Vec<i64> {
    let mut next = vec![INF; power.len() + dp.len() + 1];
    for parent_cut in 0..dp.len() {
        for child_cut in 0..power.len() {
            let total_cut = parent_cut + child_cut;
            if !is_power {
                chmin(&mut next[total_cut], dp[parent_cut] + min[child_cut]);
            }
            if power[child_cut] < INF {
                chmin(&mut next[total_cut], dp[parent_cut] + power[child_cut]);
            }
            if power[child_cut] < INF || min[child_cut] < 0 {
                chmin(&mut next[total_cut + 1], dp[parent_cut]);
            }
        }
    }
    next
}

fn chmin<T: Ord>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
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
