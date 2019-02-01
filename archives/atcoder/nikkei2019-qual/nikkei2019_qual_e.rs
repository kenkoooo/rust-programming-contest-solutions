fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let m = sc.read();
    let x: Vec<usize> = sc.read_vec(n);

    let mut edges = vec![];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let y: usize = sc.read();
        edges.push((y, a, b));
    }
    edges.sort();

    let mut can_use = vec![false; m];
    let mut uf = UnionFind::new(x);

    let mut cur = 0;
    while cur < m {
        let (cur_weight, _, _) = edges[cur];
        let mut pos = cur;
        while pos < m {
            let (w, a, b) = edges[pos];
            if w != cur_weight {
                break;
            }
            uf.unite(a, b);
            pos += 1;
        }

        let mut pos = cur;
        while pos < m {
            let (w, a, _) = edges[pos];
            if w != cur_weight {
                break;
            }
            let root = uf.find(a);
            if uf.weights[root] >= w {
                can_use[pos] = true;
            }
            pos += 1;
        }

        cur = pos;
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (w, a, b) = edges[i];
        graph[a].push((i, w, b));
        graph[b].push((i, w, a));
    }

    let mut is_used = vec![false; m];
    for i in (0..m).rev() {
        if is_used[i] || !can_use[i] {
            continue;
        }
        let (w, a, b) = edges[i];
        dfs(a, &mut is_used, &graph, w);
        dfs(b, &mut is_used, &graph, w);
    }

    println!("{}", is_used.iter().filter(|&&u| !u).count());
}

fn dfs(
    v: usize,
    is_used: &mut Vec<bool>,
    graph: &Vec<Vec<(usize, usize, usize)>>,
    max_weight: usize,
) {
    for &(edge_id, weight, next) in graph[v].iter() {
        if is_used[edge_id] {
            continue;
        }
        if weight > max_weight {
            continue;
        }
        is_used[edge_id] = true;
        dfs(next, is_used, graph, max_weight);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    weights: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(weights: Vec<usize>) -> UnionFind {
        let n = weights.len();
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            weights: weights,
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
