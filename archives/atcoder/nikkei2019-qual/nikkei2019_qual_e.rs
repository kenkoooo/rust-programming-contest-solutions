fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let m = sc.read();
    let x: Vec<u64> = sc.vec(n);

    let mut edges = vec![];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let w: u64 = sc.read();
        edges.push((w, a, b));
    }

    edges.sort();
    let mut uf = UnionFind::new(n, x);
    let mut is_candidate = vec![false; m];

    let mut i = 0;
    while i < m {
        let (w, _, _) = edges[i];
        let from = i;
        while i + 1 < m && edges[i + 1].0 == w {
            i += 1;
        }

        for (_, a, b) in (from..(i + 1)).map(|i| edges[i]) {
            uf.unite(a, b);
        }
        for i in from..(i + 1) {
            let (_, a, _) = edges[i];
            let root = uf.find(a);
            if uf.weights[root] >= w {
                is_candidate[i] = true;
            }
        }
        i += 1;
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (w, a, b) = edges[i];
        graph[a].push((b, w, i));
        graph[b].push((a, w, i));
    }

    let mut is_used = vec![false; m];
    for i in (0..m).rev() {
        if is_used[i] || !is_candidate[i] {
            continue;
        }
        let (w, a, b) = edges[i];
        is_used[i] = true;
        dfs(a, w, &mut is_used, &graph);
        dfs(b, w, &mut is_used, &graph);
    }

    println!("{}", is_used.into_iter().filter(|&t| !t).count());
}

fn dfs(v: usize, w: u64, used: &mut Vec<bool>, graph: &Vec<Vec<(usize, u64, usize)>>) {
    for &(next, weight, i) in graph[v].iter() {
        if used[i] || weight > w {
            continue;
        }
        used[i] = true;
        dfs(next, w, used, graph);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    weights: Vec<u64>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize, weights: Vec<u64>) -> Self {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            weights: weights,
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
