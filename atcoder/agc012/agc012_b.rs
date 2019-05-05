use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }
    let q: usize = sc.read();
    let mut potential = vec![-1; n];
    let mut color = vec![0; n];
    let vdc: Vec<_> = (0..q)
        .map(|_| (sc.read::<usize>() - 1, sc.read::<i64>(), sc.read::<usize>()))
        .collect();

    for (root, distance, c) in vdc.into_iter().rev() {
        if potential[root] >= distance {
            continue;
        }
        let mut q = VecDeque::new();
        q.push_back(root);
        if color[root] == 0 {
            color[root] = c;
        }
        potential[root] = distance;
        while let Some(v) = q.pop_front() {
            if potential[v] == 0 {
                continue;
            }
            for &next in graph[v].iter() {
                if color[next] == 0 {
                    color[next] = c;
                }
                if potential[next] < potential[v] - 1 {
                    potential[next] = potential[v] - 1;
                    q.push_back(next);
                }
            }
        }
    }

    for c in color.into_iter() {
        println!("{}", c);
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
