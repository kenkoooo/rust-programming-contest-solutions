use std::collections::{BTreeMap, BTreeSet, VecDeque};

const OUTSIDE: usize = 0;
const MAX_DIST: usize = 1_000_000;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut graph = BTreeMap::new();
    check(&mut graph, (0, OUTSIDE));

    let mut vertices = BTreeSet::new();

    for _ in 0..m {
        let p = sc.read::<usize>() - 1;
        let q = sc.read::<usize>() - 1;
        let c = sc.read::<usize>();

        check(&mut graph, (p, c));
        check(&mut graph, (q, c));
        check(&mut graph, (p, OUTSIDE));
        check(&mut graph, (q, OUTSIDE));

        graph.get_mut(&(p, c)).unwrap().push((q, c, 0));
        graph.get_mut(&(q, c)).unwrap().push((p, c, 0));

        vertices.insert((p, c));
        vertices.insert((q, c));
    }

    for &(v, train) in &vertices {
        graph.get_mut(&(v, train)).unwrap().push((v, OUTSIDE, 0));
        graph.get_mut(&(v, OUTSIDE)).unwrap().push((v, train, 1));
    }

    let mut dist = BTreeMap::new();
    let mut queue = VecDeque::new();
    dist.insert((0, OUTSIDE), 0);
    queue.push_back((0, OUTSIDE));
    while let Some((v, train)) = queue.pop_front() {
        if v == n - 1 {
            println!("{}", dist[&(v, train)]);
            return;
        }

        for &(next_v, next_train, cost) in &graph[&(v, train)] {
            let cur_dist = match dist.get(&(next_v, next_train)) {
                Some(&d) => d,
                _ => MAX_DIST,
            };
            let next_dist = dist[&(v, train)] + cost;
            if cur_dist <= next_dist {
                continue;
            }
            dist.insert((next_v, next_train), next_dist);
            if cost == 0 {
                queue.push_front((next_v, next_train));
            } else {
                queue.push_back((next_v, next_train));
            }
        }
    }

    println!("-1");
}

fn check(graph: &mut BTreeMap<(usize, usize), Vec<(usize, usize, usize)>>, to: (usize, usize)) {
    if !graph.contains_key(&to) {
        graph.insert(to, Vec::new());
    }
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
