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
    input!(n: usize, m: usize, edges: [(usize1, usize1, usize); m]);

    let mut graph = vec![vec![]; n];
    for &(u, v, _) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let bridge_detector = BridgeDetector::new(&graph);
    let mut bridges = BTreeSet::new();
    for &(a, b) in bridge_detector.bridges.iter() {
        bridges.insert((a, b));
        bridges.insert((b, a));
    }

    let (find, small_n) = {
        let mut uf = UnionFind::new(n);
        for &(u, v, _) in edges.iter() {
            if bridges.contains(&(u, v)) {
                continue;
            }
            uf.unite(u, v);
        }

        let mut find = vec![0; n];
        let mut set = BTreeSet::new();
        for i in 0..n {
            let cur = set.len();
            let f = uf.find(i);
            if set.contains(&f) {
                find[i] = find[f];
            } else {
                set.insert(f);
                find[f] = cur;
                find[i] = cur;
            }
        }
        (find, set.len())
    };

    let mut tree = vec![vec![]; small_n];
    let mut costs = vec![0; small_n];

    let mut total = 0;
    for &(u, v, w) in edges.iter() {
        total += w;
        if bridges.contains(&(u, v)) {
            let fu = find[u];
            let fv = find[v];
            tree[fv].push((fu, w));
            tree[fu].push((fv, w));
        } else {
            costs[find[v]] += w;
        }
    }

    let mut dp = vec![0; small_n];
    dfs(0, 0, &tree, &costs, &mut dp);
    assert_eq!(dp[0], total);

    let mut ans = vec![];
    for &(u, v, w) in edges.iter() {
        if bridges.contains(&(u, v)) {
            let fu = find[u];
            let fv = find[v];
            let min = cmp::min(dp[fu], dp[fv]);
            let other = total - min - w;
            ans.push((cmp::max(other, min) - cmp::min(other, min), u, v));
        } else {
            ans.push((total - w, u, v));
        }
    }

    ans.sort();
    let (w, u, v) = ans[0];
    println!("{} {}", u + 1, v + 1);
}

fn dfs(
    v: usize,
    p: usize,
    tree: &Vec<Vec<(usize, usize)>>,
    costs: &Vec<usize>,
    dp: &mut Vec<usize>,
) -> usize {
    if dp[v] > 0 {
        return dp[v];
    }
    dp[v] = costs[v];
    for &(next, w) in tree[v].iter() {
        if next == p {
            continue;
        }
        dp[v] += dfs(next, v, tree, costs, dp) + w;
    }
    dp[v]
}

pub struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    k: usize,
}

impl BridgeDetector {
    pub fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let mut d = BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            ord: vec![0; n],
            low: vec![0; n],
            k: 0,
        };
        d.run(graph);
        d
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
