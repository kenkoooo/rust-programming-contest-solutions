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

    let mut dif = 0;
    if x.len() >= 2 && x[1] < 0 {
        dif -= x[1];
        x.remove(0);
        for x in x.iter_mut() {
            *x += dif;
        }
    }

    let mut s = BTreeSet::new();
    let mut candidates = BTreeSet::new();
    let mut distance = vec![0; x.len() - 1];
    for i in 0..x.len() {
        s.insert(i);
        if i > 0 {
            distance[i - 1] = (x[i] - x[i - 1]).abs();
            dif += distance[i - 1];
            candidates.insert((distance[i - 1], i - 1));
        }
    }

    let mut vv = vec![];
    for i in 0..q {
        vv.push((sc.read::<i64>(), i));
    }
    vv.sort();
    let mut pre = 0;
    let mut ans = vec![0; q];
    for &(v_first, v_second) in vv.iter() {
        while let Some(&(f, t)) = candidates.iter().next() {
            if f > v_first {
                break;
            }

            assert!(candidates.remove(&(f, t)));

            dif -= distance[t] - pre;
            let &y = s.range((t + 1)..).next().unwrap();
            if y < *s.last() {
                candidates.remove(&(distance[y], y));
                dif -= distance[y] - pre;
            }
            s.remove(&y);
            let it = s.range((t + 1)..).next().cloned();
            if it.is_none() {
                continue;
            }
            let y = it.unwrap();
            assert!(t % 2 == y % 2);

            if (t % 2 == 0 && x[y] >= x[t]) || (t % 2 == 1 && x[y] <= x[t]) {
                if y < *s.last() {
                    candidates.remove(&(distance[y], y));
                    dif -= distance[y] - pre;
                }
                s.remove(&y);

                match s.range((t + 1)..).next() {
                    Some(&it) => {
                        distance[t] = (x[t] - x[it]).abs();
                        candidates.insert((distance[t], t));
                        dif += distance[t] - pre;
                    }
                    _ => {}
                }
            } else {
                assert!(s.remove(&t));
                match s.range(..y).next_back() {
                    Some(&t) => {
                        candidates.remove(&(distance[t], t));
                        dif -= distance[t] - pre;
                        distance[t] = (x[t] - x[y]).abs();
                        candidates.insert((distance[t], t));
                        dif += distance[t] - pre;
                    }
                    _ => {
                        dif += (x[t] - x[y]).abs();
                    }
                }
            }
        }

        dif -= (v_first - pre) * candidates.len() as i64;
        ans[v_second] = dif;
        pre = v_first;
    }

    for &ans in ans.iter() {
        println!("{}", ans);
    }
}

trait LastElement<K> {
    fn first(&self) -> &K;
    fn last(&self) -> &K;
}

impl LastElement<usize> for BTreeSet<usize> {
    fn first(&self) -> &usize {
        self.iter().next().unwrap()
    }
    fn last(&self) -> &usize {
        self.iter().next_back().unwrap()
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
