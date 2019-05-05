extern crate num_bigint;
use num_bigint::BigInt;

fn main() {
    let s = std::io::stdin();
    let mut sc = Scanner { reader: s.lock() };
    let n: usize = sc.read();
    if n == 1 {
        println!("1");
        println!("+ 1");
        return;
    }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..(n + 1) {
        if is_prime[p] {
            let mut cur = p * 2;
            while cur <= n {
                is_prime[cur] = false;
                cur += p;
            }
        }
    }

    let primes = is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(p, _)| {
            let mut pow = 1;
            while pow * p <= n {
                pow *= p;
            }
            pow
        })
        .collect::<Vec<_>>();
    let add_counts = primes
        .iter()
        .map(|&prime| {
            // lcs(1, 2, ..., n) / prime (mod prime)
            let mut lcs = 1;
            for &p in primes.iter().filter(|&&p| p != prime) {
                lcs *= p;
                lcs %= prime;
            }

            let mut count = 0;
            while (count * lcs) % prime != 1 {
                count += 1;
            }
            (prime, count)
        })
        .collect::<Vec<(usize, usize)>>();

    let mut add = vec![];
    let mut sub = vec![];
    for &(prime, add_count) in add_counts.iter() {
        if add_count * 2 > prime {
            let sub_count = prime - add_count;
            sub.extend(vec![prime; sub_count]);
        } else {
            add.extend(vec![prime; add_count]);
        }
    }

    let mut lcs = BigInt::from(1);
    for &(prime, _) in add_counts.iter() {
        lcs *= prime;
    }

    let mut num = BigInt::from(0);
    for &add in add.iter() {
        num += lcs.clone() / add;
    }
    for &sub in sub.iter() {
        num -= lcs.clone() / sub;
    }

    while num < BigInt::from(0) {
        num += lcs.clone();
        add.push(1);
    }
    while num > lcs {
        num -= lcs.clone();
        sub.push(1);
    }

    println!("{}", add.len() + sub.len());
    for &a in add.iter() {
        println!("+ {}", a);
    }
    for &a in sub.iter() {
        println!("- {}", a);
    }
}

pub struct Scanner<R> {
    reader: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .reader
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
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
