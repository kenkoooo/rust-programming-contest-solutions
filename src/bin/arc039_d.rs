/// Thank you tanakh!!!
///  https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

use std::cmp;
use std::collections::{BTreeSet, VecDeque};

fn main() {
    input!(
        n: usize,
        m: usize,
        edges: [(usize1, usize1); m],
        q: usize,
        queries: [(usize1, usize1, usize1); q]
    );

    let mut graph = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut bridge_detector = BridgeDetector::new(n);
    bridge_detector.run(&graph);

    let mut bridges = BTreeSet::new();
    for &(a, b) in bridge_detector.bridges.iter() {
        bridges.insert((cmp::min(a, b), cmp::max(a, b)));
    }

    let mut cur = 1;
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        let mut q = VecDeque::new();
        q.push_back(i);
        color[i] = cur;
        while let Some(v) = q.pop_back() {
            for &u in graph[v].iter() {
                if bridges.contains(&(cmp::min(u, v), cmp::max(u, v))) {
                    continue;
                }
                if color[u] != 0 {
                    continue;
                }
                q.push_back(u);
                color[u] = cur;
            }
        }
        cur += 1;
    }

    for &(a, b, c) in queries.iter() {
        if color[a] != color[b] && color[a] == color[c] {
            println!("NG");
        } else {
            println!("OK");
        }
    }
}

struct BridgeDetector {
    articulations: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    visit: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    k: usize,
}

impl BridgeDetector {
    fn new(n: usize) -> Self {
        BridgeDetector {
            articulations: vec![],
            bridges: vec![],
            visit: vec![false; n],
            ord: vec![0; n],
            low: vec![0; n],
            k: 0,
        }
    }

    fn run(&mut self, graph: &Vec<Vec<usize>>) {
        let n = graph.len();
        for i in 0..n {
            if !self.visit[i] {
                self.dfs(i, None, graph);
            }
        }
    }
    fn dfs(&mut self, v: usize, p: Option<usize>, graph: &Vec<Vec<usize>>) {
        self.visit[v] = true;
        self.ord[v] = self.k;
        self.k += 1;
        self.low[v] = self.ord[v];

        let mut is_articulation = false;
        let mut count = 0;
        for &next in graph[v].iter() {
            if !self.visit[next] {
                count += 1;
                self.dfs(next, Some(v), graph);
                if self.low[v] > self.low[next] {
                    self.low[v] = self.low[next];
                }
                if p.is_some() && self.ord[v] <= self.low[next] {
                    is_articulation = true;
                }
                if self.ord[v] < self.low[next] {
                    let (v, next) = if v < next { (v, next) } else { (next, v) };
                    self.bridges.push((v, next));
                }
            } else if p.is_none() || next != p.unwrap() && self.low[v] > self.ord[next] {
                self.low[v] = self.ord[next];
            }
        }

        if p.is_none() && count > 1 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulations.push(v);
        }
    }
}
