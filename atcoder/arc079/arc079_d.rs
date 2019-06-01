use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let mut parents = vec![0; n];
    let mut graph = vec![vec![]; n];
    for to in 0..n {
        let from = sc.read::<usize>() - 1;
        graph[from].push(to);
        parents[to] = from;
    }

    let mut vis = vec![false; n];
    let mut cur = 0;
    while !vis[cur] {
        vis[cur] = true;
        cur = parents[cur];
    }

    let mut is_cycle = vec![false; n];
    let start = cur;
    is_cycle[start] = true;
    cur = parents[cur];
    while start != cur {
        is_cycle[cur] = true;
        cur = parents[cur];
    }

    let mut grundy = vec![n; n];
    for i in 0..n {
        if is_cycle[i] {
            for &next in graph[i].iter() {
                if is_cycle[next] {
                    continue;
                }
                assign_dfs(next, &graph, &mut grundy);
            }
        }
    }

    grundy[start] = {
        let set = graph[start]
            .iter()
            .filter(|&&next| !is_cycle[next])
            .map(|&next| grundy[next])
            .collect::<BTreeSet<_>>();
        (0..).find(|v| !set.contains(v)).unwrap()
    };
    let mut cur = parents[start];
    while cur != start {
        grundy[cur] = get_grundy(cur, &graph, &grundy);
        cur = parents[cur];
    }

    if get_grundy(start, &graph, &grundy) == grundy[start] {
        println!("POSSIBLE");
        return;
    }

    grundy[start] = {
        let set = graph[start]
            .iter()
            .filter(|&&next| !is_cycle[next])
            .map(|&next| grundy[next])
            .collect::<BTreeSet<_>>();
        (0..).filter(|v| !set.contains(v)).skip(1).next().unwrap()
    };

    let mut cur = parents[start];
    while cur != start {
        grundy[cur] = get_grundy(cur, &graph, &grundy);
        cur = parents[cur];
    }

    if get_grundy(start, &graph, &grundy) == grundy[start] {
        println!("POSSIBLE");
        return;
    }
    println!("IMPOSSIBLE");
}

fn get_grundy(v: usize, graph: &Vec<Vec<usize>>, grundy: &Vec<usize>) -> usize {
    let set = graph[v]
        .iter()
        .map(|&next| grundy[next])
        .collect::<BTreeSet<_>>();
    (0..).find(|g| !set.contains(g)).unwrap()
}

fn assign_dfs(v: usize, graph: &Vec<Vec<usize>>, grundy: &mut Vec<usize>) {
    for &next in graph[v].iter() {
        assign_dfs(next, graph, grundy);
    }
    grundy[v] = get_grundy(v, graph, grundy);
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
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
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
