use std::cmp;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };
    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let k: usize = sc.read();
    let mut has_num = vec![false; n];
    let mut num = vec![0; n];
    let mut root = n;
    for _ in 0..k {
        let v = sc.read::<usize>() - 1;
        num[v] = sc.read::<i64>();
        has_num[v] = true;
        root = v;
    }

    let mut min_max = vec![(0, 0); n];
    dfs(root, root, &graph, &num, &has_num, &mut min_max);
    if !answer_dfs(root, root, &graph, &mut num, &mut min_max) {
        println!("No");
        return;
    } else {
        println!("Yes");
        for i in 0..n {
            println!("{}", num[i]);
        }
    }
}

fn answer_dfs(
    v: usize,
    p: usize,
    graph: &Vec<Vec<usize>>,
    num: &mut Vec<i64>,
    min_max: &mut Vec<(i64, i64)>,
) -> bool {
    let (min, max) = min_max[v];
    let is_root = v == p;
    if !is_root {
        if min <= num[p] + 1 && num[p] + 1 <= max {
            num[v] = num[p] + 1
        } else if min <= num[p] - 1 && num[p] - 1 <= max {
            num[v] = num[p] - 1
        } else {
            return false;
        }
    }

    for &next in graph[v].iter() {
        if next == p {
            continue;
        }
        let (mut next_min, mut next_max) = min_max[next];
        next_min = cmp::max(next_min, num[v] - 1);
        next_max = cmp::min(next_max, num[v] + 1);
        min_max[next] = (next_min, next_max);
        if !answer_dfs(next, v, graph, num, min_max) {
            return false;
        }
    }
    true
}

fn dfs(
    v: usize,
    p: usize,
    graph: &Vec<Vec<usize>>,
    num: &Vec<i64>,
    has_num: &Vec<bool>,
    min_max: &mut Vec<(i64, i64)>,
) {
    let (mut min, mut max) = if has_num[v] {
        (num[v], num[v])
    } else {
        (-1e15 as i64, 1e15 as i64)
    };
    for &next in graph[v].iter() {
        if p == next {
            continue;
        }
        dfs(next, v, graph, num, has_num, min_max);
        let (next_min, next_max) = min_max[next];
        min = cmp::max(next_min - 1, min);
        max = cmp::min(next_max + 1, max);
    }
    min_max[v] = (min, max);
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
