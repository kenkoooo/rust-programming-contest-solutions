use std::cmp;
use std::collections::{BinaryHeap, VecDeque};

const INF: i64 = 1e16 as i64;

fn main() {
    let sc = std::io::stdin();
    let mut sc = Scanner { reader: sc.lock() };

    let n = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut min = vec![-INF; n];
    let mut max = vec![INF; n];
    let mut number = vec![0; n];
    let mut is_numbered = vec![false; n];
    let k = sc.read();
    for _ in 0..k {
        let v = sc.read::<usize>() - 1;
        let p: i64 = sc.read();
        is_numbered[v] = true;
        number[v] = p;
    }

    let mut q = BinaryHeap::new();
    for i in 0..n {
        if is_numbered[i] {
            max[i] = number[i];
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

    let mut q = BinaryHeap::new();
    for i in 0..n {
        if is_numbered[i] {
            min[i] = number[i];
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

    let mut root = 0;
    for i in 0..n {
        if is_numbered[i] {
            root = i;
        }
    }

    let mut vis = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(root);
    vis[root] = true;
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if vis[next] {
                continue;
            }
            vis[next] = true;
            q.push_back(next);

            if !is_numbered[next] {
                if min[next] <= number[v] + 1 && number[v] + 1 <= max[next] {
                    number[next] = number[v] + 1;
                } else if min[next] <= number[v] - 1 && number[v] - 1 <= max[next] {
                    number[next] = number[v] - 1;
                } else {
                    println!("No");
                    return;
                }
                is_numbered[next] = true;
            } else {
                if number[next] < min[next] || max[next] < number[next] {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
    for i in 0..n {
        assert!(is_numbered[i]);
        println!("{}", number[i]);
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
