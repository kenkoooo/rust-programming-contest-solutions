use std::io;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::cmp;

const MAX_S: usize = 10000000000;
const MAX_Q: usize = 100000;

fn main() {
    let primes = {
        let mut is_prime = vec![true; 1000000];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            let mut c = i * 2;
            while c < is_prime.len() {
                is_prime[c] = false;
                c += i;
            }
        }
        let mut primes = Vec::new();
        for i in 0..is_prime.len() {
            if is_prime[i] {
                primes.push(i);
            }
        }
        primes
    };

    let p2_map = {
        let mut map = BTreeMap::new();
        for p in &primes {
            let p = *p;
            map.insert(p * p, p);
        }
        map
    };

    let n = read_values::<usize>()[0];

    let mut count_map = BTreeMap::new();
    let mut pair_map = BTreeMap::new();

    let mut one = 0;

    for _ in 0..n {
        let mut s = read_values::<usize>()[0];
        let mut v = Vec::new();

        for p in primes.iter() {
            let p = *p;
            let p3 = p * p * p;
            if p3 > s {
                break;
            }
            while s % p3 == 0 {
                s /= p3;
            }

            if s % (p * p) == 0 {
                v.push((p, 2));
                s /= (p * p);
            } else if s % p == 0 {
                v.push((p, 1));
                s /= p;
            }
        }
        if s != 1 {
            match p2_map.get(&s) {
                Some(p) => v.push((*p, 2)),
                _ => v.push((s, 1)),
            }
        }

        let mut norm = 1;
        let mut pair = 1;
        for t in &v {
            let (p, count) = *t;
            if count == 1 {
                norm *= p;
                if p >= MAX_Q || pair >= MAX_S || pair * p >= MAX_S {
                    pair = 0;
                }
                pair *= (p * p);
            } else {
                norm *= p * p;
                if pair >= MAX_S || pair * p >= MAX_S {
                    pair = 0;
                }
                pair *= p;
            }
        }

        if pair != 0 {
            pair_map.insert(pair, norm);
            pair_map.insert(norm, pair);
        }

        if norm == 1 {
            one = 1;
        } else if count_map.contains_key(&norm) {
            let c = *count_map.get(&norm).unwrap();
            count_map.insert(norm, c + 1);
        } else {
            count_map.insert(norm, 1);
        }
    }

    let mut ans = one;
    let mut viewed_set = BTreeSet::new();
    for key in count_map.keys() {
        if viewed_set.contains(key) {
            continue;
        }

        match pair_map.get(key) {
            Some(pair) => {
                let c1 = *count_map.get(key).unwrap();
                let c2 = match count_map.get(pair) {
                    Some(c2) => *c2,
                    _ => 0,
                };
                viewed_set.insert(*key);
                viewed_set.insert(*pair);
                ans += cmp::max(c1, c2);
            }
            _ => {
                ans += *count_map.get(key).unwrap();
                viewed_set.insert(*key);
            }
        }
    }

    println!("{}", ans);
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

