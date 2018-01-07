use std::io;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::cmp;

const MAX: usize = 10000000000;

fn main() {
    let primes = get_primes(100000);
    let p2 = primes.iter().map(|p| p * p).collect::<BTreeSet<_>>();
    let mut pair_map = BTreeMap::new();
    let mut count_map = BTreeMap::new();

    let n = read_values::<usize>()[0];
    for _ in 0..n {
        let mut s = read_values::<usize>()[0];
        let mut normal = 1;
        let mut pair = 1;
        for prime in primes.iter() {
            let prime = *prime;
            let p3 = prime * prime * prime;
            if p3 > MAX {
                break;
            }
            while s % p3 == 0 {
                s /= p3;
            }
            if s % (prime * prime) == 0 {
                s /= (prime * prime);
                pair *= prime;
                normal *= (prime * prime);
            } else if s % prime == 0 {
                s /= prime;
                normal *= prime;
                pair *= (prime * prime);
            }
            if pair > MAX {
                pair = 0;
            }
        }
        if s != 1 {
            if p2.contains(&s) {
                let p = (s as f64).sqrt() as usize;
                normal *= s;
                pair *= p;
            } else {
                normal *= s;
                pair *= s;
                if pair > MAX || (pair > 100000 && s > 100000) {
                    pair = 0;
                }
                pair *= s;
            }
        }

        if count_map.contains_key(&normal) {
            let cur = *count_map.get(&normal).unwrap();
            count_map.insert(normal, cur + 1);
        } else {
            count_map.insert(normal, 1);
        }
        pair_map.insert(normal, pair);
    }

    let mut ans = 0;
    let mut checked = BTreeSet::new();
    match count_map.get(&1) {
        Some(_) => {
            ans += 1;
            checked.insert(1);
        }
        _ => {}
    }
    for item in pair_map.iter() {
        let (normal, pair) = item;
        let normal_count = *count_map.get(normal).unwrap();
        let pair_count = match count_map.get(pair) {
            Some(count) => *count,
            _ => 0,
        };
        if checked.contains(normal) {
            continue;
        }
        ans += cmp::max(normal_count, pair_count);
        checked.insert(*normal);
        checked.insert(*pair);
    }
    println!("{}", ans);
}

fn get_primes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for p in 2..(n + 1) {
        if is_prime[p] {
            let mut c = 2;
            while c * p <= n {
                is_prime[c * p] = false;
                c += 1;
            }
        }
    }

    let mut primes = Vec::new();
    for i in 2..(n + 1) {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
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

