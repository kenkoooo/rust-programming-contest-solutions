use std::collections::VecDeque;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    if n == 1 {
        println!("First");
        return;
    }
    if n == 2 {
        println!("Second");
        return;
    }
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut dist = vec![n; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    dist[0] = 0;

    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if dist[next] > dist[v] + 1 {
                dist[next] = dist[v] + 1;
                q.push_back(next);
            }
        }
    }
    let (_, d1) = dist
        .into_iter()
        .enumerate()
        .map(|(i, d)| (d, i))
        .max()
        .unwrap();

    let mut dist = vec![n; n];
    q.push_back(d1);
    dist[d1] = 0;
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if dist[next] > dist[v] + 1 {
                dist[next] = dist[v] + 1;
                q.push_back(next);
            }
        }
    }

    let (dist, _) = dist
        .into_iter()
        .enumerate()
        .map(|(i, d)| (d, i))
        .max()
        .unwrap();

    let inf = n * 10;
    let mut grundy = vec![inf; dist + 1];
    grundy[0] = 1;
    grundy[1] = 0;
    grundy[2] = 1;
    for i in 3..(dist + 1) {
        let mut next = vec![];
        next.push(grundy[i - 1]);
        next.push(grundy[i - 2]);
        grundy[i] = (0..).find(|i| !next.contains(i)).unwrap();
    }
    if grundy[dist] == 0 {
        println!("Second");
    } else {
        println!("First");
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
