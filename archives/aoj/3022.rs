use std::cmp;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let w: Vec<usize> = sc.read_vec(n);
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let detection = BridgeDetector::new(&graph);
    let tree = detection.dfs_tree;
    let order = detection.order;
    let low_link = detection.low_link;
    let mut is_articulation = vec![false; n];
    for &v in detection.articulations.iter() {
        is_articulation[v] = true;
    }

    let total_power: usize = w.iter().sum();
    let mut subtree_power = vec![0; n];
    for v in 0..n {
        if is_articulation[v] {
            let mut max_child = 0;
            let mut parent_connected = 0;
            for &child in tree[v].iter() {
                let child_power = dfs(child, &tree, &mut subtree_power, &w);
                max_child = cmp::max(max_child, child_power);
                if low_link[child] < order[v] {
                    parent_connected += child_power;
                }
            }
            let subtree = dfs(v, &tree, &mut subtree_power, &w);
            let parent = total_power - subtree;
            println!("{}", cmp::max(max_child, parent + parent_connected));
        } else {
            println!("{}", total_power - w[v]);
        }
    }
}

fn dfs(v: usize, tree: &Vec<Vec<usize>>, subtree_power: &mut Vec<usize>, power: &Vec<usize>) -> usize {
    if subtree_power[v] > 0 {
        return subtree_power[v];
    }

    subtree_power[v] = power[v];
    for &child in tree[v].iter() {
        subtree_power[v] += dfs(child, tree, subtree_power, power);
    }
    subtree_power[v]
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
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
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
