use std::collections::{BTreeMap, BinaryHeap};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let n: usize = sc.read();

    let mut graph = vec![vec![]; (1 << (n + 1)) - 1];
    let mut inverse = vec![vec![]; (1 << (n + 1)) - 1];
    for i in 0..((1 << n) - 1) {
        graph[i].push(i * 2 + 1);
        graph[i].push(i * 2 + 2);
        inverse[i * 2 + 1].push(i);
        inverse[i * 2 + 2].push(i);
    }

    let v: Vec<u64> = sc.vec(1 << n);
    let count = v.into_iter().fold(BTreeMap::new(), |mut map, v| {
        *map.entry(v).or_insert(0) += 1;
        map
    });

    let mut values = vec![0; graph.len()];
    let mut heap = BinaryHeap::new();
    heap.push((1 << n, 0));

    for (value, count) in count.into_iter().rev() {
        if heap.len() < count {
            println!("No");
            return;
        }
        let vs = (0..count).map(|_| heap.pop().unwrap()).collect::<Vec<_>>();
        for (mut size, mut cur) in vs.into_iter() {
            loop {
                values[cur] = value;
                if graph[cur].is_empty() {
                    break;
                }
                heap.push((size / 2, graph[cur][1]));
                cur = graph[cur][0];
                size /= 2;
            }
        }
    }

    println!("Yes");
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
