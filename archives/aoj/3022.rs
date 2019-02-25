use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let m = sc.read();
    let power: Vec<u64> = sc.vec(n);

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let detection = BridgeDetector::new(&graph);
    let mut is_articulation = vec![false; n];
    for &v in detection.articulations.iter() {
        is_articulation[v] = true;
    }

    let order = detection.order;
    let low_link = detection.low_link;
    let dfs_tree = detection.dfs_tree;
    let mut subtree_power = vec![0; n];
    calc_subtree_power(0, 0, &mut subtree_power, &power, &dfs_tree);

    let power_sum: u64 = power.iter().sum();
    for v in 0..n {
        if is_articulation[v] {
            let mut parent_cluster = power_sum - subtree_power[v];
            let mut max_child = 0;
            for &child in dfs_tree[v].iter() {
                if order[child] < order[v] {
                    continue;
                }

                max_child = cmp::max(max_child, subtree_power[child]);
                if low_link[child] < order[v] {
                    parent_cluster += subtree_power[child];
                }
            }
            println!("{}", cmp::max(max_child, parent_cluster));
        } else {
            println!("{}", power_sum - power[v]);
        }
    }
}

fn calc_subtree_power(
    v: usize,
    p: usize,
    subtree_power: &mut Vec<u64>,
    power: &[u64],
    dfs_tree: &[Vec<usize>],
) {
    subtree_power[v] = power[v];
    for &next in dfs_tree[v].iter() {
        if next == p {
            continue;
        }
        calc_subtree_power(next, p, subtree_power, power, dfs_tree);
        subtree_power[v] += subtree_power[next];
    }
}

pub struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    order: Vec<usize>,
    low_link: Vec<usize>,
    dfs_tree: Vec<Vec<usize>>,
    k: usize,
}

impl BridgeDetector {
    pub fn new(graph: &[Vec<usize>]) -> Self {
        let n = graph.len();
        let mut d = BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            order: vec![0; n],
            low_link: vec![0; n],
            dfs_tree: vec![vec![]; n],
            k: 0,
        };
        d.run(graph);
        d
    }

    fn run(&mut self, graph: &[Vec<usize>]) {
        let n = graph.len();
        for i in 0..n {
            if !self.visit[i] {
                self.dfs(i, 0, graph, i);
            }
        }
    }

    fn dfs(&mut self, v: usize, previous: usize, graph: &[Vec<usize>], root: usize) {
        self.visit[v] = true;
        self.order[v] = self.k;
        self.k += 1;
        self.low_link[v] = self.order[v];

        let mut is_articulation = false;
        let mut dimension = 0;
        for &next in graph[v].iter() {
            if !self.visit[next] {
                // The edge (v->next) is not a backedge.
                dimension += 1;
                self.dfs_tree[v].push(next);
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
