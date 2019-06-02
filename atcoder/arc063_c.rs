use std::cmp;
use std::collections::VecDeque;

const INF: i64 = 1e15 as i64;
fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let k: usize = sc.read();

    let mut has_number = vec![false; n];
    let mut min = vec![-INF; n];
    let mut max = vec![INF; n];
    for _ in 0..k {
        let v = sc.read::<usize>() - 1;
        let p: i64 = sc.read();
        has_number[v] = true;
        min[v] = p;
        max[v] = p;
    }

    let mut q = VecDeque::new();
    for i in (0..n).filter(|&i| has_number[i]) {
        q.push_back(i);
    }
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if min[next] < min[v] - 1 {
                min[next] = min[v] - 1;
                q.push_back(next);
            }
        }
    }

    for i in (0..n).filter(|&i| has_number[i]) {
        q.push_back(i);
    }
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if max[next] > max[v] + 1 {
                max[next] = max[v] + 1;
                q.push_back(next);
            }
        }
    }

    let mut ans = vec![INF; n];
    let start = (0..n).find(|&i| has_number[i]).unwrap();
    ans[start] = min[start];
    q.push_back(start);
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if ans[next] == INF {
                if min[next] <= ans[v] - 1 && ans[v] - 1 <= max[next] {
                    ans[next] = ans[v] - 1;
                } else if min[next] <= ans[v] + 1 && ans[v] + 1 <= max[next] {
                    ans[next] = ans[v] + 1;
                } else {
                    println!("No");
                    return;
                }
                q.push_back(next);
            }
        }
    }

    println!("Yes");
    for ans in ans.into_iter() {
        println!("{}", ans);
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
