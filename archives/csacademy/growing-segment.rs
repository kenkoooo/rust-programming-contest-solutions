use std::collections::BTreeSet;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.read();
    let q = sc.read();
    let mut x: Vec<i64> = vec![0];
    for _ in 0..n {
        let t = sc.read::<i64>();
        if x[x.len() - 1] == t {
            continue;
        }
        if x.len() >= 2 && (x[x.len() - 2] < x[x.len() - 1]) == (x[x.len() - 1] < t) {
            x.pop();
        }
        x.push(t);
    }

    let mut current_difference = 0;
    if x.len() >= 2 && x[1] < 0 {
        current_difference -= x[1];
        x.remove(0);
        for x in x.iter_mut() {
            *x += current_difference;
        }
    }

    // x[even] < x[odd] > x[even] < x[odd] ...

    let mut index_set = (0..x.len()).collect::<BTreeSet<_>>();
    let mut edges = BTreeSet::new();
    let mut distance = vec![0; x.len() - 1];
    for i in 1..x.len() {
        distance[i - 1] = (x[i] - x[i - 1]).abs();
        current_difference += distance[i - 1];
        edges.insert((distance[i - 1], i - 1));
    }

    let mut queries = vec![];
    for i in 0..q {
        queries.push((sc.read::<i64>(), i));
    }
    queries.sort();

    let mut prev_query = 0;
    let mut ans = vec![0; q];
    for &(query, ans_index) in queries.iter() {
        while let Some(&(_, edge_idx)) = edges.first() {
            let edge_length = distance[edge_idx];
            if edge_length > query {
                break;
            }
            edges.remove(&(edge_length, edge_idx));
            current_difference -= distance[edge_idx] - prev_query;

            let &next_idx = index_set.range((edge_idx + 1)..).next().unwrap();
            if next_idx < *index_set.last().unwrap() {
                edges.remove(&(distance[next_idx], next_idx));
                current_difference -= distance[next_idx] - prev_query;
            }
            index_set.remove(&next_idx);
            let it = index_set.range((edge_idx + 1)..).next().cloned();
            if it.is_none() {
                continue;
            }

            let next_next_idx = it.unwrap();
            assert!(edge_idx % 2 == next_next_idx % 2);

            if (edge_idx % 2 == 0 && x[next_next_idx] >= x[edge_idx])
                || (edge_idx % 2 == 1 && x[next_next_idx] <= x[edge_idx])
            {
                // When next_next_idx is removable,
                if next_next_idx < *index_set.last().unwrap() {
                    edges.remove(&(distance[next_next_idx], next_next_idx));
                    current_difference -= distance[next_next_idx] - prev_query;
                }
                index_set.remove(&next_next_idx);

                match index_set.range((edge_idx + 1)..).next() {
                    Some(&next_idx) => {
                        distance[edge_idx] = (x[edge_idx] - x[next_idx]).abs();
                        edges.insert((distance[edge_idx], edge_idx));
                        current_difference += distance[edge_idx] - prev_query;
                    }
                    _ => {}
                }
            } else {
                // When next_next_idx can not be removed,
                assert!(index_set.remove(&edge_idx));
                match index_set.range(..next_next_idx).next_back() {
                    Some(&prev_idx) => {
                        edges.remove(&(distance[prev_idx], prev_idx));
                        current_difference -= distance[prev_idx] - prev_query;

                        distance[prev_idx] = (x[prev_idx] - x[next_next_idx]).abs();
                        edges.insert((distance[prev_idx], prev_idx));
                        current_difference += distance[prev_idx] - prev_query;
                    }
                    _ => {
                        current_difference += (x[edge_idx] - x[next_next_idx]).abs();
                    }
                }
            }
        }

        current_difference -= (query - prev_query) * edges.len() as i64;
        ans[ans_index] = current_difference;
        prev_query = query;
    }

    for &ans in ans.iter() {
        println!("{}", ans);
    }
}

trait FirstLastElement<K> {
    fn first(&self) -> Option<&K>;
    fn last(&self) -> Option<&K>;
}

impl<K> FirstLastElement<K> for BTreeSet<K> {
    fn first(&self) -> Option<&K> {
        self.iter().next()
    }
    fn last(&self) -> Option<&K> {
        self.iter().next_back()
    }
}

struct Scanner {
    ptr: usize,
    length: usize,
    buf: Vec<u8>,
    small_cache: Vec<u8>,
}

#[allow(dead_code)]
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

    fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        (0..n).map(|_| self.read()).collect()
    }

    fn usize_read(&mut self) -> usize {
        self.read()
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
