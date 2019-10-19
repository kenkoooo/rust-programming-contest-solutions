use std::collections::{BTreeMap, BinaryHeap};

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let n: usize = sc.read();
    let s: Vec<usize> = sc.vec(1 << n);

    let mut tree = vec![vec![]; (1 << (n + 1)) - 1];
    for i in 0..((1 << n) - 1) {
        tree[i].push(2 * i + 1);
        tree[i].push(2 * i + 2);
    }

    let count = s.into_iter().fold(BTreeMap::new(), |mut map, s| {
        *map.entry(s).or_insert(0) += 1;
        map
    });

    let mut heap = BinaryHeap::new();
    heap.push((1 << n, 0));
    for (_, count) in count.into_iter().rev() {
        if heap.len() < count {
            println!("No");
            return;
        }
        let vs = (0..count).map(|_| heap.pop().unwrap()).collect::<Vec<_>>();
        for (mut size, mut cur) in vs.into_iter() {
            while !tree[cur].is_empty() {
                heap.push((size / 2, tree[cur][1]));
                cur = tree[cur][0];
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
