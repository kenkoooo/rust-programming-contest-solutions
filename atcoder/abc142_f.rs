use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
    }

    let bfs = |start: usize, ans: &mut Vec<usize>, min_length: &mut usize| {
        let mut dist = vec![(n, n); n];
        dist[start] = (0, start);
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            for &next in graph[v].iter() {
                if dist[next].0 > dist[v].0 + 1 {
                    dist[next].0 = dist[v].0 + 1;
                    dist[next].1 = v;
                    queue.push_back(next);
                } else if next == start {
                    let mut a = vec![];
                    let mut cur = v;
                    a.push(cur);
                    while cur != start {
                        cur = dist[cur].1;
                        a.push(cur);
                    }
                    if *min_length > a.len() {
                        *min_length = a.len();
                        *ans = a;
                    }
                }
            }
        }
    };
    let mut min_length = n + 1;
    let mut ans: Vec<usize> = vec![];
    for start in 0..n {
        bfs(start, &mut ans, &mut min_length);
    }

    if ans.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans.len());
        for ans in ans.into_iter() {
            println!("{}", ans + 1);
        }
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
