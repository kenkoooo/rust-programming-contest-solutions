fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut x: Vec<usize> = vec![];
    let mut y: Vec<usize> = vec![];
    for _ in 0..n {
        x.push(sc.read::<usize>() - 1);
        y.push(sc.read::<usize>() - 1);
    }

    let x_upper = *x.iter().max().unwrap() + 1;
    let y_upper = *y.iter().max().unwrap() + 1;
    let mut uf = UnionFind::new(x_upper + y_upper);
    for i in 0..n {
        uf.unite(x[i], x_upper + y[i]);
    }

    let mut x_count: Vec<usize> = vec![0; x_upper + y_upper];
    let mut y_count: Vec<usize> = vec![0; x_upper + y_upper];
    let mut included = vec![0; x_upper + y_upper];
    for x in 0..x_upper {
        let x = uf.find(x);
        x_count[x] += 1;
    }
    for y in 0..y_upper {
        let y = uf.find(y + x_upper);
        y_count[y] += 1;
    }
    for i in 0..n {
        let root = uf.find(x[i]);
        included[root] += 1;
    }

    let mut total = 0;
    for root in 0..(x_upper + y_upper) {
        if x_count[root] >= 2 && y_count[root] >= 2 {
            total += x_count[root] * y_count[root] - included[root];
        }
    }

    println!("{}", total);
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
