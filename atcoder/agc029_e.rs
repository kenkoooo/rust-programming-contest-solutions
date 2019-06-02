use std::cmp;
use std::collections::BTreeMap;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut max = vec![0; n];
    let mut parent = vec![0; n];
    dfs(0, 0, &mut max, &mut parent, &graph);

    let mut map = BTreeMap::new();
    let mut diff_count = vec![0; n];
    for v in 1..n {
        if v > max[parent[v]] {
            diff_count[v] = query(v, max[v], &graph, &parent, &mut map) + 1;
        } else {
            diff_count[v] = query(v, max[v], &graph, &parent, &mut map)
                - query(v, max[parent[v]], &graph, &parent, &mut map);
        }
    }

    let mut count = vec![0; n];
    count[0] = 1;
    ans_dfs(0, 0, &diff_count, &graph, &mut count);
    for i in 1..n {
        if i > 1 {
            print!(" ");
        }
        print!("{}", count[i] - 1);
    }
    println!();
}

fn ans_dfs(
    v: usize,
    p: usize,
    diff_count: &Vec<usize>,
    graph: &Vec<Vec<usize>>,
    count: &mut Vec<usize>,
) {
    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        count[next] = diff_count[next] + count[v];
        ans_dfs(next, v, diff_count, graph, count);
    }
}

fn query(
    v: usize,
    x: usize,
    graph: &Vec<Vec<usize>>,
    parent: &Vec<usize>,
    map: &mut BTreeMap<(usize, usize), usize>,
) -> usize {
    if map.contains_key(&(v, x)) {
        return *map.get(&(v, x)).unwrap();
    }
    let mut result = 0;
    for &child in graph[v].iter() {
        if parent[child] != v || child >= x {
            continue;
        }
        result += 1 + query(child, x, graph, parent, map);
    }
    map.insert((v, x), result);
    result
}

fn dfs(v: usize, p: usize, max: &mut Vec<usize>, parent: &mut Vec<usize>, graph: &Vec<Vec<usize>>) {
    for &next in graph[v].iter() {
        if p == next {
            continue;
        }
        parent[next] = v;
        max[next] = cmp::max(max[v], v);
        dfs(next, v, max, parent, graph);
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
