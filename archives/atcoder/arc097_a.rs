use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
#[derive(Clone, Eq, PartialEq)]
struct State {
    s: String,
    i: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.s.cmp(&self.s)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut sc = Scanner::new();
    let x: Vec<char> = sc.read::<String>().chars().collect();
    let k: usize = sc.read();
    let n = x.len();
    let mut set = BTreeSet::new();
    let mut q = BinaryHeap::new();
    for i in 0..n {
        let mut t = String::new();
        t.push(x[i]);
        q.push(State { s: t, i: i });
    }
    while let Some(State { s, i }) = q.pop() {
        if set.len() >= k {
            break;
        }
        let mut t = s.clone();
        set.insert(s);
        let l = t.len();
        if i + l < n {
            t.push(x[i + l]);
            q.push(State { s: t, i: i });
        }
    }

    let t: Vec<String> = set.iter().map(|s| s.to_owned()).collect();
    println!("{}", t[k - 1]);
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
