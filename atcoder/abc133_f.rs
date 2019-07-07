use std::collections::{BTreeMap, VecDeque};

const MAX_PARENT: usize = 1 << 50;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut solver = Solver {
        sum: vec![0; n],
        count: vec![0; n],
        dist: vec![0; n],
        count_snapshot: vec![BTreeMap::new(); n],
        sum_snapshot: vec![BTreeMap::new(); n],
    };
    let mut tree = vec![vec![]; n];
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let color = sc.read::<usize>() - 1;
        let dist: u64 = sc.read();
        graph[a].push(b);
        graph[b].push(a);
        tree[a].push(Edge {
            to: b,
            color: color,
            dist: dist,
        });
        tree[b].push(Edge {
            to: a,
            color: color,
            dist: dist,
        });
    }

    let mut queries = vec![];
    let lca = LowestCommonAncestor::new(&graph);
    for _ in 0..q {
        let color = sc.read::<usize>() - 1;
        let dist = sc.read::<u64>();
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        let a = lca.get_lca(u, v);
        queries.push(Query {
            a: a,
            u: u,
            v: v,
            color: color,
            dist: dist,
        });
        solver.count_snapshot[a].insert(color, 0);
        solver.count_snapshot[u].insert(color, 0);
        solver.count_snapshot[v].insert(color, 0);
        solver.sum_snapshot[a].insert(color, 0);
        solver.sum_snapshot[u].insert(color, 0);
        solver.sum_snapshot[v].insert(color, 0);
    }

    solver.dfs(0, 0, &tree);
    for Query {
        a,
        u,
        v,
        color,
        dist,
    } in queries.into_iter()
    {
        let d = solver.dist[u] + solver.dist[v] - 2 * solver.dist[a];
        let sum =
            solver.get_sum(u, color) + solver.get_sum(v, color) - 2 * solver.get_sum(a, color);
        let count = solver.get_count(u, color) + solver.get_count(v, color)
            - 2 * solver.get_count(a, color);

        println!("{}", d - sum + (count as u64) * dist);
    }
}

struct Solver {
    sum: Vec<u64>,
    count: Vec<usize>,
    dist: Vec<u64>,
    count_snapshot: Vec<BTreeMap<usize, usize>>,
    sum_snapshot: Vec<BTreeMap<usize, u64>>,
}

impl Solver {
    fn get_count(&self, v: usize, color: usize) -> usize {
        *self.count_snapshot[v].get(&color).unwrap()
    }
    fn get_sum(&self, v: usize, color: usize) -> u64 {
        *self.sum_snapshot[v].get(&color).unwrap()
    }
    fn dfs(&mut self, v: usize, p: usize, tree: &Vec<Vec<Edge>>) {
        for &Edge { color, to, dist } in tree[v].iter().filter(|e| e.to != p) {
            self.sum[color] += dist;
            self.count[color] += 1;
            self.dist[to] = self.dist[v] + dist;
            for (&key, count) in self.count_snapshot[to].iter_mut() {
                *count = self.count[key];
            }
            for (&key, sum) in self.sum_snapshot[to].iter_mut() {
                *sum = self.sum[key];
            }
            self.dfs(to, v, tree);
            self.sum[color] -= dist;
            self.count[color] -= 1;
        }
    }
}

#[derive(Clone)]
struct Edge {
    to: usize,
    color: usize,
    dist: u64,
}

struct Query {
    u: usize,
    v: usize,
    a: usize,
    color: usize,
    dist: u64,
}

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
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
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
