use std::cmp;
use std::cmp::Ordering;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    let s = sc
        .chars()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let t = sc
        .chars()
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let n = s.len();
    let mut index = vec![vec![]; 26];
    for i in 0..s.len() {
        index[s[i]].push(i);
    }

    let mut lot = 0;
    let mut pos = 0;
    for t in t.into_iter() {
        let i = index[t]
            .binary_search_by_key(&(pos * 2), |x| x * 2 + 1)
            .err()
            .unwrap();
        if i < index[t].len() {
            //            eprintln!("{} => {}", pos, index[t][i] + 1);
            pos = index[t][i] + 1;
            continue;
        }
        lot += 1;
        pos = 0;
        let i = index[t]
            .binary_search_by_key(&(pos * 2), |i| i * 2 + 1)
            .err()
            .unwrap();
        if i < index[t].len() {
            //            eprintln!("+{} => {}", pos, index[t][i] + 1);
            pos = index[t][i] + 1;
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", lot * n + pos);
}

pub struct SuffixArray {
    n: usize,
    s: Vec<u8>,
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
                tmp[array[i]] = tmp[array[i - 1]]
                    + if compare_node(array[i - 1], array[i], k, &rank) == Ordering::Less {
                        1
                    } else {
                        0
                    }
            }
            for i in 0..(n + 1) {
                rank[i] = tmp[i];
            }
            k *= 2;
        }

        SuffixArray {
            n: n,
            array: array,
            s: s.clone(),
        }
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

    fn binary_search<F>(&self, t: &Vec<u8>, f: F) -> usize
    where
        F: Fn(&[u8], &Vec<u8>) -> bool,
    {
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

pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n')
            .take_while(|&b| b != b' ' && b != b'\n')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
