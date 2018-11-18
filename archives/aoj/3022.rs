use std::cmp;

const INF: usize = 1e15 as usize;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut power = vec![0; n];
    for i in 0..n {
        power[i] = sc.read::<usize>();
    }

    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let detector = BridgeDetector::new(&graph);
    let mut is_articulation = vec![false; n];
    for &a in detector.articulations.iter() {
        is_articulation[a] = true;
    }

    let mut dp = vec![INF; n];
    let dfs_tree = detector.dfs_tree;
    let low_link = detector.low_link;
    let order = detector.order;
    let total_power = power.iter().sum::<usize>();
    for i in 0..n {
        if !is_articulation[i] {
            println!("{}", total_power - power[i]);
            continue;
        }

        let mut max_child = 0;
        let mut connected_to_parent = 0;
        for &child in dfs_tree[i].iter() {
            let dp_child = dfs(child, &dfs_tree, &mut dp, &power);
            max_child = cmp::max(max_child, dp_child);

            if low_link[child] < order[i] {
                connected_to_parent += dp_child;
            }
        }

        let dp_parent = total_power - (dfs(i, &dfs_tree, &mut dp, &power) - connected_to_parent);
        println!("{}", cmp::max(max_child, dp_parent));
    }
}

fn dfs(v: usize, tree: &Vec<Vec<usize>>, dp: &mut Vec<usize>, power: &Vec<usize>) -> usize {
    if dp[v] != INF {
        return dp[v];
    }

    dp[v] = power[v];
    for &next in tree[v].iter() {
        dp[v] += dfs(next, tree, dp, power);
    }
    dp[v]
}

pub struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    order: Vec<usize>,
    low_link: Vec<usize>,
    k: usize,
    dfs_tree: Vec<Vec<usize>>,
}

impl BridgeDetector {
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut d = BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            order: vec![0; n],
            low_link: vec![0; n],
            k: 0,
            dfs_tree: vec![vec![]; n],
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
