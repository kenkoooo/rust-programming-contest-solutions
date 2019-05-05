use std::collections::{BTreeMap, VecDeque};

const BASE: usize = 0;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut graph = BTreeMap::new();

    for _ in 0..m {
        let p = sc.read::<usize>() - 1;
        let q = sc.read::<usize>() - 1;
        let c = sc.read::<usize>();

        check_exists(&mut graph, (p, c));
        check_exists(&mut graph, (q, c));

        graph.get_mut(&(p, c)).unwrap().push((q, c, 0));
        graph.get_mut(&(q, c)).unwrap().push((p, c, 0));
    }

    let vertices: Vec<(usize, usize)> = graph.keys().map(|&(v, c)| (v, c)).collect();
    for &(v, c) in &vertices {
        check_exists(&mut graph, (v, BASE));
        graph.get_mut(&(v, BASE)).unwrap().push((v, c, 1));
        graph.get_mut(&(v, c)).unwrap().push((v, BASE, 0));
    }

    if !graph.contains_key(&(0, BASE)) {
        println!("-1");
        return;
    }

    let mut dist = BTreeMap::new();
    dist.insert((0, BASE), 0);
    let mut queue = VecDeque::new();
    queue.push_back((0, BASE, 0));
    while let Some((v, c, d)) = queue.pop_front() {
        if v == n - 1 {
            println!("{}", d);
            return;
        }

        for &(next_v, next_c, cost) in &graph[&(v, c)] {
            if !dist.contains_key(&(next_v, next_c)) || dist[&(next_v, next_c)] > d + cost {
                dist.insert((next_v, next_c), d + cost);
                if cost == 0 {
                    queue.push_front((next_v, next_c, d));
                } else {
                    queue.push_back((next_v, next_c, d + cost));
                }
            }
        }
    }

    println!("-1");
}

fn check_exists<T, S>(map: &mut BTreeMap<T, Vec<S>>, key: T)
where
    T: Ord,
{
    if !map.contains_key(&key) {
        map.insert(key, Vec::new());
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
