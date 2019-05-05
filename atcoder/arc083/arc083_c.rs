use std::cmp;
use std::collections::BTreeSet;

const INF: usize = 1e16 as usize;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let p: Vec<usize> = sc.read_vec(n - 1);
    let x: Vec<usize> = sc.read_vec(n);

    let mut graph = vec![vec![]; n];
    for i in 1..n {
        let parent = p[i - 1] - 1;
        graph[i].push(parent);
        graph[parent].push(i);
    }

    if dfs(0, 0, &graph, &x) < INF {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}

fn dfs(v: usize, p: usize, tree: &Vec<Vec<usize>>, x: &Vec<usize>) -> usize {
    if tree[v].len() == 1 && v != 0 {
        return 0;
    }

    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    set.insert((0, 0));
    for &next in tree[v].iter() {
        if next != p {
            let mut next_set = BTreeSet::new();
            let black = x[next];
            let white = dfs(next, v, tree, x);
            for &(b, w) in set.iter() {
                next_set.insert((b + black, w + white));
                next_set.insert((w + black, b + white));
            }
            set = next_set;
        }
    }

    let min1 = set
        .iter()
        .filter(|&&(_, y)| y <= x[v])
        .map(|&(x, _)| x)
        .min()
        .unwrap_or(INF);
    let min2 = set
        .iter()
        .filter(|&&(y, _)| y <= x[v])
        .map(|&(_, y)| y)
        .min()
        .unwrap_or(INF);
    cmp::min(min1, min2)
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
