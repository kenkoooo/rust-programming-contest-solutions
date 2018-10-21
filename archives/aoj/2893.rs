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
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input!(n: usize, m: usize, uvw: [(usize1, usize1, usize); m]);

    let mut graph = vec![vec![]; n];
    for &(u, v, _) in uvw.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut union_find = UnionFind::new(n);
    let bridges = BridgeDetector::new(&graph)
        .bridges
        .iter()
        .map(|&(u, v)| (cmp::min(u, v), cmp::max(u, v)))
        .collect::<BTreeSet<_>>();
    let mut edges = vec![];
    for &(u, v, w) in uvw.iter() {
        assert!(u < v);
        if bridges.contains(&(u, v)) {
            edges.push((u, v, w));
            continue;
        }
        union_find.unite(u, v);
    }

    let mut convert = BTreeMap::new();
    for i in 0..n {
        let parent = union_find.find(i);
        if convert.contains_key(&parent) {
            continue;
        }
        let n = convert.len();
        convert.insert(parent, n);
    }

    let n = convert.len();
    let mut costs = vec![0; n];
    for &(u, v, w) in uvw.iter() {
        if union_find.find(u) == union_find.find(v) {
            let parent = union_find.find(u);
            let i = *convert.get(&parent).unwrap();
            costs[i] += w;
        }
    }

    let edges = edges
        .iter()
        .map(|&(u, v, w)| {
            let u = union_find.find(u);
            let v = union_find.find(v);
            assert!(u != v);
            let u = *convert.get(&u).unwrap();
            let v = *convert.get(&v).unwrap();
            (u, v, w)
        }).collect::<Vec<_>>();

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in edges.iter() {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    dfs(0, 0, &graph, &mut costs);
    let sum = uvw.iter().map(|&(_, _, w)| w).sum::<usize>();
    let mut candidates = uvw
        .iter()
        .map(|&(u, v, w)| {
            if union_find.find(u) == union_find.find(v) {
                (sum - w, u, v)
            } else {
                let fu = *convert.get(&union_find.find(u)).unwrap();
                let fv = *convert.get(&union_find.find(v)).unwrap();
                let cost = cmp::min(costs[fu], costs[fv]);
                let other = sum - cost - w;
                (cmp::max(other, cost) - cmp::min(other, cost), u, v)
            }
        }).collect::<Vec<_>>();
    candidates.sort();
    let (_, u, v) = candidates[0];
    println!("{} {}", u + 1, v + 1);
}

fn dfs(v: usize, parent: usize, graph: &Vec<Vec<(usize, usize)>>, costs: &mut Vec<usize>) -> usize {
    for &(child, w) in graph[v].iter() {
        if child != parent {
            let c = dfs(child, v, graph, costs);
            costs[v] += c + w;
        }
    }
    costs[v]
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
