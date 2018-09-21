/// Thank you tanakh!!!
///  https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() {
    input!(
        n: usize,
        m: usize,
        edges: [(usize1, usize1); m],
        q: usize,
        queries: [(usize1, usize1, usize1); q]
    );
    solve(n, &edges, &queries);
}

fn solve(n: usize, edges: &Vec<(usize, usize)>, queries: &Vec<(usize, usize, usize)>) {
    let bridges = {
        let mut graph = vec![vec![]; n];
        for &(u, v) in edges.iter() {
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut bridge_detector = BridgeDetector::new(n);
        bridge_detector.run(&graph);

        let mut bridges = BTreeSet::new();
        for &(a, b) in bridge_detector.bridges.iter() {
            bridges.insert((a, b));
            bridges.insert((b, a));
        }
        bridges
    };

    let mut uf = UnionFind::new(n);
    for &(a, b) in edges.iter() {
        if bridges.contains(&(a, b)) {
            continue;
        }
        uf.unite(a, b);
    }

    let mut set = BTreeSet::new();
    let mut find = vec![0; n];
    for i in 0..n {
        let f = uf.find(i);
        let cur = set.len();
        set.insert(f);
        if cur == set.len() {
            find[i] = find[f];
        } else {
            find[f] = cur;
            find[i] = find[f];
        }
    }

    let super_tree = {
        let mut super_tree = vec![vec![]; set.len()];
        for &(a, b) in edges.iter() {
            let a = find[a];
            let b = find[b];
            if a == b {
                continue;
            }
            super_tree[a].push(b);
            super_tree[b].push(a);
        }
        super_tree
    };

    let lca = LowestCommonAncestor::new(&super_tree);
    for &(a, b, c) in queries.iter() {
        let (a, b, c) = (find[a], find[b], find[c]);

        if lca.get_dist(a, b) + lca.get_dist(b, c) == lca.get_dist(a, c) {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}

struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    k: usize,
}

impl BridgeDetector {
    fn new(n: usize) -> Self {
        BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            ord: vec![0; n],
            low: vec![0; n],
            k: 0,
        }
    }

    fn run(&mut self, graph: &Vec<Vec<usize>>) {
        let n = graph.len();
        for i in 0..n {
            if !self.visit[i] {
                self.dfs(i, None, graph);
            }
        }
    }
    fn dfs(&mut self, v: usize, p: Option<usize>, graph: &Vec<Vec<usize>>) {
        self.visit[v] = true;
        self.ord[v] = self.k;
        self.k += 1;
        self.low[v] = self.ord[v];

        let mut is_articulation = false;
        let mut count = 0;
        for &next in graph[v].iter() {
            if !self.visit[next] {
                count += 1;
                self.dfs(next, Some(v), graph);
                if self.low[v] > self.low[next] {
                    self.low[v] = self.low[next];
                }
                if p.is_some() && self.ord[v] <= self.low[next] {
                    is_articulation = true;
                }
                if self.ord[v] < self.low[next] {
                    let (v, next) = if v < next { (v, next) } else { (next, v) };
                    self.bridges.push((v, next));
                }
            } else if p.is_none() || next != p.unwrap() && self.low[v] > self.ord[next] {
                self.low[v] = self.ord[next];
            }
        }

        if p.is_none() && count > 1 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulations.push(v);
        }
    }
}

const MAX_PARENT: usize = 1 << 50;
pub struct LowestCommonAncestor {
    parent: Vec<Vec<usize>>,
    depth: Vec<usize>,
    log_v: usize,
}

impl LowestCommonAncestor {
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let num_v = graph.len();
        let root = 0;
        let mut depth = vec![0; num_v];

        let mut log_v = 1;
        let mut i = 1;
        while i <= num_v {
            i *= 2;
            log_v += 1;
        }
        let mut parent: Vec<Vec<usize>> = vec![vec![0; num_v]; log_v];

        let mut depth_vis = vec![false; num_v];
        let mut stack = VecDeque::new();
        stack.push_front(root);
        parent[0][root] = MAX_PARENT;
        depth[root] = 0;
        depth_vis[root] = true;
        while !stack.is_empty() {
            let v = stack.pop_front().unwrap();
            stack.push_front(v);
            for &u in &graph[v] {
                if depth_vis[u] {
                    continue;
                }
                parent[0][u] = v;
                depth[u] = depth[v] + 1;
                depth_vis[u] = true;
                stack.push_front(u);
            }

            let head = stack.pop_front().unwrap();
            if head != v {
                stack.push_front(head);
            }
        }

        for k in 0..(log_v - 1) {
            for u in 0..num_v {
                parent[k + 1][u] = if parent[k][u] == MAX_PARENT {
                    MAX_PARENT
                } else {
                    parent[k][parent[k][u]]
                };
            }
        }

        LowestCommonAncestor {
            parent: parent,
            depth: depth,
            log_v: log_v,
        }
    }

    pub fn get_lca(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth[u] <= self.depth[v] {
            (u, v)
        } else {
            (v, u)
        };
        for k in 0..self.log_v {
            if ((self.depth[v] - self.depth[u]) & (1 << k)) != 0 {
                v = self.parent[k][v];
            }
        }
        if u == v {
            return u;
        }

        for k in (0..self.log_v).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u];
                v = self.parent[k][v];
            }
        }
        return self.parent[0][u];
    }

    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        let lca = self.get_lca(u, v);
        self.depth[u] + self.depth[v] - self.depth[lca] * 2
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        return true;
    }
}
