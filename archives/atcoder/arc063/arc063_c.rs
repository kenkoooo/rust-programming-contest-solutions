use std::collections::{BinaryHeap, VecDeque};

const INF: i64 = 1e17 as i64;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };

    let n = sc.read();
    let mut graph = vec![vec![]; n];
    for _ in 1..n {
        let a = sc.read::<usize>() - 1;
        let b = sc.read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }

    let k = sc.read();
    let mut num = vec![INF; n];
    let mut has_number = vec![false; n];
    for _ in 0..k {
        let v = sc.read::<usize>() - 1;
        let p: i64 = sc.read();
        has_number[v] = true;
        num[v] = p;
    }

    let mut max = vec![INF; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if has_number[i] {
            heap.push((-num[i], i));
            max[i] = num[i];
        }
    }

    while let Some((_, v)) = heap.pop() {
        for &next in graph[v].iter() {
            if max[next] > max[v] + 1 {
                max[next] = max[v] + 1;
                heap.push((-max[next], next));
            }
        }
    }

    let mut min = vec![-INF; n];
    for i in 0..n {
        if has_number[i] {
            min[i] = num[i];
            heap.push((min[i], i));
        }
    }
    while let Some((_, v)) = heap.pop() {
        for &next in graph[v].iter() {
            if min[next] < min[v] - 1 {
                min[next] = min[v] - 1;
                heap.push((min[next], next));
            }
        }
    }

    let root = (0..n).filter(|&i| has_number[i]).next().unwrap();
    let mut vis = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(root);
    while let Some(v) = q.pop_front() {
        for &next in graph[v].iter() {
            if !vis[next] {
                if has_number[next] {
                    if num[next] < min[next] || max[next] < num[next] {
                        println!("No");
                        return;
                    }
                } else if min[next] <= num[v] + 1 && num[v] + 1 <= max[next] {
                    num[next] = num[v] + 1;
                } else if min[next] <= num[v] - 1 && num[v] - 1 <= max[next] {
                    num[next] = num[v] - 1;
                } else {
                    println!("No");
                    return;
                }
                vis[next] = true;
                q.push_back(next);
            }
        }
    }

    println!("Yes");
    for i in 0..n {
        println!("{}", num[i]);
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
