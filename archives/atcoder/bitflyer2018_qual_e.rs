use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

const INF: i64 = 1e15 as i64;

fn f(left: i64, right: i64, dist: usize) -> usize {
    let dist = dist as i64;
    if left == right {
        0
    } else if right - left - 1 <= dist {
        (right - left) as usize
    } else {
        1
    }
}

fn insert(date: usize, map: &mut BTreeMap<usize, usize>, dist: usize) -> usize {
    {
        let count: &mut usize = map.entry(date).or_insert(0);
        *count += 1;
        if *count != 1 {
            return 0;
        }
    }

    let lower = match map.range((Unbounded, Excluded(date))).next_back() {
        Some((&lower, _)) => lower as i64,
        _ => -INF,
    };
    let upper = match map.range((Excluded(date), Unbounded)).next() {
        Some((&upper, _)) => upper as i64,
        _ => INF,
    };
    let date = date as i64;
    f(lower, date, dist) + f(date, upper, dist) - f(lower, upper, dist)
}

fn remove(date: usize, map: &mut BTreeMap<usize, usize>, dist: usize) -> usize {
    {
        let count = map.get_mut(&date).unwrap();
        *count -= 1;
        if *count > 0 {
            return 0;
        }
    }

    map.remove(&date);
    let lower = match map.range((Unbounded, Excluded(date))).next_back() {
        Some((&lower, _)) => lower as i64,
        _ => -INF,
    };
    let upper = match map.range((Excluded(date), Unbounded)).next() {
        Some((&upper, _)) => upper as i64,
        _ => INF,
    };
    let date = date as i64;
    f(lower, date, dist) + f(date, upper, dist) - f(lower, upper, dist)
}

fn main() {
    let mut sc = Scanner::new();
    let _ = sc.usize_read();
    let w = sc.usize_read();
    let n = sc.usize_read();
    let m = sc.usize_read();
    let dist = sc.usize_read();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = sc.usize_read() - 1;
    }
    let mut b = vec![0; m];
    let mut c = vec![0; m];
    for i in 0..m {
        b[i] = sc.usize_read() - 1;
        c[i] = sc.usize_read() - 1;
    }
    let mut events = vec![vec![]; w];
    for i in 0..m {
        events[c[i]].push(i);
    }

    let mut map = BTreeMap::new();
    let mut ans = 0;
    for &a in a.iter() {
        ans += insert(a, &mut map, dist);
    }
    for i in 0..m {
        ans += insert(b[i] * w + c[i], &mut map, dist);
    }

    for t in 0..w {
        println!("{}", ans);
        for &a in a.iter() {
            ans -= remove(a + t, &mut map, dist);
        }
        for &a in a.iter() {
            ans += insert(a + t + 1, &mut map, dist);
        }

        for &i in events[t].iter() {
            ans -= remove(b[i] * w + c[i], &mut map, dist);
            ans += insert(b[i] * w + c[i] + w, &mut map, dist);
        }
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
