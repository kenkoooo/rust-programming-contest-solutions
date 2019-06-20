const INF: i64 = 1e15 as i64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let a: Vec<i64> = sc.vec(n);
    let mut tree = vec![vec![]; n];
    for _ in 1..n {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut power_dp = vec![vec![]; n];
    let mut dp = vec![vec![]; n];
    dfs(0, 0, &tree, &mut power_dp, &mut dp, &a);

    for i in 0..power_dp[0].len() {
        if power_dp[0][i] > 0 && power_dp[0][i] < INF {
            println!("{}", i);
            return;
        }
        if dp[0][i] < 0 {
            println!("{}", i);
            return;
        }
    }
}

fn dfs(
    v: usize,
    p: usize,
    tree: &Vec<Vec<usize>>,
    power_dp: &mut Vec<Vec<i64>>,
    dp: &mut Vec<Vec<i64>>,
    a: &[i64],
) {
    power_dp[v] = if a[v] > 0 { vec![a[v]] } else { vec![INF] };
    dp[v] = vec![a[v]];
    for &next in tree[v].iter() {
        if p == next {
            continue;
        }
        dfs(next, v, tree, power_dp, dp, a);
        power_dp[v] = connect(&power_dp[v], &power_dp[next], &dp[next], true);
        dp[v] = connect(&dp[v], &power_dp[next], &dp[next], false);
    }
}

fn connect(cur: &[i64], power: &[i64], dp: &[i64], for_power: bool) -> Vec<i64> {
    let main_cuts = cur.len();
    let child_cuts = power.len();
    let mut next = vec![INF; main_cuts + child_cuts + 1];
    for main_cut in 0..main_cuts {
        let main_top = cur[main_cut];
        if main_top >= INF {
            continue;
        }
        for child_cut in 0..child_cuts {
            let child_power_top = power[child_cut];
            let child_top = dp[child_cut];
            if child_power_top < INF {
                apply_min(&mut next[child_cut + main_cut], main_top + child_power_top);
            }
            if !for_power && child_top < INF {
                apply_min(&mut next[child_cut + main_cut], main_top + child_top);
            }

            if child_top < 0 || (child_power_top > 0 && child_power_top < INF) {
                if !for_power {
                    apply_min(&mut next[child_cut + main_cut + 1], main_top);
                } else if main_top > 0 {
                    apply_min(&mut next[child_cut + main_cut + 1], main_top);
                }
            }
        }
    }
    next
}

fn apply_min<T: Ord>(x: &mut T, y: T) {
    if *x > y {
        *x = y;
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
