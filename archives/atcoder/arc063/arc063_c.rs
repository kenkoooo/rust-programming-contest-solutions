use std::collections::{BinaryHeap, VecDeque};

const INF: i64 = 1e15 as i64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let k: usize = sc.read();
    let mut has = vec![false; n];
    let mut min = vec![-INF; n];
    let mut max = vec![INF; n];
    for _ in 0..k {
        let v = sc.read::<usize>() - 1;
        let p: i64 = sc.read();
        has[v] = true;
        min[v] = p;
        max[v] = p;
    }

    let mut q = BinaryHeap::new();
    for i in 0..n {
        if has[i] {
            q.push((min[i], i));
        }
    }
    while let Some((_, v)) = q.pop() {
        for &next in graph[v].iter() {
            if min[next] < min[v] - 1 {
                min[next] = min[v] - 1;
                q.push((min[next], next));
            }
        }
    }

    let mut q = BinaryHeap::new();
    for i in 0..n {
        if has[i] {
            q.push((-max[i], i));
        }
    }
    while let Some((_, v)) = q.pop() {
        for &next in graph[v].iter() {
            if max[next] > max[v] + 1 {
                max[next] = max[v] + 1;
                q.push((-max[next], next));
            }
        }
    }

    if (0..n).any(|i| min[i] > max[i]) {
        println!("No");
        return;
    }

    let mut q = VecDeque::new();
    for v in 0..n {
        if has[v] {
            q.push_back(v);
        }
    }

    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            let upper = min[v] + 1;
            let lower = min[v] - 1;

            if has[next] {
                if upper != min[next] && lower != min[next] {
                    println!("No");
                    return;
                }
            } else {
                if min[next] <= lower && lower <= max[next] {
                    min[next] = lower;
                    max[next] = lower;
                } else if min[next] <= upper && upper <= max[next] {
                    min[next] = upper;
                    max[next] = upper;
                } else {
                    println!("No");
                    return;
                }

                has[next] = true;
                q.push_back(next);
            }
        }
    }

    println!("Yes");
    for i in 0..n {
        assert_eq!(min[i], max[i]);
        println!("{}", min[i]);
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
