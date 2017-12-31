use std::io;

fn main() {
    let (n, m) = {
        let v = read_values::<usize>();
        (v[0], v[1])
    };
    let (a, b) = {
        let mut a = vec![0; m];
        let mut b = vec![0; m];
        for i in 0..m {
            let v = read_values::<usize>();
            a[i] = v[0] - 1;
            b[i] = v[1] - 1;
        }
        (a, b)
    };
    let q = read_values::<usize>()[0];
    let (x, y, z) = {
        let mut x = vec![0; q];
        let mut y = vec![0; q];
        let mut z = vec![0; q];
        for i in 0..q {
            let v = read_values::<usize>();
            x[i] = v[0] - 1;
            y[i] = v[1] - 1;
            z[i] = v[2];
        }
        (x, y, z)
    };


    let mut ng = vec![0; q];
    let mut ok = vec![m; q];

    for _ in 0..30 {
        let mut check: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
        for i in 0..q {
            let med = (ng[i] + ok[i]) / 2;
            check[med].push(i);
        }

        let mut uf = UnionFind::new(n);
        for edge_id in 0..m {
            uf.unite(a[edge_id], b[edge_id]);
            for query in check[edge_id].iter() {
                let from = uf.find(x[*query]);
                let to = uf.find(y[*query]);
                let count = if from == to {
                    uf.sizes[from]
                } else {
                    uf.sizes[from] + uf.sizes[to]
                };

                if count >= z[*query] {
                    ok[*query] = edge_id;
                } else {
                    ng[*query] = edge_id;
                }
            }
        }
    }

    for i in 0..q {
        println!("{}", ok[i] + 1);
    }
}

struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| { i }).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
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

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}