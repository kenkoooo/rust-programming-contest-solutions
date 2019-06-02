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

fn main() {
    input!(
        n: usize,
        m: usize,
        xy: [(usize1, usize1); m],
        q: usize,
        abc: [(usize1, usize1, usize1); q]
    );
    let mut graph = vec![vec![]; n];
    for &(x, y) in xy.iter() {
        graph[x].push(y);
        graph[y].push(x);
    }

    let bridges = BridgeDetector::new(&graph)
        .bridges
        .iter()
        .map(|&(x, y)| (x, y))
        .collect::<BTreeSet<_>>();
    let mut uf = UnionFind::new(n);
    for &(x, y) in xy.iter() {
        if bridges.contains(&(x, y)) || bridges.contains(&(y, x)) {
            continue;
        }
        uf.unite(x, y);
    }

    let mut map = vec![n; n];
    let mut n_tree = 0;
    for i in 0..n {
        let p = uf.find(i);
        if map[p] == n {
            map[p] = n_tree;
            n_tree += 1;
        }
        map[i] = map[p];
    }

    let n_tree = map.len();
    let mut tree = vec![vec![]; n_tree];
    for &(x, y) in bridges.iter() {
        let x = map[x];
        let y = map[y];
        tree[x].push(y);
        tree[y].push(x);
    }

    let lca = LowestCommonAncestor::new(&tree);
    for &(a, b, c) in abc.iter() {
        let a = map[a];
        let b = map[b];
        let c = map[c];
        if lca.get_dist(a, b) + lca.get_dist(b, c) == lca.get_dist(a, c) {
            println!("OK");
        } else {
            println!("NG");
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
pub struct BridgeDetector {
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
