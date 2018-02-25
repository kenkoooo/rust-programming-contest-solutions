use std::cmp;
use std::cmp::Ordering;

const START: usize = 0;
const INSERT: usize = 1;
const END: usize = 2;

fn main() {
    let mut sc = Scanner::new();
    let s = sc.read::<String>().bytes().collect::<Vec<_>>();
    let n = s.len();
    let sa = SuffixArray::new(&s);

    let q = sc.read::<usize>();
    let mut query = Vec::new();
    let mut queue = Vec::new();

    for i in 0..q {
        let l = sc.read::<usize>();
        let mut r = sc.read::<usize>();
        let m = sc.read::<String>().bytes().collect::<Vec<_>>();
        r -= m.len() - 1;
        if l > r {
            continue;
        }

        if !sa.contains(&m) {
            query.push((0, 0));
            continue;
        }

        let low = sa.lower_bound(&m);
        let up = sa.upper_bound(&m);

        query.push((low, up));
        queue.push((l, START, i));
        queue.push((r, END, i));
    }

    for i in 0..(n + 1) {
        queue.push((sa.array[i], INSERT, i));
    }

    queue.sort();

    let mut ans = vec![0; q];
    let mut bit = FenwickTree::new(n + 1);
    for t in &queue {
        let (_, kind, pos) = *t;
        if kind == 1 {
            bit.add(pos, 1);
        } else {
            let (low, up) = query[pos];
            let sum = bit.sum(low, up) as i64;
            if kind == START {
                ans[pos] -= sum;
            } else {
                ans[pos] += sum;
            }
        }
    }

    for a in &ans {
        println!("{}", a);
    }
}

#[allow(dead_code)]
pub struct SuffixArray {
    n: usize,
    s: Vec<u8>,
    rank: Vec<i32>,
    array: Vec<usize>,
}

fn compare_node(i: usize, j: usize, k: usize, rank: &Vec<i32>) -> Ordering {
    if rank[i] != rank[j] {
        rank[i].cmp(&rank[j])
    } else {
        let ri = if i + k <= rank.len() { rank[i + k] } else { -1 };
        let rj = if j + k <= rank.len() { rank[j + k] } else { -1 };
        ri.cmp(&rj)
    }
}

impl SuffixArray {
    pub fn new(s: &Vec<u8>) -> SuffixArray {
        let n = s.len();
        let mut rank = vec![0; n + 1];
        let mut array = vec![0; n + 1];

        for i in 0..(n + 1) {
            array[i] = i;
            rank[i] = if i < n { s[i] as i32 } else { -1 };
        }

        let mut tmp = vec![0; n + 1];
        let mut k = 1;
        while k <= n {
            array.sort_by(|a, b| compare_node(*a, *b, k, &rank));

            tmp[array[0]] = 0;
            for i in 1..(n + 1) {
                tmp[array[i]] = tmp[array[i - 1]] + if compare_node(array[i - 1], array[i], k, &rank) == Ordering::Less { 1 } else { 0 }
            }
            for i in 0..(n + 1) {
                rank[i] = tmp[i];
            }
            k *= 2;
        }

        SuffixArray { n: n, rank: rank, array: array, s: s.clone() }
    }

    pub fn contains(&self, t: &Vec<u8>) -> bool {
        let b = self.lower_bound(t);
        if b >= self.array.len() {
            false
        } else {
            let start = self.array[b];
            let end = cmp::min(t.len() + start, self.s.len());
            let sub = &self.s[start..end];
            sub.cmp(t) == Ordering::Equal
        }
    }

    fn binary_search<F>(&self, t: &Vec<u8>, f: F) -> usize where F: Fn(&[u8], &Vec<u8>) -> bool {
        let (mut a, mut b) = (-1, self.n as i32 + 1);
        while b - a > 1 {
            let c = (a + b) / 2;
            let start = self.array[c as usize];
            let end = cmp::min(start + t.len(), self.s.len());
            let sub = &self.s[start..end];
            if f(sub, t) {
                a = c;
            } else {
                b = c;
            }
        }
        b as usize
    }

    pub fn lower_bound(&self, t: &Vec<u8>) -> usize {
        let check_function = |sub: &[u8], s: &Vec<u8>| sub.cmp(s) == Ordering::Less;
        self.binary_search(t, check_function)
    }

    pub fn upper_bound(&self, t: &Vec<u8>) -> usize {
        let check_function = |sub: &[u8], s: &Vec<u8>| sub.cmp(s) != Ordering::Greater;
        self.binary_search(t, check_function)
    }
}

pub struct FenwickTree {
    n: usize,
    data: Vec<usize>,
}

impl FenwickTree {
    fn new(size: usize) -> FenwickTree {
        FenwickTree { n: size + 1, data: vec![0; size + 1] }
    }

    fn add(&mut self, k: usize, value: usize) {
        let mut x = k;
        while x < self.n {
            self.data[x] += value;
            x |= x + 1;
        }
    }

    /// returns sum of [l, r)
    fn sum(&self, l: usize, r: usize) -> usize {
        return self.sum_one(r) - self.sum_one(l);
    }

    /// returns sum of [0, k)
    fn sum_one(&self, k: usize) -> usize {
        if k >= self.n {
            panic!("");
        }

        let mut result = 0;
        let mut x = k as i32 - 1;
        while x >= 0 {
            result += self.data[x as usize];
            x = (x & (x + 1)) - 1;
        }

        return result;
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
        Scanner { ptr: 0, length: 0, buf: vec![0; 1024], small_cache: vec![0; 1024] }
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

    fn is_space(b: u8) -> bool { b == b'\n' || b == b'\r' || b == b'\t' || b == b' ' }

    fn read<T>(&mut self) -> T where T: std::str::FromStr, T::Err: std::fmt::Debug, {
        let mut b = self.byte();
        while Scanner::is_space(b) {
            b = self.byte();
        }

        for pos in 0..self.small_cache.len() {
            self.small_cache[pos] = b;
            b = self.byte();
            if Scanner::is_space(b) {
                return String::from_utf8_lossy(&self.small_cache[0..(pos + 1)]).parse().unwrap();
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

