use std::io;
use std::cmp;
use std::cmp::Ordering;
use std::i64::MAX;

fn main() {
    let s = read_line().trim().to_owned().bytes().collect::<Vec<_>>();
    let n = s.len();
    let reverse_s = {
        let mut r = s.clone();
        r.reverse();
        r
    };

    let sa = SuffixArray::new(s);
    let reverse_sa = SuffixArray::new(reverse_s);

    let mut rmq = RangeMinimumQuery::new(n + 1);
    let mut reverse_rmq = RangeMinimumQuery::new(n + 1);
    for i in 0..(n + 1) {
        rmq.update(i, sa.array[i] as i64);
        reverse_rmq.update(i, reverse_sa.array[i] as i64);
    }

    let m = read_values::<usize>()[0];
    for _ in 0..m {
        let input = read_values::<String>();
        let x = input[0].to_owned().bytes().collect();
        let y = {
            let mut y = input[1].to_owned().bytes().collect::<Vec<_>>();
            y.reverse();
            y
        };

        if !sa.contains(&x) {
            println!("0");
            continue;
        }
        let low = sa.lower_bound(&x);
        let up = sa.upper_bound(&x);

        if !reverse_sa.contains(&y) {
            println!("0");
            continue;
        }
        let reverse_low = reverse_sa.lower_bound(&y);
        let reverse_up = reverse_sa.upper_bound(&y);

        if low >= up || reverse_low >= reverse_up {
            println!("0");
        }

        let s = rmq.query(low, up) as usize;
        let t = n - reverse_rmq.query(reverse_low, reverse_up) as usize;
        if s + x.len() <= t && s <= t - y.len() {
            println!("{}", t - s);
        } else {
            println!("0");
        }
    }
}

pub struct RangeMinimumQuery {
    seg: Vec<i64>,
    n: usize,
}

impl RangeMinimumQuery {
    pub fn new(size: usize) -> RangeMinimumQuery {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        RangeMinimumQuery {
            seg: vec![MAX; m * 2],
            n: m,
        }
    }

    pub fn update(&mut self, mut k: usize, value: i64) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = cmp::min(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> i64 {
        return self.query_range(a, b, 0, 0, self.n);
    }

    pub fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return MAX;
        }
        if a <= l && r <= b {
            return self.seg[k];
        }
        let x = self.query_range(a, b, k * 2 + 1, l, (l + r) >> 1);
        let y = self.query_range(a, b, k * 2 + 2, (l + r) >> 1, r);
        cmp::min(x, y)
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