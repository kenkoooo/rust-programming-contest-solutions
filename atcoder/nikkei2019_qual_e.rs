fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let x: Vec<u64> = sc.vec(n);

    let mut graph = vec![vec![]; n];
    let mut edges = vec![];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        let x: u64 = sc.read();
        edges.push((x, a, b));
    }
    edges.sort();

    for (i, &(x, a, b)) in edges.iter().enumerate() {
        graph[a].push((b, x, i));
        graph[b].push((a, x, i));
    }

    let mut uf = UnionFind::new(n, &x);
    let mut good = vec![false; m];
    let mut cur = 0;
    while cur < m {
        let (lowest, _, _) = edges[cur];
        let from = cur;
        while cur < m && edges[cur].0 == lowest {
            cur += 1;
        }

        for i in from..cur {
            let (_, a, b) = edges[i];
            uf.unite(a, b);
        }

        for i in from..cur {
            let (x, a, _) = edges[i];
            let root = uf.find(a);
            if uf.weights[root] >= x {
                good[i] = true;
            }
        }
    }

    let mut is_ok = vec![false; m];
    for i in (0..m).rev() {
        if good[i] {
            is_ok[i] = true;
            let (x, a, b) = edges[i];
            dfs(a, x, &graph, &mut is_ok);
            dfs(b, x, &graph, &mut is_ok);
        }
    }

    println!("{}", m - is_ok.into_iter().filter(|&b| b).count());
}

fn dfs(v: usize, upper: u64, graph: &Vec<Vec<(usize, u64, usize)>>, is_ok: &mut Vec<bool>) {
    for &(next, weight, i) in graph[v].iter() {
        if weight > upper || is_ok[i] {
            continue;
        }
        is_ok[i] = true;
        dfs(next, upper, graph, is_ok);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
    weights: Vec<u64>,
}

impl UnionFind {
    pub fn new(n: usize, x: &Vec<u64>) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
            weights: x.iter().cloned().collect(),
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
