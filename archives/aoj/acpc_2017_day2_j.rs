/// Thank you tanakh!!!
/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::cmp;
use std::collections::{BTreeSet, VecDeque};

const INF: usize = 1e15 as usize;

fn main() {
    input!(
        n: usize,
        m: usize,
        power: [usize; n],
        edges: [(usize1, usize1); m]
    );

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        graph[v].push(u);
        graph[u].push(v);
    }

    let bridge_detector = BridgeDetector::new(&graph);
    let total: usize = power.iter().sum();

    let mut is_articulation = vec![false; n];
    for &v in bridge_detector.articulations.iter() {
        is_articulation[v] = true;
    }

    let mut dp = vec![INF; n];
    let tree = bridge_detector.dfs_tree;
    let low_link = bridge_detector.low_link;
    let order = bridge_detector.order;
    for i in 0..n {
        if is_articulation[i] {
            let mut max_child = 0;
            let mut connected_to_parent = 0;
            for &child in tree[i].iter() {
                let dp_child = dfs(child, &tree, &mut dp, &power);
                max_child = cmp::max(max_child, dp_child);

                if low_link[child] < order[i] {
                    connected_to_parent += dp_child;
                }
            }

            let dp_parent = total - (dfs(i, &tree, &mut dp, &power) - connected_to_parent);

            println!("{}", cmp::max(dp_parent, max_child));
        } else {
            println!("{}", total - power[i]);
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
