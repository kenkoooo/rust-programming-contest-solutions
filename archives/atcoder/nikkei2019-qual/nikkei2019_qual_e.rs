use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let m = sc.read();
    let x: Vec<u64> = sc.vec(n);

    let mut edges = (0..m)
        .map(|_| {
            let a = sc.read::<usize>() - 1;
            let b = sc.read::<usize>() - 1;
            let w = sc.read::<u64>();
            (w, a, b)
        })
        .collect::<Vec<_>>();
    edges.sort();

    let mut uf = UnionFind::new(n, &x);

    let mut ok = VecDeque::new();
    let mut cur = 0;
    while cur < m {
        let (cost, _, _) = edges[cur];
        let from = cur;
        while cur < m && edges[cur].0 == cost {
            cur += 1;
        }

        for i in from..cur {
            let (_, a, b) = edges[i];
            uf.unite(a, b);
        }
        for i in from..cur {
            let (w, a, _) = edges[i];
            let root = uf.find(a);
            let sum = uf.weights[root];
            if sum >= w {
                ok.push_front(i);
            }
        }
    }

    let mut graph = vec![vec![]; n];
    for (i, &(w, a, b)) in edges.iter().enumerate() {
        graph[a].push((b, w, i));
        graph[b].push((a, w, i));
    }

    let mut used = vec![false; m];
    while let Some(i) = ok.pop_front() {
        let (w, a, b) = edges[i];
        dfs(a, &graph, &mut used, w);
        dfs(b, &graph, &mut used, w);
    }

    let used_count = used.into_iter().filter(|&x| x).count();
    println!("{}", m - used_count);
}

fn dfs(v: usize, graph: &Vec<Vec<(usize, u64, usize)>>, used: &mut Vec<bool>, max_weight: u64) {
    for &(next, edge_weight, edge_id) in graph[v].iter() {
        if used[edge_id] || edge_weight > max_weight {
            continue;
        }
        used[edge_id] = true;
        dfs(next, graph, used, max_weight);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    weights: Vec<u64>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize, x: &Vec<u64>) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            weights: x.clone(),
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
        self.weights[large] += self.weights[small];
        self.weights[small] = 0;
        self.size -= 1;
        return true;
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
