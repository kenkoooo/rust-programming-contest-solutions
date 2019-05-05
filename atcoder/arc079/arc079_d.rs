use std::collections::BTreeSet;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let p = (0..n).map(|_| sc.read::<usize>() - 1).collect::<Vec<_>>();
    let mut graph = vec![vec![]; n];
    for (i, &p) in p.iter().enumerate() {
        graph[p].push(i);
    }

    let mut is_cycle = vec![false; n];
    {
        let mut cur = 0;
        let mut vis = vec![false; n];
        while !vis[cur] {
            vis[cur] = true;
            cur = p[cur];
        }

        is_cycle[cur] = true;
        let mut x = p[cur];
        while x != cur {
            is_cycle[x] = true;
            x = p[x];
        }
    }

    let mut grundy = vec![0; n];
    for v in (0..n).filter(|&i| is_cycle[i]) {
        dfs(v, &mut grundy, &graph, &is_cycle);
    }

    let start = (0..n).filter(|&i| is_cycle[i]).next().unwrap();
    let mut start_set = graph[start]
        .iter()
        .filter(|&&next| !is_cycle[next])
        .map(|&next| grundy[next])
        .collect::<BTreeSet<_>>();
    grundy[start] = (0..)
        .skip_while(|&i| start_set.contains(&i))
        .next()
        .unwrap();
    let mut cur = p[start];
    while cur != start {
        let set = graph[cur]
            .iter()
            .map(|&next| grundy[next])
            .collect::<BTreeSet<_>>();
        grundy[cur] = (0..).skip_while(|&i| set.contains(&i)).next().unwrap();
        cur = p[cur];
    }

    {
        let set = graph[start]
            .iter()
            .map(|&next| grundy[next])
            .collect::<BTreeSet<_>>();
        if grundy[start] == (0..).skip_while(|&i| set.contains(&i)).next().unwrap() {
            println!("POSSIBLE");
            return;
        }
    }

    start_set.insert(grundy[start]);
    grundy[start] = (0..)
        .skip_while(|&i| start_set.contains(&i))
        .next()
        .unwrap();
    let mut cur = p[start];
    while cur != start {
        let set = graph[cur]
            .iter()
            .map(|&next| grundy[next])
            .collect::<BTreeSet<_>>();
        grundy[cur] = (0..).skip_while(|&i| set.contains(&i)).next().unwrap();
        cur = p[cur];
    }
    {
        let set = graph[start]
            .iter()
            .map(|&next| grundy[next])
            .collect::<BTreeSet<_>>();
        if grundy[start] == (0..).skip_while(|&i| set.contains(&i)).next().unwrap() {
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}

fn dfs(v: usize, grundy: &mut [usize], graph: &[Vec<usize>], is_cycle: &[bool]) {
    let next_grundy = graph[v]
        .iter()
        .filter(|&&next| !is_cycle[next])
        .map(|&next| {
            dfs(next, grundy, graph, is_cycle);
            grundy[next]
        })
        .collect::<BTreeSet<_>>();
    grundy[v] = (0..)
        .skip_while(|i| next_grundy.contains(&i))
        .next()
        .unwrap();
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
