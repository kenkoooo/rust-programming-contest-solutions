use std::collections::BTreeMap;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let k: usize = sc.read();
    let l: usize = sc.read();

    let mut uf1 = UnionFind::new(n);
    for _ in 0..k {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        uf1.unite(a, b);
    }

    let mut uf2 = UnionFind::new(n);
    for _ in 0..l {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        uf2.unite(a, b);
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        let root1 = uf1.find(i);
        let root2 = uf2.find(i);
        *map.entry((root1, root2)).or_insert(0) += 1;
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        let root1 = uf1.find(i);
        let root2 = uf2.find(i);
        let ans = *map.get(&(root1, root2)).unwrap();
        print!("{}", ans);
    }
    println!();
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
