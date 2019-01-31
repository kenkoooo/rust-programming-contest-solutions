use std::cmp;

const INF: usize = 1e16 as usize;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let m = sc.read();
    let power: Vec<usize> = sc.read_vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let detection = BridgeDetector::new(&graph);
    let total = power.iter().sum::<usize>();

    let mut is_articulation = vec![false; n];
    for &v in detection.articulations.iter() {
        is_articulation[v] = true;
    }

    let tree = detection.dfs_tree;
    let low_link = detection.low_link;
    let order = detection.order;
    let mut dp = vec![INF; n];
    for v in 0..n {
        if is_articulation[v] {
            let mut max_child = 0;
            let mut connected_to_parent = 0;
            for &child in tree[v].iter() {
                let dp_child = dfs(child, &tree, &mut dp, &power);
                max_child = cmp::max(max_child, dp_child);

                if low_link[child] < order[v] {
                    connected_to_parent += dp_child;
                }
            }
            let dp_parent = total - dfs(v, &tree, &mut dp, &power) + connected_to_parent;
            println!("{}", cmp::max(dp_parent, max_child));
        } else {
            println!("{}", total - power[v]);
        }
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
                self.dfs_tree[v].push(next);

                // The edge (v->next) is not a backedge.
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
