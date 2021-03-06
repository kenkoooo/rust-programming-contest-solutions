use std::cmp;
use std::collections::BTreeSet;

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

fn main() {
    input!(n: usize, q: usize, queries: [(usize, usize, usize); q]);
    let mut uf = UnionFind::new(n);

    let mut edges = BTreeSet::new();
    let mut graph = vec![vec![]; n];

    for &(t, u, v) in queries.iter() {
        match t {
            1 => {
                let u = u - 1;
                let v = v - 1;
                let (u, v) = (cmp::min(u, v), cmp::max(u, v));

                edges.insert((u, v));

                let u = uf.find(u);
                let v = uf.find(v);
                graph[u].push(v);
                graph[v].push(u);
            }
            2 => {
                let u = uf.find(u - 1);
                complete_dfs(u, &mut uf, &mut graph);
            }
            3 => {
                let u = u - 1;
                let v = v - 1;
                let (u, v) = (cmp::min(u, v), cmp::max(u, v));

                if edges.contains(&(u, v)) || uf.find(u) == uf.find(v) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => unreachable!(),
        }
    }
}

fn complete_dfs(v: usize, uf: &mut UnionFind, graph: &mut Vec<Vec<usize>>) {
    let next = graph[v].clone();
    graph[v].clear();
    for &next in next.iter() {
        uf.unite(next, v);
        complete_dfs(next, uf, graph);
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
