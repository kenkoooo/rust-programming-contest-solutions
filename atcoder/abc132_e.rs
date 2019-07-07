use std::collections::VecDeque;

const INF: u64 = std::u64::MAX / 2;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let v = sc.read::<usize>() - 1;
        let u = sc.read::<usize>() - 1;
        graph[v].push(u);
    }
    let s = sc.read::<usize>() - 1;
    let t = sc.read::<usize>() - 1;

    let mut dist = vec![vec![INF; n]; 3];
    dist[0][s] = 0;

    let mut q = VecDeque::new();
    q.push_back((s, 0));
    while let Some((v, step)) = q.pop_front() {
        if v == t && step == 0 {
            println!("{}", dist[step][t] / 3);
            return;
        }
        let next_step = (step + 1) % 3;
        for &next in graph[v].iter() {
            if dist[next_step][next] > dist[step][v] + 1 {
                dist[next_step][next] = dist[step][v] + 1;
                q.push_back((next, next_step));
            }
        }
    }
    println!("-1");
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
