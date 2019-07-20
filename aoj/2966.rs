use std::cmp;

const INF: i64 = std::i64::MAX;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let w: usize = sc.read();
    let p: Vec<i64> = sc.vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        let cost = sc.read::<usize>();
        graph[u].push((v, cost));
        graph[v].push((u, cost));
    }

    let mut tree = vec![vec![]; n];
    tree_dfs(0, 0, &graph, &mut tree);

    let mut ans = -INF;
    for i in 0..n {
        let mut state = vec![-INF; w + 1];
        state[0] = 0;
        let state = rec_dp(i, &state, &tree, w, &p);
        ans = cmp::max(ans, state.into_iter().max().unwrap());
    }
    println!("{}", ans);
}

fn tree_dfs(
    v: usize,
    p: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    tree: &mut Vec<Vec<(usize, usize)>>,
) {
    for &(next, cost) in graph[v].iter() {
        if next == p {
            continue;
        }
        tree_dfs(next, v, graph, tree);
        tree[v].push((next, cost));
    }
}

fn rec_dp(
    v: usize,
    state: &Vec<i64>,
    tree: &Vec<Vec<(usize, usize)>>,
    w: usize,
    values: &Vec<i64>,
) -> Vec<i64> {
    let mut state = state.clone();
    for &(next, cost) in tree[v].iter() {
        let next_state = rec_dp(next, &state, tree, w, values);
        for i in 0..(w + 1) {
            if i + cost > w {
                break;
            }
            state[i + cost] = cmp::max(next_state[i], state[i + cost]);
        }
    }

    for i in 0..(w + 1) {
        state[i] += values[v];
    }
    state
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
