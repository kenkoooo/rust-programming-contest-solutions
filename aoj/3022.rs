use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let m: usize = sc.read();
    let w: Vec<u64> = sc.vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let detector = BridgeDetector::new(&graph);
    let mut is_articulation = vec![false; n];
    for &v in detector.articulations.iter() {
        is_articulation[v] = true;
    }
    let tree = detector.dfs_tree;
    let low_link = detector.low_link;
    let order = detector.order;

    let sum = w.iter().sum::<u64>();
    let mut tree_dp = vec![0; n];
    for i in 0..n {
        if is_articulation[i] {
            let mut parent = sum - w[i];
            let mut max_child = 0;
            for &next in tree[i].iter() {
                let child = dfs(next, &tree, &w, &mut tree_dp);
                max_child = cmp::max(max_child, child);
                parent -= child;

                if low_link[next] < order[i] {
                    parent += child;
                }
            }
            println!("{}", cmp::max(max_child, parent));
        } else {
            println!("{}", sum - w[i]);
        }
    }
}

fn dfs(v: usize, tree: &Vec<Vec<usize>>, w: &Vec<u64>, dp: &mut Vec<u64>) -> u64 {
    if dp[v] > 0 {
        return dp[v];
    }
    dp[v] = w[v];
    for &next in tree[v].iter() {
        dp[v] += dfs(next, tree, w, dp);
    }
    return dp[v];
}

pub struct BridgeDetector {
    dfs_tree: Vec<Vec<usize>>,
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    order: Vec<usize>,
    low_link: Vec<usize>,
    k: usize,
}

impl BridgeDetector {
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut d = BridgeDetector {
            dfs_tree: vec![vec![]; n],
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            order: vec![0; n],
            low_link: vec![0; n],
            k: 0,
        };
        d.run(graph);
        d
    }

    fn run(&mut self, graph: &Vec<Vec<usize>>) {
        let n = graph.len();
        for i in 0..n {
            if !self.visit[i] {
                self.dfs(i, 0, graph, i);
            }
        }
    }

    fn dfs(&mut self, v: usize, previous: usize, graph: &Vec<Vec<usize>>, root: usize) {
        self.visit[v] = true;
        self.order[v] = self.k;
        self.k += 1;
        self.low_link[v] = self.order[v];

        let mut is_articulation = false;
        let mut dimension = 0;
        for &next in graph[v].iter() {
            if !self.visit[next] {
                // The edge (v->next) is not a backedge.
                self.dfs_tree[v].push(next);
                dimension += 1;
                self.dfs(next, v, graph, root);
                self.low_link[v] = cmp::min(self.low_link[v], self.low_link[next]);
                if v != root && self.order[v] <= self.low_link[next] {
                    is_articulation = true;
                }
                if self.order[v] < self.low_link[next] {
                    let min = cmp::min(v, next);
                    let max = cmp::max(v, next);
                    self.bridges.push((min, max));
                }
            } else if v == root || next != previous {
                // The edge (v->next) is a backedge.
                self.low_link[v] = cmp::min(self.low_link[v], self.order[next]);
            }
        }

        if v == root && dimension > 1 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulations.push(v);
        }
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
