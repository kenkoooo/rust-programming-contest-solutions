use std::cmp;
use std::collections::{BTreeMap, BTreeSet};

fn get_primes(n: usize) -> Vec<u64> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..(n + 1) {
        if !is_prime[p] {
            continue;
        }
        let mut cur = 2 * p;
        while cur <= n {
            is_prime[cur] = false;
            cur += p;
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(p, _)| p as u64)
        .collect()
}

fn main() {
    let primes = get_primes(100000);
    let prime2 = primes
        .iter()
        .map(|&p| (p * p, p))
        .collect::<BTreeMap<u64, u64>>();
    let mut sc = Scanner::new();
    let n = sc.usize_read();

    let mut count = BTreeMap::new();

    for _ in 0..n {
        let mut s: u64 = sc.read();
        let mut s1 = 1;
        let mut s2 = 1;
        for &prime in &primes {
            let p2 = prime * prime;
            let p3 = p2 * prime;
            if p3 > s {
                break;
            }
            while s % p3 == 0 {
                s /= p3;
            }
            if s % p2 == 0 {
                s /= p2;
                s2 *= prime;
            } else if s % prime == 0 {
                s /= prime;
                s1 *= prime;
            }
        }

        match prime2.get(&s) {
            Some(&p) => s2 *= p,
            None => s1 *= s,
        }

        let &cur = count.get(&(s1, s2)).unwrap_or(&0);
        count.insert((s1, s2), cur + 1);
    }

    let mut ans = 0;
    let mut set = BTreeSet::new();
    set.insert((1, 1));
    for &(s1, s2) in count.keys() {
        if set.contains(&(s1, s2)) {
            continue;
        }
        let &count1 = count.get(&(s1, s2)).unwrap_or(&0);
        let &count2 = count.get(&(s2, s1)).unwrap_or(&0);
        ans += cmp::max(count1, count2);
        set.insert((s1, s2));
        set.insert((s2, s1));
    }

    if count.contains_key(&(1, 1)) {
        ans += 1;
    }
    println!("{}", ans);
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
