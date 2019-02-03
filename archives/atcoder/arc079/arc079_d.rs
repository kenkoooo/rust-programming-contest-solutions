use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let mut graph = vec![vec![]; n];
    let mut parents = vec![0; n];
    for i in 0..n {
        let p = sc.read::<usize>() - 1;
        graph[p].push(i);
        parents[i] = p;
    }

    let mut vis = vec![false; n];
    let mut cur = 0;
    loop {
        if vis[cur] {
            break;
        }
        vis[cur] = true;
        cur = parents[cur];
    }

    let mut is_cycle = vec![false; n];
    loop {
        if is_cycle[cur] {
            break;
        }
        is_cycle[cur] = true;
        cur = parents[cur];
    }

    let mut grundy = vec![n; n];
    for i in 0..n {
        if is_cycle[i] {
            for &next in graph[i].iter() {
                if !is_cycle[next] {
                    dfs(next, &graph, &mut grundy);
                }
            }
        }
    }

    let head = is_cycle
        .iter()
        .enumerate()
        .filter(|&(_, &is_cycle)| is_cycle)
        .map(|(v, _)| v)
        .next()
        .unwrap();
    let mut set = graph[head]
        .iter()
        .map(|&next| grundy[next])
        .collect::<BTreeSet<usize>>();

    grundy[head] = (0..).filter(|i| !set.contains(i)).next().unwrap();
    if solve(head, &mut grundy, &parents, &graph) {
        println!("POSSIBLE");
        return;
    }

    set.insert(grundy[head]);
    grundy[head] = (0..).filter(|i| !set.contains(i)).next().unwrap();
    if solve(head, &mut grundy, &parents, &graph) {
        println!("POSSIBLE");
        return;
    }
    println!("IMPOSSIBLE");
}

fn solve(
    head: usize,
    grundy: &mut Vec<usize>,
    parents: &Vec<usize>,
    graph: &Vec<Vec<usize>>,
) -> bool {
    let mut cur = parents[head];
    while cur != head {
        let set = graph[cur]
            .iter()
            .map(|&next| grundy[next])
            .collect::<BTreeSet<usize>>();
        grundy[cur] = (0..).filter(|i| !set.contains(i)).next().unwrap();
        cur = parents[cur];
    }
    let set = graph[head]
        .iter()
        .map(|&next| grundy[next])
        .collect::<BTreeSet<usize>>();
    grundy[head] == (0..).filter(|i| !set.contains(i)).next().unwrap()
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, grundy: &mut Vec<usize>) {
    let mut set = BTreeSet::new();
    for &next in graph[v].iter() {
        dfs(next, graph, grundy);
        set.insert(grundy[next]);
    }
    for i in 0.. {
        if set.contains(&i) {
            continue;
        }
        grundy[v] = i;
        break;
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
