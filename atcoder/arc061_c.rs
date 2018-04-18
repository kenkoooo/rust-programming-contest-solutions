use std::collections::BTreeMap;
use std::collections::VecDeque;

const BASE: u32 = 0;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut graph: BTreeMap<(usize, u32), Vec<(usize, u32, usize)>> = BTreeMap::new();
    for _ in 0..m {
        let p = sc.read::<usize>() - 1;
        let q = sc.read::<usize>() - 1;
        let company: u32 = sc.read();

        add(&mut graph, (p, BASE));
        add(&mut graph, (q, BASE));
        add(&mut graph, (p, company));
        add(&mut graph, (q, company));

        if graph[&(p, company)].is_empty() {
            graph.get_mut(&(p, company)).unwrap().push((p, BASE, 0));
            graph.get_mut(&(p, BASE)).unwrap().push((p, company, 1));
        }
        if graph[&(q, company)].is_empty() {
            graph.get_mut(&(q, company)).unwrap().push((q, BASE, 0));
            graph.get_mut(&(q, BASE)).unwrap().push((q, company, 1));
        }

        graph.get_mut(&(p, company)).unwrap().push((q, company, 0));
        graph.get_mut(&(q, company)).unwrap().push((p, company, 0));
    }

    let start = 0;
    let goal = n - 1;

    add(&mut graph, (start, BASE));
    add(&mut graph, (goal, BASE));

    let mut queue: VecDeque<(usize, u32)> = VecDeque::new();
    let mut dist = BTreeMap::new();
    dist.insert((start, BASE), 0);
    queue.push_front((start, BASE));
    while !queue.is_empty() {
        let (v, company) = queue.pop_front().unwrap();
        let &v_dist = dist.get(&(v, company)).unwrap();
        if v == goal && company == BASE {
            println!("{}", v_dist);
            return;
        }

        for &(to, next_company, edge_cost) in &graph[&(v, company)] {
            let &to_dist = dist.get(&(to, next_company)).unwrap_or(&std::usize::MAX);
            if v_dist + edge_cost >= to_dist {
                continue;
            }
            dist.insert((to, next_company), v_dist + edge_cost);
            if edge_cost == 0 {
                queue.push_front((to, next_company));
            } else {
                queue.push_back((to, next_company));
            }
        }
    }

    println!("-1");
}

fn add<K, T>(map: &mut BTreeMap<K, Vec<T>>, key: K)
where
    K: std::cmp::Ord,
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
