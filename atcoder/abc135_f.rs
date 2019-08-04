use std::cmp;
use std::collections::BTreeSet;

const BASE: u64 = 1e9 as u64 + 7;
const MOD: u64 = 1e9 as u64 + 9;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };

    let s: Vec<u8> = sc.chars().into_iter().map(|c| c as u8).collect();
    let t: Vec<u8> = sc.chars().into_iter().map(|c| c as u8).collect();
    let s_len = s.len();
    let m = t.len();
    let n = (m + s_len) * 2;
    let s = {
        let mut result = vec![0; n];
        let size = s.len();
        for i in 0..n {
            result[i] = s[i % size];
        }
        result
    };

    let s_hash = RollingHash::new(&s, BASE);
    let t_hash = RollingHash::new(&t, BASE);
    let mut prefix_set = BTreeSet::new();
    for i in 0..s_len {
        if s_hash.get_hash(i, i + m) == t_hash.get_hash(0, m) {
            prefix_set.insert(i);
        }
    }

    let mut length = 0;
    let mut used = BTreeSet::new();
    for &prefix in prefix_set.iter() {
        if used.contains(&prefix) {
            continue;
        }
        let mut current = BTreeSet::new();
        let mut cur = prefix;
        current.insert(cur);
        loop {
            if prefix_set.contains(&((cur + m) % s_len)) {
                cur = (cur + m) % s_len;
                if current.contains(&cur) {
                    println!("-1");
                    return;
                }
                current.insert(cur);
            } else {
                break;
            }
        }
        let mut cur = prefix;
        loop {
            if prefix_set.contains(&((cur + s_len - m) % s_len)) {
                cur = (cur + s_len - m) % s_len;
                if current.contains(&cur) {
                    println!("-1");
                    return;
                }
                current.insert(cur);
            } else {
                break;
            }
        }
        length = cmp::max(length, current.len());
        used.extend(current);
    }
    println!("{}", length);
}

pub struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

impl RollingHash {
    pub fn new(s: &[u8], base: u64) -> RollingHash {
        let n = s.len();
        let mut hash: Vec<u64> = vec![0; n + 1];
        let mut pow: Vec<u64> = vec![0; n + 1];
        pow[0] = 1;
        for i in 0..n {
            pow[i + 1] = (pow[i] * base) % MOD;
            hash[i + 1] = (hash[i] * base + (s[i] as u64)) % MOD;
        }
        RollingHash {
            hash: hash,
            pow: pow,
        }
    }

    /// Get hash of [l, r)
    pub fn get_hash(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + MOD - (self.hash[l] * self.pow[r - l]) % MOD) % MOD
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
