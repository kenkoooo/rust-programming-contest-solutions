use std::cmp;
use std::collections::VecDeque;

const INF: usize = 1_000_000_000_000_000_000;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let mut tree = vec![vec![]; n];

    let mut edges = Vec::new();
    for _ in 0..(n - 1) {
        let p = sc.read::<usize>() - 1;
        let q = sc.read::<usize>() - 1;
        tree[p].push(q);
        tree[q].push(p);
        edges.push((p, q));
    }

    let mut min = (n + 1) / 2;
    let mut answers = Vec::new();
    for center in 0..n {
        let mut queue = VecDeque::new();
        let mut depth = vec![n; n];
        queue.push_back(center);
        depth[center] = 1;
        let mut max_depth = 1;
        while !queue.is_empty() {
            let s = queue.len();
            for _ in 0..s {
                let p = queue.pop_front().unwrap();
                for &to in &tree[p] {
                    if depth[to] <= depth[p] {
                        continue;
                    }
                    depth[to] = depth[p] + 1;
                    queue.push_back(to);
                    max_depth = cmp::max(max_depth, depth[to]);
                }
            }
        }

        min = cmp::min(min, max_depth);
        if min != max_depth {
            continue;
        }
        let mut max_children = calc_max_children(&tree, &depth, max_depth);
        max_children[0] = 1;
        let mut ans = 1;
        for i in 0..max_depth {
            ans *= max_children[i];
            if ans > INF {
                break;
            }
        }
        if ans <= INF {
            answers.push((min, ans));
        }
    }

    for &(p, q) in &edges {
        let mut queue = VecDeque::new();
        let mut depth = vec![n; n];
        queue.push_back(p);
        queue.push_back(q);
        depth[p] = 1;
        depth[q] = 1;
        let mut max_depth = 1;
        while !queue.is_empty() {
            let s = queue.len();
            for _ in 0..s {
                let p = queue.pop_front().unwrap();
                for &to in &tree[p] {
                    if depth[to] <= depth[p] {
                        continue;
                    }
                    depth[to] = depth[p] + 1;
                    queue.push_back(to);
                    max_depth = cmp::max(max_depth, depth[to]);
                }
            }
        }

        min = cmp::min(min, max_depth);
        if min != max_depth {
            continue;
        }
        let mut max_children = calc_max_children(&tree, &depth, max_depth);
        max_children[0] = 2;
        let mut ans = 1;
        for i in 0..max_depth {
            ans *= max_children[i];
            if ans > INF {
                break;
            }
        }
        if ans <= INF {
            answers.push((min, ans));
        }
    }

    answers.sort();
    let (p, q) = answers[0];

    println!("{} {}", p, q);
}

fn calc_max_children(tree: &Vec<Vec<usize>>, depth: &Vec<usize>, max_depth: usize) -> Vec<usize> {
    let n = tree.len();
    let mut max_children_count = vec![0; max_depth + 1];
    for i in 0..n {
        let mut children = 0;
        for &to in &tree[i] {
            if depth[to] <= depth[i] {
                continue;
            }
            children += 1;
        }
        max_children_count[depth[i]] = cmp::max(max_children_count[depth[i]], children);
    }
    return max_children_count;
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            ptr: 0,
            length: 0,
            buf: vec![0; 1024],
            small_cache: vec![0; 1024],
        }
    }

    fn load(&mut self) {
        use std::io::Read;
        let mut s = std::io::stdin();
        self.length = s.read(&mut self.buf).unwrap();
    }

    fn byte(&mut self) -> u8 {
        if self.ptr >= self.length {
            self.ptr = 0;
            self.load();
            if self.length == 0 {
                self.buf[0] = b'\n';
                self.length = 1;
            }
        }

        self.ptr += 1;
        return self.buf[self.ptr - 1];
    }

    fn is_space(b: u8) -> bool {
        b == b'\n' || b == b'\r' || b == b'\t' || b == b' '
    }

    fn read<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)])
                    .parse()
                    .unwrap();
            }
        }

        let mut v = self.small_cache.clone();
        while !Scanner::is_space(b) {
            v.push(b);
            b = self.byte();
        }
        return String::from_utf8_lossy(&v).parse().unwrap();
    }
}
