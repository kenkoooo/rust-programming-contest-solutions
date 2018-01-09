use std::io;
use std::cmp;
use std::cmp::Ordering;
use std::usize::MAX;

fn main() {
    let s = read_line().trim().to_owned().bytes().collect::<Vec<_>>();
    let n = s.len();

    let reverse_sa = {
        let mut reverse_s = s.clone();
        reverse_s.reverse();
        SuffixArray::new(reverse_s)
    };
    let sa = SuffixArray::new(s);

    let m = read_values::<usize>()[0];

    let mut rmq = SegmentTree::new(n + 1, MAX, |a, b| cmp::min(a, b));
    let mut reverse_rmq = SegmentTree::new(n + 1, MAX, |a, b| cmp::min(a, b));
    for i in 0..(n + 1) {
        rmq.update(i, sa.array[i]);
        reverse_rmq.update(i, reverse_sa.array[i]);
    }

    for _ in 0..m {
        let v = read_values::<String>();
        let head = v[0].to_owned().bytes().collect::<Vec<_>>();

        if !sa.contains(&head) {
            println!("0");
            continue;
        }

        let left = {
            let head_lower = sa.lower_bound(&head);
            let head_upper = sa.upper_bound(&head);
            rmq.query(head_lower, head_upper)
        };

        let mut tail = v[1].to_owned().bytes().collect::<Vec<_>>();
        tail.reverse();

        if !reverse_sa.contains(&tail) {
            println!("0");
            continue;
        }

        let right = {
            let tail_lower = reverse_sa.lower_bound(&tail);
            let tail_upper = reverse_sa.upper_bound(&tail);
            n - reverse_rmq.query(tail_lower, tail_upper)
        };

        if left + head.len() <= right && left + tail.len() <= right {
            println!("{}", right - left);
        } else {
            println!("0");
        }
    }
}

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
    pub fn new(s: Vec<u8>) -> SuffixArray {
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

        SuffixArray { n: n, rank: rank, array: array, s: s }
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

pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Clone, F> SegmentTree<T, F> where F: Fn(T, T) -> T {
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initial_value.clone(); m * 2],
            n: m,
            f: f,
            initial_value: initial_value.clone(),
        }
    }

    pub fn update(&mut self, mut k: usize, value: T) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1].clone(), self.seg[k * 2 + 2].clone());
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> T {
        assert!(a < b);
        return self.query_range(a, b, 0, 0, self.n);
    }

    pub fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.initial_value.clone();
        }
        if a <= l && r <= b {
            return self.seg[k].clone();
        }
        let x = self.query_range(a, b, k * 2 + 1, l, (l + r) >> 1);
        let y = self.query_range(a, b, k * 2 + 2, (l + r) >> 1, r);
        (self.f)(x, y)
    }
}


fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T> where T: std::str::FromStr, T::Err: std::fmt::Debug {
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}