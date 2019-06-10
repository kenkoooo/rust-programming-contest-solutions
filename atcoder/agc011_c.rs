fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        let u = sc.read::<usize>() - 1;
        let v = sc.read::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
        uf.unite(u, v);
    }

    let mut vis = vec![false; n];
    let mut color = vec![2; n];
    let mut single = 0;
    let mut bipartite = 0;
    let mut non_bipartite = 0;

    for i in 0..n {
        let root = uf.find(i);
        if !vis[root] {
            vis[root] = true;
            color[root] = 1;
            if uf.sizes[root] == 1 {
                single += 1;
            } else if color_dfs(root, &graph, &mut color) {
                bipartite += 1;
            } else {
                non_bipartite += 1;
            }
        }
    }

    let mut ans = 0;
    ans += single * n * 2 - single * single;
    ans += non_bipartite * non_bipartite + non_bipartite * bipartite * 2;
    ans += bipartite * bipartite * 2;
    println!("{}", ans);
}

fn color_dfs(v: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<usize>) -> bool {
    let next_color = color[v] ^ 1;
    for &next in graph[v].iter() {
        if color[next] == color[v] {
            return false;
        }
        if color[next] != next_color {
            color[next] = next_color;
            if !color_dfs(next, graph, color) {
                return false;
            }
        }
    }
    true
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
